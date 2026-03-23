//! ada-llm: OpenAI-compatible LLM Gateway with streaming support.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate_plan(&self, prompt: &str) -> anyhow::Result<String>;
    async fn chat(&self, messages: Vec<ChatMessage>) -> anyhow::Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub mod providers {
    pub mod openai_compat;
    pub mod mock;
}

pub mod template_engine;
pub use template_engine::{IntentTaxonomy, LocalTemplateEngine};

pub mod prompts;
pub use prompts::*;

pub mod routing;
