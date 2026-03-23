//! PluginHost: Orchestrates plugin execution and security.

use std::collections::HashMap;
use std::sync::Arc;
use crate::manifest::PluginManifest;
use crate::guard::CapabilityGuard;
use crate::signing::PluginVerifier;
use crate::runner::ProcessPluginRunner;
use ada_tool_api::router::ToolRouter;
use ada_tool_api::{ToolRequest, ToolResponse, ToolError, ToolErrorCode};

pub struct PluginInstance {
    pub manifest: PluginManifest,
    pub guard: CapabilityGuard,
    pub runner: Option<ProcessPluginRunner>,
}

pub struct PluginHost {
    plugins: HashMap<String, Arc<PluginInstance>>,
    router: Arc<ToolRouter>,
    verifier: Option<PluginVerifier>,
}

impl PluginHost {
    pub fn new(router: Arc<ToolRouter>, root_key: Option<[u8; 32]>) -> Self {
        Self {
            plugins: HashMap::new(),
            router,
            verifier: root_key.map(|k| PluginVerifier::new(&k)),
        }
    }

    pub fn load_plugin(
        &mut self, 
        manifest: PluginManifest, 
        allowed_roots: Vec<String>, 
        blocked_patterns: Vec<String>,
        executable_path: Option<String>
    ) -> anyhow::Result<()> {
        manifest.validate()?;

        // Verify signature if a verifier is configured
        if let Some(verifier) = &self.verifier {
            let signature = manifest.signature.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Plugin '{}' is missing a signature", manifest.id))?;
            
            let mut manifest_for_sig = manifest.clone();
            manifest_for_sig.signature = None;
            let data = serde_json::to_vec(&manifest_for_sig)?;
            
            verifier.verify(&data, signature)?;
        }

        let id = manifest.id.clone();
        let guard = CapabilityGuard::new(manifest.clone(), allowed_roots, blocked_patterns);
        
        let runner = executable_path.map(|path| {
            let r = ProcessPluginRunner::new(manifest.clone(), guard.clone());
            // Note: In a real scenario, we might delay starting until first use
            let _ = r.start(&path);
            r
        });

        let instance = Arc::new(PluginInstance { manifest, guard, runner });
        self.plugins.insert(id, instance);
        Ok(())
    }

    pub async fn dispatch_for_plugin(&self, plugin_id: &str, req: &mut ToolRequest) -> ToolResponse {
        let instance = match self.plugins.get(plugin_id) {
            Some(i) => i,
            None => return ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::NotFound,
                message: format!("Plugin '{}' not found", plugin_id),
                detail: None,
            }, 0),
        };

        // If plugin has a dedicated runner (sandbox), use it
        if let Some(runner) = &instance.runner {
            return runner.call_tool(req).await;
        }

        // Otherwise fallback to in-process capability enforcement
        if let Err(e) = instance.guard.check_request(req) {
            return ToolResponse::err(&req.id, e, 0);
        }

        self.router.dispatch(req).await
    }
}
