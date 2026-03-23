//! ShellHandler: cross-platform shell execution with timeout + output limit.

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::ToolHandler;
use async_trait::async_trait;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};


pub struct ShellHandler {
    output_limit: usize,
}

impl ShellHandler {
    pub fn new(output_limit_kb: usize) -> Self {
        Self { output_limit: output_limit_kb * 1024 }
    }
}

impl Default for ShellHandler {
    fn default() -> Self { Self::new(512) }
}

#[cfg(windows)]
fn build_cmd(cmd: &str) -> Command {
    let mut c = Command::new("cmd");
    c.args(["/C", cmd]);
    c
}

#[cfg(not(windows))]
fn build_cmd(cmd: &str) -> Command {
    let mut c = Command::new("sh");
    c.args(["-c", cmd]);
    c
}

#[async_trait]
impl ToolHandler for ShellHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        let cmd = req.args["cmd"].as_str().ok_or_else(|| ToolError {
            code: ToolErrorCode::InvalidArgs,
            message: "Missing 'cmd' arg".to_string(),
            detail: None,
        })?;

        let cwd = req.cwd.as_deref().unwrap_or(".");
        let timeout_ms = if req.timeout_ms > 0 { req.timeout_ms } else { 30_000 };

        info!("shell/run: {} (timeout={}ms, cwd={})", cmd, timeout_ms, cwd);

        let mut child = build_cmd(cmd);
        child.current_dir(cwd);

        // Inject env vars
        for (k, v) in &req.env {
            child.env(k, v);
        }

        let fut = child.output();
        let result = timeout(Duration::from_millis(timeout_ms), fut).await;

        match result {
            Err(_) => {
                warn!("shell/run TIMEOUT: {}", cmd);
                Err(ToolError {
                    code: ToolErrorCode::Timeout,
                    message: format!("Command timed out after {}ms: {}", timeout_ms, cmd),
                    detail: None,
                })
            }
            Ok(Err(e)) => Err(ToolError {
                code: ToolErrorCode::ExecFailed,
                message: e.to_string(),
                detail: None,
            }),
            Ok(Ok(output)) => {
                let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let mut truncated = false;

                let limit = self.output_limit;
                if stdout.len() > limit {
                    stdout.truncate(limit);
                    stdout.push_str("\n[...OUTPUT TRUNCATED...]");
                    truncated = true;
                }
                if stderr.len() > limit {
                    stderr.truncate(limit);
                    stderr.push_str("\n[...STDERR TRUNCATED...]");
                    truncated = true;
                }

                let exit_code = output.status.code().unwrap_or(-1);
                let success = output.status.success();

                if !success {
                    return Err(ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Exit code {}: {}", exit_code, stderr),
                        detail: Some(serde_json::json!({
                            "exit_code": exit_code,
                            "stdout": stdout,
                            "stderr": stderr,
                            "truncated": truncated,
                        })),
                    });
                }

                Ok(serde_json::json!({
                    "exit_code": exit_code,
                    "stdout": stdout,
                    "stderr": stderr,
                    "truncated": truncated,
                }))
            }
        }
    }
}
