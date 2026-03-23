//! Error Signature Extraction and Clustering.

use regex::Regex;
use std::collections::HashMap;

pub struct ErrorExtractor {
    patterns: Vec<(String, Regex)>,
}

impl ErrorExtractor {
    pub fn new() -> Self {
        let patterns = vec![
            ("rust".to_string(), Regex::new(r"error\[E\d+\]:.*").unwrap()),
            ("tsc".to_string(), Regex::new(r"error TS\d+:.*").unwrap()),
            ("python".to_string(), Regex::new(r"Traceback \(most recent call last\):").unwrap()),
            ("generic".to_string(), Regex::new(r"(?i)failed|error|exception").unwrap()),
        ];
        Self { patterns }
    }

    pub fn extract_signatures(&self, log: &str) -> Vec<String> {
        let mut signatures = Vec::new();
        for line in log.lines() {
            for (_, re) in &self.patterns {
                if let Some(mat) = re.find(line) {
                    signatures.push(mat.as_str().to_string());
                    break;
                }
            }
        }
        signatures
    }

    pub fn cluster_errors(&self, signatures: Vec<String>) -> Vec<(String, usize)> {
        let mut counts = HashMap::new();
        for sig in signatures {
            *counts.entry(sig).or_insert(0) += 1;
        }
        let mut result: Vec<_> = counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
}
