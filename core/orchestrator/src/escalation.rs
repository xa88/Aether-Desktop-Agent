//! Escalation engine for handling complex failures and escalation strategies.

use std::sync::Arc;
use ada_tool_api::router::ToolRouter;
use ada_llm::PlanContext;
use ada_executor::PlanValidator;
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, warn};

/// Trait that components can implement to support escalation.
#[async_trait]
pub trait Escalatable {
    async fn escalate(&self, error: &anyhow::Error) -> Result<()>;
}

/// The EscalationEngine struct.
pub struct EscalationEngine {
    pub router: Arc<ToolRouter>,
    pub validator: PlanValidator,
    pub failure_threshold: usize,
}

impl EscalationEngine {
    pub fn new(router: Arc<ToolRouter>, validator: PlanValidator, failure_threshold: usize) -> Self {
        Self { router, validator, failure_threshold }
    }

    /// Perform escalation based on the provided error and attempt count.
    pub async fn maybe_escalate(&self, attempt: usize, _error: &anyhow::Error, goal: &str, _context: &PlanContext) -> Result<bool> {
        if attempt >= self.failure_threshold {
            error!("Escalation threshold reached. Escalating task: {}", goal);
            // Here we could call a higher-level planner or cloud service.
            // For now we just log and return true indicating escalation.
            Ok(true)
        } else {
            warn!("Escalation not yet triggered (attempt {}/{})", attempt, self.failure_threshold);
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ada_tool_api::router::ToolRouter;
    use ada_executor::PlanValidator;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_maybe_escalate() {
        let router = ToolRouter::new();
        let validator = PlanValidator::new().unwrap();
        let engine = EscalationEngine::new(Arc::new(router), validator, 3);

        let context = PlanContext::default();
        let error = anyhow::anyhow!("test error");

        // Attempt 1: Should not escalate
        let result = engine.maybe_escalate(1, &error, "test goal", &context).await.unwrap();
        assert!(!result);

        // Attempt 3: Should escalate (threshold is 3)
        let result = engine.maybe_escalate(3, &error, "test goal", &context).await.unwrap();
        assert!(result);
    }
}
