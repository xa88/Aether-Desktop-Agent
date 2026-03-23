//! OpenAI-compatible LLM provider implementation.

use crate::{ChatMessage, LlmProvider};
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};
use tracing::warn;

pub struct OpenAiCompatProvider {
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
    client: Client,
    max_retries: u32,
}

impl OpenAiCompatProvider {
    pub fn new(base_url: String, api_key: String, model_id: String) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(120)) // 2 min timeout for plan generation
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            base_url,
            api_key,
            model_id,
            client,
            max_retries: 5,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatProvider {
    async fn generate_plan(&self, prompt: &str) -> anyhow::Result<String> {
        let messages = vec![
            ChatMessage {
                role: "system".into(),
                content: "You are an ADA plan generator. Output only a valid plan.yaml. Do NOT wrap the output in markdown code blocks.".into(),
            },
            ChatMessage {
                role: "user".into(),
                content: prompt.into(),
            },
        ];
        self.chat(messages).await
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> anyhow::Result<String> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let body = json!({
            "model": self.model_id,
            "messages": messages,
            "temperature": 0.1,
        });

        let mut attempt = 0;
        let mut backoff_ms = 1000;

        loop {
            attempt += 1;
            let req = self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&body);

            let result = req.send().await;

            match result {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        let res_json: Value = resp.json().await?;
                        let content = res_json["choices"][0]["message"]["content"]
                            .as_str()
                            .ok_or_else(|| anyhow::anyhow!("Invalid response from LLM"))?;
                        return Ok(content.to_string());
                    } else if status.as_u16() == 429 || status.is_server_error() {
                        if attempt >= self.max_retries {
                            let err_text = resp.text().await.unwrap_or_default();
                            return Err(anyhow::anyhow!("LLM request failed after {} retries. Status: {}. Details: {}", attempt, status, err_text));
                        }
                        warn!("LLM provider returned status {}, retrying in {}ms (attempt {}/{})", status, backoff_ms, attempt, self.max_retries);
                    } else {
                        // Client error like 400, 401, 403, 404
                        let err_text = resp.text().await.unwrap_or_default();
                        return Err(anyhow::anyhow!("LLM API client error ({}): {}", status, err_text));
                    }
                }
                Err(e) => {
                    if attempt >= self.max_retries {
                        return Err(anyhow::anyhow!("LLM network error after {} retries: {}", attempt, e));
                    }
                    warn!("LLM network error: {}, retrying in {}ms (attempt {}/{})", e, backoff_ms, attempt, self.max_retries);
                }
            }

            sleep(Duration::from_millis(backoff_ms)).await;
            backoff_ms *= 2; // exponential backoff
        }
    }
}
