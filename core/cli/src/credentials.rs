//! Secure credential persistence using system keyring.

use keyring::Entry;
use anyhow::{Result, anyhow};

#[allow(dead_code)]
pub struct CredentialManager {
    service: String,
}

#[allow(dead_code)]
impl CredentialManager {
    pub fn new() -> Self {
        Self {
            service: "ada-system".to_string(),
        }
    }

    pub fn set_secret(&self, key: &str, value: &str) -> Result<()> {
        let entry = Entry::new(&self.service, key).map_err(|e| anyhow!("Keyring error: {}", e))?;
        entry.set_password(value).map_err(|e| anyhow!("Failed to set secret: {}", e))
    }

    pub fn get_secret(&self, key: &str) -> Result<String> {
        let entry = Entry::new(&self.service, key).map_err(|e| anyhow!("Keyring error: {}", e))?;
        entry.get_password().map_err(|e| anyhow!("Failed to get secret for {}: {}", key, e))
    }

    pub fn delete_secret(&self, key: &str) -> Result<()> {
        let entry = Entry::new(&self.service, key).map_err(|e| anyhow!("Keyring error: {}", e))?;
        entry.delete_credential().map_err(|e| anyhow!("Failed to delete secret: {}", e))
    }
}

#[allow(dead_code)]
pub fn scrub_secrets(text: &str) -> String {
    ada_policy::secrets::SecretScanner::scrub(text)
}
