use ada_audit::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_metrics_collection_and_flushing() {
    reset_metrics();
    
    record_cloud_round();
    record_local_round();
    record_cache_hit();
    record_self_heal();
    record_step_duration("step-1".to_string(), 100, "shell_run".to_string());
    
    let metrics = get_metrics();
    assert_eq!(metrics.cloud_rounds, 1);
    assert_eq!(metrics.total_rounds, 2);
    assert_eq!(metrics.cache_hits, 1);
    assert_eq!(metrics.self_heal_hits, 1);
    assert_eq!(metrics.step_durations_ms.len(), 1);
    assert_eq!(metrics.step_durations_ms[0].step_id, "step-1");
    
    let dir = tempdir().unwrap();
    let path = dir.path().join("metrics.json");
    flush_run_metrics(&path).expect("Failed to flush metrics");
    
    let content = fs::read_to_string(path).expect("Failed to read flushed metrics");
    assert!(content.contains("\"cloud_rounds\": 1"));
    assert!(content.contains("\"step_id\": \"step-1\""));
}
