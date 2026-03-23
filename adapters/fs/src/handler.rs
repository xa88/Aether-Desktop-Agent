//! FsHandler: transactional fs operations (temp -> fsync -> atomic rename).

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::ToolHandler;
use ada_policy::guard::{PolicyGuard, PolicyError};
use async_trait::async_trait;
use sha2::{Sha256, Digest};
use std::path::{PathBuf};
use tokio::fs;
use tracing::{info, warn};

pub struct FsHandler {
    guard: PolicyGuard,
}

impl FsHandler {
    pub fn new(allowed_roots: Vec<String>, blocked_patterns: Vec<String>) -> Self {
        let handler = Self { guard: PolicyGuard::new(allowed_roots, blocked_patterns) };
        // Trigger non-blocking cleanup
        let _ = handler.cleanup_quarantine(7); 
        handler
    }

    /// Cleanup quarantine files older than `days` old.
    pub fn cleanup_quarantine(&self, days: i64) -> anyhow::Result<()> {
        let quarantine_root = PathBuf::from(".ada_quarantine");
        if !quarantine_root.exists() {
            return Ok(());
        }

        let now = chrono::Local::now();
        let threshold = chrono::Duration::days(days);

        // Simple synchronous cleanup for now, as it's called in new()
        if let Ok(entries) = std::fs::read_dir(&quarantine_root) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    // Folders are named YYYY-MM-DD
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(&name, "%Y-%m-%d") {
                        // Calculate age
                        let duration = now.date_naive() - date;
                        if duration > threshold {
                            info!("Cleaning up old quarantine folder: {}", name);
                            let _ = std::fs::remove_dir_all(entry.path());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn guard_path(&self, p: &str) -> Result<PathBuf, ToolError> {
        self.guard.check_path(p).map_err(|e| match e {
            PolicyError::PathEscape { path } => ToolError {
                code: ToolErrorCode::PathEscape,
                message: format!("Path '{}' outside allowed roots", path),
                detail: None,
            },
            _ => ToolError {
                code: ToolErrorCode::PolicyViolation,
                message: e.to_string(),
                detail: None,
            },
        })
    }
}

#[async_trait]
impl ToolHandler for FsHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        match req.action.as_str() {
            "read" => {
                let path = req.args["path"].as_str().unwrap_or_default();
                let pb = self.guard_path(path)?;
                let content = fs::read_to_string(&pb).await.map_err(|e| ToolError {
                    code: ToolErrorCode::NotFound,
                    message: e.to_string(),
                    detail: None,
                })?;
                Ok(serde_json::json!({ "content": content }))
            }
            "write" => {
                let path = req.args["path"].as_str().unwrap_or_default();
                let content = req.args["content"].as_str().unwrap_or_default();
                let pb = self.guard_path(path)?;

                // Transactional write: staging -> fsync -> atomic rename
                let staging_dir = PathBuf::from(".ada_staging");
                if !staging_dir.exists() {
                    fs::create_dir_all(&staging_dir).await.ok();
                }

                let file_id = uuid::Uuid::new_v4().to_string();
                let tmp = staging_dir.join(file_id);
                
                // Use std::fs for synchronous write+sync within the transactional block
                // to avoid racing with tokio's background close/flush on Windows
                {
                    use std::io::Write;
                    let mut file = std::fs::File::create(&tmp).map_err(|e| ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Failed to create staging file: {}", e),
                        detail: None,
                    })?;
                    file.write_all(content.as_bytes()).map_err(|e| ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Failed to write to staging: {}", e),
                        detail: None,
                    })?;
                    file.sync_all().map_err(|e| ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Failed to fsync staging file: {}", e),
                        detail: None,
                    })?;
                }

                // Commit: atomic rename
                fs::rename(&tmp, &pb).await.map_err(|e| ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: format!("Failed to commit file from staging: {}", e),
                    detail: None,
                })?;

                info!("fs/write (transactional): {}", path);
                let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
                Ok(serde_json::json!({ "path": path, "sha256": hash }))
            }
            "mkdir" => {
                let path = req.args["path"].as_str().unwrap_or_default();
                let pb = self.guard_path(path)?;
                fs::create_dir_all(&pb).await.map_err(|e| ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: e.to_string(),
                    detail: None,
                })?;
                Ok(serde_json::json!({ "created": path }))
            }
            "list" => {
                let path = req.args["path"].as_str().unwrap_or(".");
                let mut entries = vec![];
                let mut dir = fs::read_dir(path).await.map_err(|e| ToolError {
                    code: ToolErrorCode::NotFound,
                    message: e.to_string(),
                    detail: None,
                })?;
                while let Ok(Some(entry)) = dir.next_entry().await {
                    entries.push(entry.file_name().to_string_lossy().to_string());
                }
                Ok(serde_json::json!({ "entries": entries }))
            }
            "delete" => {
                let path = req.args["path"].as_str().unwrap_or_default();
                let pb = self.guard_path(path)?;
                
                // Managed Quarantine: move to .ada_quarantine/[date]/
                let date_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let quarantine_root = PathBuf::from(".ada_quarantine").join(date_str);
                if !quarantine_root.exists() {
                    fs::create_dir_all(&quarantine_root).await.ok();
                }

                let file_name = pb.file_name().unwrap_or_default();
                let quarantine_path = quarantine_root.join(format!("{}_{}", uuid::Uuid::new_v4(), file_name.to_string_lossy()));
                
                fs::rename(&pb, &quarantine_path).await.map_err(|e| ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: format!("Failed to quarantine file: {}", e),
                    detail: None,
                })?;
                
                warn!("fs/delete quarantined: {} -> {}", path, quarantine_path.display());
                Ok(serde_json::json!({ "quarantined_at": quarantine_path.to_string_lossy() }))
            }
            _ => Err(ToolError {
                code: ToolErrorCode::InvalidArgs,
                message: format!("Unknown fs action: {}", req.action),
                detail: None,
            }),
        }
    }
}
