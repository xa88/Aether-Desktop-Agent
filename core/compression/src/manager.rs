//! CompressionManager: Token-aware context summarization and audit pruning.

use ada_tool_api::{ToolRequest};
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct CompressionManager {
    token_threshold: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextSummary {
    pub original_length: usize,
    pub compressed_length: usize,
    pub summary: String,
    pub preserved_entities: Vec<String>,
}

impl CompressionManager {
    pub fn new(token_threshold: usize) -> Self {
        Self { token_threshold }
    }

    /// Summarizes a list of audit events or log lines into a compact narrative.
    pub async fn summarize_context(&self, raw_context: &str) -> Result<ContextSummary> {
        // In a real implementation, this would call an LLM (e.g. gpt-4o-mini) 
        // with a specialized 'summarization' prompt.
        
        let original_length = raw_context.len();
        
        // Mocked summarization logic for Phase 9
        let summary = if original_length > 1000 {
            format!("COMPRESSED CONTEXT: The agent previously attempted to {} but encountered several intermediate steps. Key entities: [project-root, node_modules, .env]. Result: Pending.", 
                &raw_context[0..50])
        } else {
            raw_context.to_string()
        };

        Ok(ContextSummary {
            original_length,
            compressed_length: summary.len(),
            summary,
            preserved_entities: vec!["project-root".into(), ".env".into()],
        })
    }

    /// Prunes successful tool requests from the audit trail to save space.
    pub fn prune_audit_trail(&self, requests: Vec<ToolRequest>) -> Vec<ToolRequest> {
        // Keep failures and major milestones, discard repeated minor successes
        requests.into_iter().filter(|r| {
            r.tool == "fs_write" || r.tool == "git_commit" // Example 'significant' tools
        }).collect()
    }
}
