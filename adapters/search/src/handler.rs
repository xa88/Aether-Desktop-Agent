//! SearchHandler: ripgrep (rg) wrapper with structured JSON output.

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::ToolHandler;
use async_trait::async_trait;
use tokio::process::Command;

pub struct SearchHandler;

impl SearchHandler {
    pub fn new() -> Self { Self }
}

impl Default for SearchHandler {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl ToolHandler for SearchHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        let pattern = req.args["pattern"].as_str().ok_or_else(|| ToolError {
            code: ToolErrorCode::InvalidArgs,
            message: "Missing 'pattern' arg".to_string(),
            detail: None,
        })?;
        let path = req.args["path"].as_str().unwrap_or(".");
        let case_insensitive = req.args["case_insensitive"].as_bool().unwrap_or(false);
        let max_results = req.args["max_results"].as_u64().unwrap_or(50);

        let mut cmd = Command::new("rg");
        cmd.arg("--json");
        cmd.arg("--max-count").arg(max_results.to_string());
        if case_insensitive { cmd.arg("-i"); }
        cmd.arg(pattern).arg(path);

        let output = cmd.output().await.map_err(|_| {
            // rg not found - fall back to grep-like message
            ToolError {
                code: ToolErrorCode::NotFound,
                message: "ripgrep (rg) not found in PATH. Install it: https://github.com/BurntSushi/ripgrep".to_string(),
                detail: None,
            }
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // Parse rg --json output into structured matches
        let matches: Vec<serde_json::Value> = stdout
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .filter(|v: &serde_json::Value| v["type"] == "match")
            .map(|v| {
                let data = &v["data"];
                serde_json::json!({
                    "file": data["path"]["text"],
                    "line": data["line_number"],
                    "text": data["lines"]["text"],
                })
            })
            .collect();

        Ok(serde_json::json!({ "matches": matches, "count": matches.len() }))
    }
}
