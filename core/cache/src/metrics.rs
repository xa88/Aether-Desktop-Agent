//! Metrics collection and local reporting.

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub time_saved_ms: u64,
}

lazy_static::lazy_static! {
    static ref GLOBAL_METRICS: Mutex<Metrics> = Mutex::new(Metrics::default());
}

pub fn record_hit(saved_ms: u64) {
    if let Ok(mut m) = GLOBAL_METRICS.lock() {
        m.cache_hits += 1;
        m.time_saved_ms += saved_ms;
    }
}

pub fn record_miss() {
    if let Ok(mut m) = GLOBAL_METRICS.lock() {
        m.cache_misses += 1;
    }
}

pub fn flush_metrics<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    if let Ok(m) = GLOBAL_METRICS.lock() {
        let json = serde_json::to_string_pretty(&*m)?;
        fs::write(path, json)?;
    }
    Ok(())
}
