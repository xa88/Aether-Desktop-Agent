//! Plan Validator: ensuring LLM-generated plans follow schema and safety rules.

use crate::plan::Plan;
use jsonschema::JSONSchema;
use std::sync::Arc;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("JSON Schema mismatch: {0}")]
    SchemaMismatch(String),
    #[error("Invalid YAML: {0}")]
    InvalidYaml(String),
    #[error("Safety violation: {0}")]
    SafetyViolation(String),
}

#[derive(Clone)]
pub struct PlanValidator {
    schema: Arc<JSONSchema>,
}

impl PlanValidator {
    pub fn new() -> anyhow::Result<Self> {
        let schema_json: Value = serde_json::from_str(include_str!("../../../schemas/plan.schema.json"))?;
        let schema = JSONSchema::compile(&schema_json)
            .map_err(|e| anyhow::anyhow!("Failed to compile schema: {}", e))?;
        Ok(Self { schema: Arc::new(schema) })
    }

    /// Validate raw YAML string from LLM.
    pub fn validate_raw(&self, raw_yaml: &str) -> Result<Plan, ValidationError> {
        let val: serde_json::Value = serde_yaml::from_str(raw_yaml)
            .map_err(|e| ValidationError::InvalidYaml(e.to_string()))?;

        if let Err(errors) = self.schema.validate(&val) {
            let msg = errors.map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
            return Err(ValidationError::SchemaMismatch(msg));
        }

        let plan: Plan = serde_json::from_value(val)
            .map_err(|e| ValidationError::InvalidYaml(e.to_string()))?;

        self.check_safety(&plan)?;

        Ok(plan)
    }

    fn check_safety(&self, plan: &Plan) -> Result<(), ValidationError> {
        // Example: block any plan that tries to touch certain keys or paths
        for step in &plan.steps {
            if step.step_type == "shell_run" {
                if let Some(cmd) = step.args["cmd"].as_str() {
                    if cmd.contains("rm -rf /") || cmd.contains("format") {
                        return Err(ValidationError::SafetyViolation(format!(
                            "Dangerous command detected in step {}: {}", step.id, cmd
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}
