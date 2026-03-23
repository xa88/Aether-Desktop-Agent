//! Secret pattern scanner — redacts API keys, JWTs, SSH keys, PEM blocks.

use regex::Regex;

pub struct SecretScanner {
    patterns: Vec<(&'static str, Regex)>,
}

impl SecretScanner {
    pub fn new() -> Self {
        let raw = vec![
            ("api_key",      r"(?i)(sk-|api[_-]?key)[a-zA-Z0-9\-_]{20,}"),
            ("aws_access_key", r"AKIA[0-9A-Z]{16}"),
            ("aws_secret_key", r"(?i)aws_secret[_-]?key\s*[:=]\s*[a-zA-Z0-9/=+]{40}"),
            ("jwt",          r"eyJ[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}"),
            ("pem_block",    r"-----BEGIN [A-Z ]+-----[\s\S]*?-----END [A-Z ]+-----"),
            ("ssh_private",  r"-----BEGIN OPENSSH PRIVATE KEY-----[\s\S]*?-----END OPENSSH PRIVATE KEY-----"),
            ("generic_secret", r"(?i)(password|secret|token|credential)\s*[:=]\s*[^\s]{10,}"),
            ("bearer_token", r"(?i)bearer\s+[a-zA-Z0-9\-_\.]{20,}"),
        ];
        let patterns = raw.into_iter()
            .filter_map(|(name, pat)| Regex::new(pat).ok().map(|r| (name, r)))
            .collect();
        Self { patterns }
    }

    /// Scan text and return list of (label, match_snippet) found.
    pub fn scan<'a>(&self, text: &'a str) -> Vec<(&'static str, &'a str)> {
        let mut found = vec![];
        for (name, pat) in &self.patterns {
            for m in pat.find_iter(text) {
                found.push((*name, m.as_str()));
            }
        }
        found
    }

    /// Redact secrets, replacing matches with [REDACTED:<label>].
    pub fn redact(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (name, pat) in &self.patterns {
            result = pat.replace_all(&result, format!("[REDACTED:{name}]").as_str()).to_string();
        }
        result
    }

    /// Global utility for one-off scrubbing.
    pub fn scrub(text: &str) -> String {
        let scanner = Self::new();
        scanner.redact(text)
    }
}

impl Default for SecretScanner {
    fn default() -> Self { Self::new() }
}
