//! Async JSONL audit writer backed by a file.

use crate::events::AuditEvent;
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct AuditWriter {
    file: Arc<Mutex<tokio::fs::File>>,
}

impl AuditWriter {
    pub async fn open(path: &str) -> Result<Self> {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        Ok(Self { file: Arc::new(Mutex::new(f)) })
    }

    pub async fn write(&self, event: &AuditEvent) -> Result<()> {
        let mut line = serde_json::to_string(event)?;
        
        // Output Memory Limit Degradation (Max 500 KB per log trace to prevent bloat)
        const MAX_BYTES: usize = 500 * 1024;
        if line.len() > MAX_BYTES {
            line = format!("{}...[TRUNCATED_BY_BUDGET_ENGINE]", &line[..MAX_BYTES]);
        }
        
        // PII and Secret Extraction
        line = ada_redaction::Redactor::redact(&line);
        
        line.push('\n');
        let mut f = self.file.lock().await;
        f.write_all(line.as_bytes()).await?;
        f.flush().await?;
        Ok(())
    }
}
