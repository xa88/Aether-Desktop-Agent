//! Mock LLM provider for local development and testing.

use crate::{ChatMessage, LlmProvider};
use async_trait::async_trait;

pub struct MockProvider;

#[async_trait]
impl LlmProvider for MockProvider {
    async fn generate_plan(&self, _prompt: &str) -> anyhow::Result<String> {
        Ok("schema_version: '1.0'\nmeta:\n  task_id: 'mock'\n  title: 'Mock Plan'\n  created_at: '2026-03-03T12:00:00Z'\n  execution_mode: 'host'\n  budgets:\n    max_wall_time_s: 60\n    max_steps: 10\nsteps: []".to_string())
    }

    async fn chat(&self, _messages: Vec<ChatMessage>) -> anyhow::Result<String> {
        Ok("Mock response".to_string())
    }
}
