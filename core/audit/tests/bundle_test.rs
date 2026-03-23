use ada_audit::artifacts::{BundleArchiver, ArtifactsManifest, ArtifactEntry, EnvironmentFingerprint};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_bundle_creation() {
    let dir = tempdir().unwrap();
    let run_id = "test-run-1234";
    
    // Mock the run directory
    let run_dir = std::path::Path::new("runs").join(run_id);
    std::fs::create_dir_all(&run_dir).unwrap();
    
    let mut audit = File::create(run_dir.join("audit.jsonl")).unwrap();
    writeln!(audit, "{{\"event\": \"test\"}}").unwrap();
    
    let mut metrics = File::create(run_dir.join("metrics_summary.json")).unwrap();
    writeln!(metrics, "{{\"total_tokens\": 100}}").unwrap();

    let mut manifest = ArtifactsManifest::new(run_id);
    manifest.add(ArtifactEntry {
        path: "/mock/path".to_string(),
        sha256: "0000".to_string(),
        size_bytes: 1024,
        producer_step: "test".to_string(),
        description: None,
    });
    std::fs::write(run_dir.join("artifacts.json"), manifest.to_json().unwrap()).unwrap();

    // Create the bundle
    let output_dir = dir.path();
    let zip_path = BundleArchiver::create_bundle(run_id, output_dir).unwrap();
    
    assert!(zip_path.exists());
    
    // Verify contents
    let file = File::open(&zip_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    
    let mut has_audit = false;
    let mut has_metrics = false;
    let mut has_artifacts = false;
    let mut has_fingerprint = false;

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        match file.name() {
            "audit.jsonl" => has_audit = true,
            "metrics_summary.json" => has_metrics = true,
            "artifacts.json" => has_artifacts = true,
            "fingerprint.json" => has_fingerprint = true,
            _ => {}
        }
    }

    assert!(has_audit && has_metrics && has_artifacts && has_fingerprint);

    // Clean up mock runs dir
    std::fs::remove_dir_all(&run_dir).unwrap();
}
