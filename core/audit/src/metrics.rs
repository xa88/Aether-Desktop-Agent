//! RunMetrics collector for Phase 12.

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RunMetrics {
    pub total_rounds: u32,
    pub cloud_rounds: u32,
    pub cache_hits: u32,
    pub self_heal_hits: u32,
    pub step_durations_ms: Vec<StepDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDuration {
    pub step_id: String,
    pub duration_ms: u64,
    pub tool: String,
}

lazy_static::lazy_static! {
    static ref GLOBAL_RUN_METRICS: Mutex<RunMetrics> = Mutex::new(RunMetrics::default());
}

pub fn record_cloud_round() {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        m.cloud_rounds += 1;
        m.total_rounds += 1;
    }
}

pub fn record_local_round() {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        m.total_rounds += 1;
    }
}

pub fn record_cache_hit() {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        m.cache_hits += 1;
    }
}

pub fn record_self_heal() {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        m.self_heal_hits += 1;
    }
}

pub fn record_step_duration(step_id: String, duration_ms: u64, tool: String) {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        m.step_durations_ms.push(StepDuration { step_id, duration_ms, tool });
    }
}

pub fn get_metrics() -> RunMetrics {
    if let Ok(m) = GLOBAL_RUN_METRICS.lock() {
        m.clone()
    } else {
        RunMetrics::default()
    }
}

pub fn flush_run_metrics<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    if let Ok(m) = GLOBAL_RUN_METRICS.lock() {
        let json = serde_json::to_string_pretty(&*m)?;
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, json)?;
    }
    Ok(())
}

pub fn reset_metrics() {
    if let Ok(mut m) = GLOBAL_RUN_METRICS.lock() {
        *m = RunMetrics::default();
    }
}
