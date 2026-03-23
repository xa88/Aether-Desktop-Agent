//! Path root guard, command blacklist, symlink escape detection.

use regex::Regex;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("Path escape: '{path}' is outside allowed roots")]
    PathEscape { path: String },
    #[error("Blocked command pattern matched: '{pattern}'")]
    BlockedCommand { pattern: String },
    #[error("Risk tier {tier} requires pre-authorization on host")]
    RiskTierBlocked { tier: String },
}

#[derive(Clone)]
pub struct PolicyGuard {
    allowed_roots: Vec<PathBuf>,
    blocked_patterns: Vec<Regex>,
}

impl PolicyGuard {
    pub fn new(allowed_roots: Vec<String>, blocked_patterns: Vec<String>) -> Self {
        let roots = allowed_roots.into_iter().map(PathBuf::from).collect();
        let patterns = blocked_patterns
            .into_iter()
            .filter_map(|p| Regex::new(&p).ok())
            .collect();
        Self { allowed_roots: roots, blocked_patterns: patterns }
    }

    /// Verify a file path is within an allowed root and has no symlink escape.
    pub fn check_path(&self, raw: &str) -> Result<PathBuf, PolicyError> {
        let p = Path::new(raw);
        
        // Canonicalize to resolve symlinks, '..', and '.'
        // If it doesn't exist, we check the directory it's supposed to be in
        let canonical = if p.exists() {
            p.canonicalize().map_err(|_| PolicyError::PathEscape { path: raw.to_string() })?
        } else {
            // Traverse up to find the first existing parent to canonicalize
            let mut current = p;
            let mut suffix = PathBuf::new();
            while !current.exists() && current.parent().is_some() {
                if let Some(name) = current.file_name() {
                    let mut new_suffix = PathBuf::from(name);
                    new_suffix.push(suffix);
                    suffix = new_suffix;
                }
                current = current.parent().unwrap();
            }
            let mut resolved = current.canonicalize().unwrap_or_else(|_| PathBuf::from("."));
            resolved.push(suffix);
            resolved
        };

        let allowed = self.allowed_roots.iter().any(|root| {
            // Ensure both are canonicalized for comparison
            if let Ok(c_root) = root.canonicalize() {
                canonical.starts_with(c_root)
            } else {
                canonical.starts_with(root)
            }
        });

        if !allowed {
            return Err(PolicyError::PathEscape { path: raw.to_string() });
        }
        Ok(canonical)
    }

    /// Verify a shell command doesn't match any blocked pattern.
    pub fn check_cmd(&self, cmd: &str) -> Result<(), PolicyError> {
        for pat in &self.blocked_patterns {
            if pat.is_match(cmd) {
                return Err(PolicyError::BlockedCommand {
                    pattern: pat.as_str().to_string(),
                });
            }
        }
        Ok(())
    }
}

/// Default dangerous command patterns.
pub fn default_blocked_patterns() -> Vec<String> {
    vec![
        r"(?i)rm\s+-rf\s+/".to_string(),
        r"(?i)format\s+[a-z]:".to_string(),
        r"(?i)dd\s+if=".to_string(),
        r"(?i)mkfs\.".to_string(),
        r"(?i)shutdown".to_string(),
        r"(?i)reg\s+delete\s+HKLM".to_string(),
        r"(?i)powershell".to_string(),
        r"(?i)cmd\.exe".to_string(),
        r"(?i)sudo\s+rm".to_string(),
        r"(?i)chmod\s+777".to_string(),
    ]
}
