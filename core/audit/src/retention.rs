use std::path::{Path, PathBuf};
use std::time::SystemTime;
use fs_extra::dir::get_size;
use tracing::{info, warn};

pub struct RetentionPolicy {
    pub max_days: u64,
    pub max_size_mb: u64,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_days: 7,
            max_size_mb: 500,
        }
    }
}

pub fn cleanup_old_runs<P: AsRef<Path>>(runs_dir: P, policy: RetentionPolicy) -> anyhow::Result<()> {
    let runs_dir = runs_dir.as_ref();
    if !runs_dir.exists() {
        return Ok(());
    }

    let cutoff_time = SystemTime::now() - std::time::Duration::from_secs(policy.max_days * 24 * 3600);
    
    // 1. Delete by age
    for entry in std::fs::read_dir(runs_dir)?.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && wait_is_safe_run_dir(&path) {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff_time {
                        info!("Retention Policy Engine deleting stale run: {:?}", path);
                        let _ = std::fs::remove_dir_all(&path);
                    }
                }
            }
        }
    }

    // 2. Delete by total size if exceeding max_size_mb
    let mut total_size_bytes = get_size(runs_dir).unwrap_or(0);
    let max_size_bytes = policy.max_size_mb * 1024 * 1024;
    
    if total_size_bytes > max_size_bytes {
        warn!("Runs directory size {} MB exceeds limit {} MB. Trimming oldest...", total_size_bytes / 1024 / 1024, policy.max_size_mb);
        
        // Collect and sort remaining run directories by modified time
        let mut runs: Vec<(PathBuf, SystemTime)> = std::fs::read_dir(runs_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir() && wait_is_safe_run_dir(&e.path()))
            .filter_map(|e| e.metadata().ok().and_then(|m| m.modified().ok()).map(|time| (e.path(), time)))
            .collect();
            
        // Sort oldest first
        runs.sort_by(|a, b| a.1.cmp(&b.1));
        
        for (path, _) in runs {
            if total_size_bytes <= max_size_bytes {
                break;
            }
            if let Ok(size) = get_size(&path) {
                info!("Capacity Policy Engine deleting run: {:?}", path);
                if std::fs::remove_dir_all(&path).is_ok() {
                    total_size_bytes = total_size_bytes.saturating_sub(size);
                }
            }
        }
    }

    Ok(())
}

fn wait_is_safe_run_dir(path: &Path) -> bool {
    // Only delete directories that look like uuid/run-id format.
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name != "bundles") // Do not delete the global bundles directory!
        .unwrap_or(false)
}
