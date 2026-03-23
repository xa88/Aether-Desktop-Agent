//! CapabilityGuard: Enforces plugin-specific permissions.

use crate::manifest::PluginManifest;
use ada_tool_api::{ToolRequest, ToolErrorCode, ToolError};
use ada_policy::PolicyGuard;
use std::path::PathBuf;

#[derive(Clone)]
pub struct CapabilityGuard {
    manifest: PluginManifest,
    inner_policy: PolicyGuard,
}

impl CapabilityGuard {
    pub fn new(manifest: PluginManifest, allowed_roots: Vec<String>, blocked_patterns: Vec<String>) -> Self {
        let inner_policy = PolicyGuard::new(allowed_roots, blocked_patterns);
        Self { manifest, inner_policy }
    }

    /// Verifies if a tool request is allowed by the plugin's manifest.
    pub fn check_request(&self, req: &ToolRequest) -> Result<(), ToolError> {
        // 1. Check Tool Capability
        let tool_action = format!("{}_{}", req.tool, req.action);
        
        if !self.manifest.capabilities.tools.contains(&req.tool) && 
           !self.manifest.capabilities.tools.contains(&tool_action) {
            return Err(ToolError {
                code: ToolErrorCode::PermissionDenied,
                message: format!("Plugin '{}' does not have capability for tool '{}/{}'", 
                    self.manifest.id, req.tool, req.action),
                detail: None,
            });
        }

        // 2. Check Risk Tier
        if req.risk_tier > self.manifest.capabilities.max_risk_tier {
            return Err(ToolError {
                code: ToolErrorCode::RiskTierBlocked,
                message: format!("Plugin '{}' maximum risk tier is {:?}, but request is {:?}", 
                    self.manifest.id, self.manifest.capabilities.max_risk_tier, req.risk_tier),
                detail: None,
            });
        }

        // 3. Path Guard (if applicable in args)
        if let Some(path_val) = req.args.get("path").and_then(|v| v.as_str()) {
            if let Err(e) = self.check_path(path_val) {
                return Err(ToolError {
                    code: ToolErrorCode::PathEscape,
                    message: e.to_string(),
                    detail: None,
                });
            }
        }

        Ok(())
    }

    pub fn check_path(&self, raw: &str) -> anyhow::Result<PathBuf> {
        // First check against plugin's own allowed paths
        let p = std::path::Path::new(raw);
        let allowed_by_manifest = self.manifest.capabilities.paths.iter().any(|root| {
            p.starts_with(root)
        });

        if !allowed_by_manifest && !self.manifest.capabilities.paths.is_empty() {
            return Err(anyhow::anyhow!("Path '{}' is outside plugin manifest allowed paths", raw));
        }

        // Then check against global host policy (symlinks, escapes, etc.)
        self.inner_policy.check_path(raw).map_err(|e| anyhow::anyhow!(e))
    }
}
