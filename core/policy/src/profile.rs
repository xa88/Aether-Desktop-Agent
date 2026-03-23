use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnterpriseProfile {
    // Defines explicit fully-qualified bounds for TCP egress.
    pub allowed_domains: Vec<String>,
    // Restricts the agent to writing or exploring outside these specific directories.
    pub restricted_paths: Vec<String>,
    // Maximum executor risk-tier execution allowance
    pub max_risk_tier: u8,
}

impl EnterpriseProfile {
    pub fn is_network_allowed(&self, target_host: &str) -> bool {
        if self.allowed_domains.is_empty() {
             return true; // No constraints applied yet
        }
        self.allowed_domains.iter().any(|domain| {
             target_host == domain || target_host.ends_with(&format!(".{}", domain))
        })
    }

    pub fn is_path_allowed(&self, target_path: &str) -> bool {
        if self.restricted_paths.is_empty() {
             return true; 
        }
        self.restricted_paths.iter().any(|allowed| target_path.starts_with(allowed))
    }
}
