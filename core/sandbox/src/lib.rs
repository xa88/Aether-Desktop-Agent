//! ada-sandbox: Environment abstraction for VM and Container isolation.

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest, ToolResponse, RiskTier};
use async_trait::async_trait;
use tracing::{info, warn};
use std::sync::Arc;

#[async_trait]
pub trait SandboxProvider: Send + Sync {
    async fn create_env(&self) -> anyhow::Result<()>;
    async fn run_step(&self, req: &ToolRequest) -> anyhow::Result<ToolResponse>;
    async fn snapshot_create(&self, name: &str) -> anyhow::Result<()>;
    async fn snapshot_restore(&self, name: &str) -> anyhow::Result<()>;
    async fn teardown(&self) -> anyhow::Result<()>;
}

pub struct SandboxManager {
    provider: Arc<dyn SandboxProvider>,
    execution_mode: String,
}

impl SandboxManager {
    pub fn new(provider: Arc<dyn SandboxProvider>, execution_mode: String) -> Self {
        Self {
            provider,
            execution_mode,
        }
    }

    /// Dispatch based on risk tier and execution mode
    pub async fn dispatch(&self, req: &ToolRequest, local_router: &ada_tool_api::router::ToolRouter) -> ToolResponse {
        let is_high_risk = match req.risk_tier {
            RiskTier::T2 | RiskTier::T3 => true,
            _ => false,
        };

        if self.execution_mode == "host" && is_high_risk {
            warn!("Blocked high-risk action {}/{} on host.", req.tool, req.action);
            return ToolResponse::err(
                &req.id,
                ToolError {
                    code: ToolErrorCode::RiskTierBlocked,
                    message: format!("High risk (T2/T3) action '{}' not permitted in 'host' mode. Requires 'sandbox' or 'hybrid'.", req.tool),
                    detail: None,
                },
                0
            );
        }

        if (self.execution_mode == "sandbox" || is_high_risk) && self.execution_mode != "host" {
            info!("Routing action {}/{} to sandbox.", req.tool, req.action);
            match self.provider.run_step(req).await {
                Ok(resp) => resp,
                Err(e) => ToolResponse::err(
                    &req.id,
                    ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Sandbox error: {}", e),
                        detail: None,
                    },
                    0
                )
            }
        } else {
            // Run locally via the provided ToolRouter
            local_router.dispatch(req).await
        }
    }

    pub async fn snapshot_create(&self, name: &str) -> anyhow::Result<()> {
        info!("Creating sandbox snapshot: {}", name);
        self.provider.snapshot_create(name).await
    }

    pub async fn snapshot_restore(&self, name: &str) -> anyhow::Result<()> {
        info!("Restoring sandbox snapshot: {}", name);
        self.provider.snapshot_restore(name).await
    }
}

pub mod providers {
    pub mod docker;
    pub mod mock;
    pub mod remote;
}
