use crate::SandboxProvider;
use ada_tool_api::{ToolRequest, ToolResponse, ToolError, ToolErrorCode};
use async_trait::async_trait;
use tracing::{info, error};

pub struct RemoteSandboxProvider {
    pub endpoint: String,
    pub http_client: reqwest::Client,
}

impl RemoteSandboxProvider {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SandboxProvider for RemoteSandboxProvider {
    async fn create_env(&self) -> anyhow::Result<()> {
        info!("Remote sandbox env connected at endpoint: {}", self.endpoint);
        Ok(())
    }

    async fn run_step(&self, req: &ToolRequest) -> anyhow::Result<ToolResponse> {
        let url = format!("{}/api/v1/tools/execute", self.endpoint);
        
        info!("Dispatching Tool Request to Remote Worker: {}/{}", req.tool, req.action);
        let res = self.http_client.post(&url)
            .json(req)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let tool_resp: ToolResponse = resp.json().await?;
                Ok(tool_resp)
            }
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                error!("Remote worker rejected request: {} - {}", status, text);
                Ok(ToolResponse::err(&req.id, ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: format!("Worker HTTP {}", status),
                    detail: Some(text.into()),
                }, 0))
            }
            Err(e) => {
                error!("Network failure triggering Remote Worker: {}", e);
                Ok(ToolResponse::err(&req.id, ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: "Network Transport Failure".into(),
                    detail: Some(e.to_string().into()),
                }, 0))
            }
        }
    }

    async fn snapshot_create(&self, _name: &str) -> anyhow::Result<()> {
        // Handled centrally via Job Bundle ZIPs instead of individual snapshots
        Ok(())
    }

    async fn snapshot_restore(&self, _name: &str) -> anyhow::Result<()> {
        Ok(())
    }

    async fn teardown(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
