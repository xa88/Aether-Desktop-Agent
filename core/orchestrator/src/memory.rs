use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tracing::{info, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceFragment {
    pub goal_signature: String,
    pub successful_plan_yaml: String,
    pub timestamp: i64,
    pub metadata: ExperienceMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceMeta {
    pub model: String,
    pub agent_count: usize,
    pub duration_ms: u64,
}

pub struct MemoryManager {
    pub storage_path: PathBuf,
}

impl MemoryManager {
    pub fn new(path: PathBuf) -> Self {
        if !path.exists() {
            std::fs::create_dir_all(&path).ok();
        }
        Self { storage_path: path }
    }

    pub fn save_experience(&self, fragment: ExperienceFragment) -> anyhow::Result<()> {
        let filename = format!("exp_{}.json", fragment.goal_signature.replace(" ", "_").to_lowercase());
        let full_path = self.storage_path.join(filename);
        let data = serde_json::to_string_pretty(&fragment)?;
        std::fs::write(full_path, data)?;
        info!("Experience saved: {}", fragment.goal_signature);
        Ok(())
    }

    pub fn recall_experience(&self, goal: &str) -> Option<ExperienceFragment> {
        // In a real system, this would use Vector Search. 
        // For MVP, we use exact or fuzzy match on the signature.
        let target = goal.to_lowercase();
        let entries = std::fs::read_dir(&self.storage_path).ok()?;
        
        for entry in entries {
            if let Ok(e) = entry {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if let Ok(fragment) = serde_json::from_str::<ExperienceFragment>(&content) {
                            if target.contains(&fragment.goal_signature.to_lowercase()) {
                                debug!("Memory hit: found past experience for '{}'", goal);
                                return Some(fragment);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
