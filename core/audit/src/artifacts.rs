//! Artifacts manifest: artifacts.json
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactEntry {
    pub path: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub producer_step: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtifactsManifest {
    pub run_id: String,
    pub entries: Vec<ArtifactEntry>,
}

impl ArtifactsManifest {
    pub fn new(run_id: &str) -> Self {
        Self { run_id: run_id.to_string(), entries: Vec::new() }
    }

    pub fn add(&mut self, entry: ArtifactEntry) {
        self.entries.push(entry);
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentFingerprint {
    pub os: String,
    pub arch: String,
    pub build_toolchains: std::collections::HashMap<String, String>,
}

impl EnvironmentFingerprint {
    pub fn capture() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        Self {
            os: sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown".into()),
            arch: std::env::consts::ARCH.to_string(),
            build_toolchains: std::collections::HashMap::new(),
        }
    }
}

pub struct BundleArchiver;

impl BundleArchiver {
    pub fn create_bundle(run_id: &str, output_dir: &Path) -> anyhow::Result<PathBuf> {
        let run_dir = Path::new("runs").join(run_id);
        std::fs::create_dir_all(output_dir)?;
        let zip_path = output_dir.join(format!("{}.zip", run_id));
        let file = File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);

        #[allow(deprecated)]
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        let mut files_to_zip = vec![
            ("audit.jsonl", run_dir.join("audit.jsonl")),
            ("metrics_summary.json", run_dir.join("metrics_summary.json")),
            ("artifacts.json", run_dir.join("artifacts.json")),
        ];

        let fingerprint = EnvironmentFingerprint::capture();
        let fingerprint_path = run_dir.join("fingerprint.json");
        std::fs::write(&fingerprint_path, serde_json::to_string_pretty(&fingerprint)?)?;
        files_to_zip.push(("fingerprint.json", fingerprint_path));

        // Attempt to collect SBOM if it exists upstream
        let sbom_path = Path::new("sdist").join("sbom.json");
        if sbom_path.exists() {
            files_to_zip.push(("sbom.json", sbom_path));
        }

        for (name, path) in files_to_zip {
            if path.exists() {
                zip.start_file(name, options)?;
                let mut f = File::open(path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }
        zip.finish()?;
        Ok(zip_path)
    }
}
