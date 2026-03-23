//! Audit event schema written to audit.jsonl.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditActor {
    Executor,
    Plugin(String),
    Llm,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub ts: DateTime<Utc>,
    pub run_id: String,
    pub step_id: String,
    pub actor: AuditActor,
    pub tool: String,
    pub action: String,
    pub args_hash: String,
    pub result: AuditResult,
    pub duration_ms: u64,
    pub risk_tier: String,
    pub is_cached: bool,
    pub is_self_heal: bool,
    pub redactions: Vec<String>,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum AuditResult {
    Success,
    Failure { reason: String },
    Blocked { reason: String },
}
