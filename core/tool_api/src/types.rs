//! Core types shared across all ADA tools.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Risk tier for a tool call or plan step.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RiskTier {
    #[default]
    T0,
    T1,
    T2,
    T3,
}

/// A request to invoke a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub id: String,
    pub run_id: String,
    pub step_id: String,
    pub tool: String,
    pub action: String,
    pub args: serde_json::Value,
    pub risk_tier: RiskTier,
    pub timeout_ms: u64,
    pub cwd: Option<String>,
    pub env: HashMap<String, String>,
}

impl ToolRequest {
    pub fn new(tool: &str, action: &str, args: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            run_id: String::new(),
            step_id: String::new(),
            tool: tool.to_string(),
            action: action.to_string(),
            args,
            risk_tier: RiskTier::T0,
            timeout_ms: 30_000,
            cwd: None,
            env: HashMap::new(),
        }
    }
}

/// A response from a tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    pub id: String,
    pub request_id: String,
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<ToolError>,
    pub duration_ms: u64,
    pub truncated: bool,
}

impl ToolResponse {
    pub fn ok(request_id: &str, output: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: request_id.to_string(),
            success: true,
            output,
            error: None,
            duration_ms,
            truncated: false,
        }
    }

    pub fn err(request_id: &str, error: ToolError, duration_ms: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: request_id.to_string(),
            success: false,
            output: serde_json::Value::Null,
            error: Some(error),
            duration_ms,
            truncated: false,
        }
    }
}

/// Structured error from a tool call.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{code}: {message}")]
pub struct ToolError {
    pub code: ToolErrorCode,
    pub message: String,
    pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ToolErrorCode {
    Timeout,
    PermissionDenied,
    PathEscape,
    NotFound,
    InvalidArgs,
    ExecFailed,
    RiskTierBlocked,
    PolicyViolation,
    Unknown,
}

impl std::fmt::Display for ToolErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Metadata attached to an audit event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMeta {
    pub ts: DateTime<Utc>,
    pub run_id: String,
    pub step_id: String,
    pub actor: String,
    pub tool: String,
    pub action: String,
    pub args_hash: String,
    pub risk_tier: RiskTier,
}
