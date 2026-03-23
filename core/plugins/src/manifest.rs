//! Plugin Manifest schema and validation.

use serde::{Deserialize, Serialize};
use ada_tool_api::RiskTier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub capabilities: PluginCapabilities,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub tools: Vec<String>,
    pub paths: Vec<String>,
    #[serde(default = "default_network")]
    pub network: String,
    pub max_risk_tier: RiskTier,
}

fn default_network() -> String {
    "deny".to_string()
}

impl PluginManifest {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.id.is_empty() {
            return Err(anyhow::anyhow!("Plugin ID cannot be empty"));
        }
        // Add more validation as needed (regex for ID, version format, etc.)
        Ok(())
    }
}
