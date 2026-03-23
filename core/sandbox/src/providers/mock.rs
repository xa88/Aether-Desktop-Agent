//! Mock sandbox provider.

use crate::{SandboxProvider};
use ada_tool_api::{ToolRequest, ToolResponse};
use async_trait::async_trait;

pub struct MockSandboxProvider;

#[async_trait]
impl SandboxProvider for MockSandboxProvider {
    async fn create_env(&self) -> anyhow::Result<()> {
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
