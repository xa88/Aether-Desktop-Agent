//! ada-adapter-web-playwright: Bridge to execute web automation via Node+Playwright.

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::ToolHandler;
use async_trait::async_trait;
use serde_json::Value;
use tokio::process::Command;
use tracing::{info, warn, error};

pub struct PlaywrightHandler {
    node_script_path: String,
}

impl PlaywrightHandler {
    pub fn new(node_script_path: String) -> Self {
        Self { node_script_path }
    }
}

#[async_trait]
impl ToolHandler for PlaywrightHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        info!("playwright/{}: Invoking Playwright...", req.action);
        
        let payload = serde_json::json!({
            "action": req.action,
            "args": req.args,
            "run_id": req.run_id
        });
        
        let payload_str = payload.to_string();
        
        let output = Command::new("node")
            .arg(&self.node_script_path)
            .arg(&payload_str)
            .output()
            .await
            .map_err(|e| ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("Failed to spawn node: {}", e),
                detail: None,
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if !output.status.success() {
            error!("Playwright runner exit status {}: {}", output.status, stderr);
            return Err(ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("Playwright runner failed: {}", stderr),
                detail: Some(serde_json::json!({ "stdout": stdout })),
            });
        }

        // Try to parse JSON output from stdout
        let last_line = stdout.lines().last().unwrap_or("");
        match serde_json::from_str::<Value>(last_line) {
            Ok(json) => {
                let success = json["success"].as_bool().unwrap_or(false);
                if success {
                    Ok(json["output"].clone())
                } else {
                    let err_msg = json["error"].as_str().unwrap_or("Unknown playwright error");
                    let trace = json["trace"].as_str().unwrap_or("");
                    warn!("Playwright Execution Failed: {}, trace dumped to {}", err_msg, trace);
                    Err(ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: err_msg.to_string(),
                        detail: Some(serde_json::json!({ "trace": trace })),
                    })
                }
            }
            Err(e) => {
                error!("Failed to parse Playwright output: {}\nRaw: {}", e, stdout);
                Err(ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: "Invalid output from Playwright runner".into(),
                    detail: Some(serde_json::json!({ "raw": stdout })),
                })
            }
        }
    }
}
