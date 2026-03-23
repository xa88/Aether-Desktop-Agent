//! GitHandler: wraps git CLI for status/diff/stash/commit/checkout.

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::ToolHandler;
use async_trait::async_trait;
use tokio::process::Command;
use tracing::info;

pub struct GitHandler;

impl GitHandler {
    pub fn new() -> Self { Self }

    async fn git(&self, args: &[&str], cwd: &str) -> Result<String, ToolError> {
        let output = Command::new("git")
            .args(args)
            .current_dir(cwd)
            .output()
            .await
            .map_err(|e| ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("git spawn failed: {e}"),
                detail: None,
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(ToolError {
                code: ToolErrorCode::ExecFailed,
                message: stderr,
                detail: Some(serde_json::json!({ "exit_code": output.status.code() })),
            })
        }
    }
}

impl Default for GitHandler {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl ToolHandler for GitHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        let cwd = req.cwd.as_deref().unwrap_or(".");
        match req.action.as_str() {
            "status" => {
                let out = self.git(&["status", "--porcelain"], cwd).await?;
                Ok(serde_json::json!({ "output": out }))
            }
            "diff" => {
                let out = self.git(&["diff"], cwd).await?;
                Ok(serde_json::json!({ "diff": out }))
            }
            "stash" => {
                let msg = req.args["message"].as_str().unwrap_or("ada-stash");
                let out = self.git(&["stash", "push", "-m", msg], cwd).await?;
                Ok(serde_json::json!({ "output": out }))
            }
            "commit" => {
                let msg = req.args["message"].as_str().unwrap_or("ADA auto-commit");
                // Stage all changes first
                self.git(&["add", "-A"], cwd).await?;
                let out = self.git(&["commit", "-m", msg], cwd).await?;
                info!("git/commit in {cwd}: {msg}");
                Ok(serde_json::json!({ "output": out }))
            }
            "checkout" => {
                let branch = req.args["branch"].as_str().unwrap_or("main");
                let out = self.git(&["checkout", branch], cwd).await?;
                Ok(serde_json::json!({ "output": out }))
            }
            "log" => {
                let n = req.args["n"].as_u64().unwrap_or(10);
                let out = self.git(&["log", "--oneline", &format!("-{n}")], cwd).await?;
                Ok(serde_json::json!({ "log": out }))
            }
            _ => Err(ToolError {
                code: ToolErrorCode::InvalidArgs,
                message: format!("Unknown git action: {}", req.action),
                detail: None,
            }),
        }
    }
}
