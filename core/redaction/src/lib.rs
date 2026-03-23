use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}").unwrap();
    static ref IPV4_REGEX: Regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    static ref GENERIC_SECRET_REGEX: Regex = Regex::new(r#"(?i)(key|token|secret|password|bearer)[\s\=\:]+['"]?(\w{16,})['"”]??"#).unwrap();
}

pub struct Redactor;

impl Redactor {
    /// Scrub generic PII and Secrets from any raw string payload before it's persisted.
    pub fn redact(input: &str) -> String {
        let mut scrubbed = input.to_string();
        
        scrubbed = EMAIL_REGEX.replace_all(&scrubbed, "[REDACTED_EMAIL]").to_string();
        scrubbed = IPV4_REGEX.replace_all(&scrubbed, "[REDACTED_IP]").to_string();
        
        // Complex replacement to retain the 'key=' prefix but redact the actual token hash
        scrubbed = GENERIC_SECRET_REGEX.replace_all(&scrubbed, |caps: &regex::Captures| {
            format!("{} [REDACTED_SECRET]", &caps[1])
        }).to_string();

        scrubbed
    }
}
