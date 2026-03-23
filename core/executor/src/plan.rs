//! plan.yaml data structures — parsed with serde_yaml.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    pub schema_version: String,
    pub meta: PlanMeta,
    #[serde(default)]
    pub artifacts_to_produce: Vec<String>,
    #[serde(default)]
    pub rollback: Option<RollbackConfig>,
    #[serde(default)]
    pub safety: Option<SafetyConfig>,
    pub steps: Vec<PlanStep>,
    #[serde(default)]
    pub checks: Vec<PlanCheck>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanMeta {
    pub task_id: String,
    pub title: String,
    pub created_at: String,
    #[serde(default)]
    pub requested_by: String,
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
    pub execution_mode: ExecutionMode,
    pub budgets: Budgets,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub path: String,
    pub repo_url: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    Sandbox,
    Host,
    Hybrid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Budgets {
    #[serde(default = "defaults::max_wall_time_s")]
    pub max_wall_time_s: u64,
    #[serde(default = "defaults::max_steps")]
    pub max_steps: usize,
    #[serde(default = "defaults::max_cmd_output_kb")]
    pub max_cmd_output_kb: usize,
    #[serde(default = "defaults::max_artifacts_mb")]
    pub max_artifacts_mb: usize,
    #[serde(default = "defaults::max_retries_total")]
    pub max_retries_total: usize,
    #[serde(default = "defaults::max_cloud_rounds")]
    pub max_cloud_rounds: u32,
}

mod defaults {
    pub fn max_wall_time_s() -> u64 { 3600 }
    pub fn max_steps() -> usize { 120 }
    pub fn max_cmd_output_kb() -> usize { 512 }
    pub fn max_artifacts_mb() -> usize { 500 }
    pub fn max_retries_total() -> usize { 20 }
    pub fn max_cloud_rounds() -> u32 { 15 }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackConfig {
    pub strategy: String,
    #[serde(default)]
    pub checkpoint_steps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SafetyConfig {
    #[serde(default)]
    pub risk_tier_default: String,
    #[serde(default)]
    pub blocked_cmd_patterns: Vec<String>,
    #[serde(default)]
    pub allowed_path_roots: Vec<String>,
    #[serde(default = "default_network")]
    pub network_policy: String,
    #[serde(default)]
    pub network_allowlist: Vec<String>,
}

fn default_network() -> String { "allowlist".to_string() }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    #[serde(default)]
    pub risk_tier: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_s: u64,
    #[serde(default)]
    pub retries: u32,
    #[serde(default)]
    pub retry_backoff_ms: u64,
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub args: serde_json::Value,
    pub on_fail: Option<OnFail>,
}

fn default_timeout() -> u64 { 60 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OnFail {
    pub action: OnFailAction,
    pub max_attempts: Option<u32>,
    #[serde(default)]
    pub collect: Vec<String>,
    pub rollback_to: Option<String>,
    pub escalate_hint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OnFailAction {
    Retry,
    Skip,
    Rollback,
    CollectAndAbort,
    EscalatePlan,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanCheck {
    pub id: String,
    #[serde(rename = "type")]
    pub check_type: String,
    #[serde(default)]
    pub args: serde_json::Value,
    #[serde(default)]
    pub severity: Option<String>,
}

/// Load and parse a plan.yaml file.
pub fn load_plan(path: &str) -> anyhow::Result<Plan> {
    let content = std::fs::read_to_string(path)?;
    let plan: Plan = serde_yaml::from_str(&content)?;
    Ok(plan)
}
