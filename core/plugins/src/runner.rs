//! Process-based plugin runner.
//! Handles IPC communication with a plugin child process.

use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use ada_tool_api::{ToolRequest, ToolResponse, ToolError, ToolErrorCode};
use crate::manifest::PluginManifest;
use crate::guard::CapabilityGuard;

#[derive(Clone)]
pub struct ProcessPluginRunner {
    pub manifest: PluginManifest,
    pub guard: CapabilityGuard,
    child: Arc<Mutex<Option<Child>>>,
}

impl ProcessPluginRunner {
    pub fn new(manifest: PluginManifest, guard: CapabilityGuard) -> Self {
        Self {
            manifest,
            guard,
            child: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, executable_path: &str) -> anyhow::Result<()> {
        let child = Command::new(executable_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn plugin process: {}", e))?;
        
        // This is a simplified runner. In a real scenario, we'd manage the lifecycle.
        let mut guard = self.child.blocking_lock();
        *guard = Some(child);
        Ok(())
    }

    pub async fn call_tool(&self, req: &ToolRequest) -> ToolResponse {
        // Enforce capabilities before sending to child
        if let Err(e) = self.guard.check_request(req) {
            return ToolResponse::err(&req.id, e, 0);
        }

        let mut child_guard = self.child.lock().await;
        let child = match child_guard.as_mut() {
            Some(c) => c,
            None => return ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::ExecFailed,
                message: "Plugin process not started".to_string(),
                detail: None,
            }, 0),
        };

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let stdout = child.stdout.as_mut().expect("Failed to open stdout");

        // Send JSON-RPC request (simplified: one JSON line)
        let req_json = serde_json::to_string(&req).unwrap();
        if let Err(e) = writeln!(stdin, "{}", req_json) {
            return ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("Failed to write to plugin stdin: {}", e),
                detail: None,
            }, 0);
        }

        // Wait for JSON-RPC response
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        if let Err(e) = reader.read_line(&mut line) {
            return ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("Failed to read from plugin stdout: {}", e),
                detail: None,
            }, 0);
        }

        serde_json::from_str(&line).unwrap_or_else(|e| {
            ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("Invalid JSON from plugin: {}", e),
                detail: None,
            }, 0)
        })
    }
}
