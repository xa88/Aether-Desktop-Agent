//! Docker sandbox provider (mock for now).

use crate::{SandboxProvider};
use ada_tool_api::{ToolRequest, ToolResponse};
use async_trait::async_trait;
use tracing::info;

pub struct DockerProvider {
    pub exposed_port: Option<u16>,
}

impl DockerProvider {
    pub fn new(exposed_port: Option<u16>) -> Self {
        Self { exposed_port }
    }
}

#[async_trait]
impl SandboxProvider for DockerProvider {
    async fn create_env(&self) -> anyhow::Result<()> {
        if let Some(port) = self.exposed_port {
            info!("Booting Docker Sandbox Container linking physical Loopback on 127.0.0.1:{}", port);
        } else {
            info!("Booting standard isolated Docker Sandbox");
        }
        Ok(())
    }
    async fn run_step(&self, _req: &ToolRequest) -> anyhow::Result<ToolResponse> {
        Ok(ToolResponse::ok("mock", serde_json::json!({}), 0))
    }
    async fn snapshot_create(&self, _name: &str) -> anyhow::Result<()> {
        Ok(())
    }
    async fn snapshot_restore(&self, _name: &str) -> anyhow::Result<()> {
        Ok(())
    }
    async fn teardown(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
