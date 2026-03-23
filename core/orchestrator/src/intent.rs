//! Intent Taxonomy & Local Template Engine

use serde::{Serialize, Deserialize};
use ada_executor::Plan;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    Dev,       // Development tasks (coding, debugging)
    Query,     // Information retrieval
    SysOp,     // System operations (cleanup, config)
    Cancel,    // Abort current execution
    Pause,     // Pause current execution
    Unknown,
}

pub struct IntentMatcher;

impl IntentMatcher {
    pub fn match_text(text: &str) -> Intent {
        let text = text.to_lowercase();
        if text.contains("stop") || text.contains("cancel") || text.contains("abort") {
            Intent::Cancel
        } else if text.contains("pause") || text.contains("wait") {
            Intent::Pause
        } else if text.contains("clean") || text.contains("delete") || text.contains("remove") {
            Intent::SysOp
        } else if text.contains("what") || text.contains("how") || text.contains("search") {
            Intent::Query
        } else {
            Intent::Dev
        }
    }
}

pub struct LocalPlanGenerator;

impl LocalPlanGenerator {
    pub fn generate_for_intent(intent: Intent, goal: &str) -> Option<Plan> {
        match intent {
            Intent::Cancel => {
                // Return a plan that essentially stops everything or triggers a signal
                None // Placeholder for now
            }
            Intent::SysOp if goal.contains("clean") => {
                let yaml = format!(r#"
version: "1.0"
meta:
  title: "Local Cleanup"
  budgets: {{ time_s: 60 }}
steps:
  - id: "clean_target"
    step_type: "shell_run"
    args:
        command: "cargo clean"
"#);
                serde_yaml::from_str(&yaml).ok()
            }
            _ => None,
        }
    }
}
