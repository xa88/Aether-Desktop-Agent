//! Plugin signing and verification logic (Ed25519).

use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use std::convert::TryFrom;

pub struct PluginVerifier {
    root_key: VerifyingKey,
}

impl PluginVerifier {
    pub fn new(root_key_bytes: &[u8; 32]) -> Self {
        let root_key = VerifyingKey::from_bytes(root_key_bytes).expect("Invalid root public key");
        Self { root_key }
    }

    /// Verifies the signature of the provided manifest data.
    pub fn verify(&self, data: &[u8], signature_hex: &str) -> anyhow::Result<()> {
        let sig_bytes = hex::decode(signature_hex)
            .map_err(|_| anyhow::anyhow!("Invalid hex signature"))?;
        
        let signature = Signature::try_from(&sig_bytes[..])
            .map_err(|_| anyhow::anyhow!("Invalid signature format"))?;
            
        self.root_key.verify(data, &signature)
            .map_err(|e| anyhow::anyhow!("Signature verification failed: {}", e))
    }
}
