//! Enterprise LLM Gateway Router.
use crate::LlmProvider;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TaskClassification {
    Planning,
    Execution,
    Summarization,
    Fallback
}

pub struct LlmRouter {
    routes: HashMap<TaskClassification, Arc<dyn LlmProvider>>,
    default_provider: Arc<dyn LlmProvider>,
}

impl LlmRouter {
    pub fn new(default_provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            routes: HashMap::new(),
            default_provider,
        }
    }

    pub fn register_route(&mut self, classification: TaskClassification, provider: Arc<dyn LlmProvider>) {
        self.routes.insert(classification, provider);
    }

    pub fn route(&self, classification: &TaskClassification) -> Arc<dyn LlmProvider> {
        self.routes.get(classification).cloned().unwrap_or_else(|| self.default_provider.clone())
    }
}
