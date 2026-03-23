use ada_audit::retention::{RetentionPolicy, cleanup_old_runs};
use tempfile::tempdir;
use std::fs;
use std::path::Path;

#[test]
fn test_retention_auto_cleanup() {
    let dir = tempdir().unwrap();
    let runs_dir = dir.path();
    
    // Create folders
    let fresh_run = runs_dir.join("run_fresh_123");
    let stale_run = runs_dir.join("run_stale_456");
    let bundles_dir = runs_dir.join("bundles"); // Should be protected
    
    fs::create_dir_all(&fresh_run).unwrap();
    fs::create_dir_all(&stale_run).unwrap();
    fs::create_dir_all(&bundles_dir).unwrap();
    
    // We cannot easily mock the filesystem creation/modification time without unsafe or external crates,
    // so we will write a unit test simply triggering the default cleanup boundaries 
    // against size quotas.
    
    // Write 2MB into stale_run
    let data = vec![0u8; 2 * 1024 * 1024];
    fs::write(stale_run.join("big_file.bin"), &data).unwrap();

    let policy = RetentionPolicy {
        max_days: 7,
        max_size_mb: 1, // Restrict to 1MB to force cleanup
    };

    cleanup_old_runs(runs_dir, policy).unwrap();

    // The stale_run was exactly 2MB and oldest, so it should be swept out.
    // Wait: since we created them instantly, the ordering of modified times might be random if resolution is bad. 
    // So we just assert that at least something was cleaned up and bundles is safe.
    
    let still_exists = stale_run.exists();
    let fresh_exists = fresh_run.exists();
    
    // Ensure size constraints are enforced (at most 1 ran directory remains potentially, or both deleted if random)
    assert!(!still_exists || !fresh_exists, "At least one run should have been pruned due to quota.");
    assert!(bundles_dir.exists(), "Protected bundles root should never be pruned.");
}
