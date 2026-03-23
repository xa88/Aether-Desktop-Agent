//! ada-playbooks: FixIt Playbook engine for automated self-healing.
pub mod recipes;

use serde::{Deserialize, Serialize};
use regex::Regex;
use std::path::{Path};
use walkdir::WalkDir;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub id: String,
    pub description: String,
    pub matches: Vec<String>,
    pub fix_steps: Vec<serde_json::Value>,
    #[serde(default)]
    pub check_steps: Vec<serde_json::Value>,
}

pub struct PlaybookRegistry {
    pub playbooks: Vec<Playbook>,
}

impl PlaybookRegistry {
    pub fn new() -> Self {
        Self { playbooks: vec![] }
    }

    pub fn load_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> anyhow::Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            warn!("Playbook directory does not exist: {}", dir.display());
            return Ok(());
        }

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(entry.path())?;
                match serde_yaml::from_str::<Playbook>(&content) {
                    Ok(pb) => {
                        info!("Loaded playbook: {} ({})", pb.id, pb.description);
                        self.playbooks.push(pb);
                    }
                    Err(e) => {
                        error!("Failed to parse playbook at {}: {}", entry.path().display(), e);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn find_match(&self, signature: &str) -> Option<&Playbook> {
        for pb in &self.playbooks {
            for pattern in &pb.matches {
                if let Ok(re) = Regex::new(pattern) {
                    if re.is_match(signature) {
                        return Some(pb);
                    }
                } else if signature.contains(pattern) {
                    return Some(pb);
                }
            }
        }
        None
    }
}
