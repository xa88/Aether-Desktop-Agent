use ada_plugins::*;
use ada_tool_api::*;
use ada_tool_api::router::ToolRouter;
use serde_json::json;

#[test]
fn test_plugin_manifest_validation() {
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        capabilities: PluginCapabilities {
            tools: vec!["fs_read".to_string()],
            paths: vec!["/tmp".to_string()],
            network: "deny".to_string(),
            max_risk_tier: RiskTier::T1,
        },
        signature: None,
    };
    assert!(manifest.validate().is_ok());

    let invalid = PluginManifest {
        id: "".to_string(),
        signature: None,
        ..manifest.clone()
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_plugin_signature_verification() {
    use ed25519_dalek::{SigningKey, Signer};
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    let root_key_bytes: [u8; 32] = verifying_key.to_bytes();

    let manifest = PluginManifest {
        id: "signed-plugin".to_string(),
        version: "1.0.0".to_string(),
        capabilities: PluginCapabilities {
            tools: vec!["fs_read".to_string()],
            paths: vec![],
            network: "deny".to_string(),
            max_risk_tier: RiskTier::T1,
        },
        signature: None,
    };

    let data = serde_json::to_vec(&manifest).unwrap();
    let signature = signing_key.sign(&data);
    let sig_hex = hex::encode(signature.to_bytes());

    let mut signed_manifest = manifest.clone();
    signed_manifest.signature = Some(sig_hex);

    let router = std::sync::Arc::new(ToolRouter::new());
    let mut host = PluginHost::new(router, Some(root_key_bytes));

    // Success case
    assert!(host.load_plugin(signed_manifest.clone(), vec![], vec![], None).is_ok());

    // Failure case: tampered manifest
    let mut tampered = signed_manifest.clone();
    tampered.version = "1.0.1".to_string();
    assert!(host.load_plugin(tampered, vec![], vec![], None).is_err());

    // Failure case: missing signature
    let mut missing_sig = signed_manifest.clone();
    missing_sig.signature = None;
    assert!(host.load_plugin(missing_sig, vec![], vec![], None).is_err());
}

#[test]
fn test_capability_guard_tools() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_str().unwrap().to_string();
    
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        capabilities: PluginCapabilities {
            tools: vec!["fs_read".to_string(), "shell_run".to_string()],
            paths: vec![root.clone()],
            network: "deny".to_string(),
            max_risk_tier: RiskTier::T1,
        },
        signature: None,
    };
    let guard = CapabilityGuard::new(manifest, vec![root.clone()], vec![]);

    // Allowed tool + Allowed path
    let test_file = temp.path().join("foo.txt");
    std::fs::write(&test_file, "hello").unwrap();
    
    let req_ok = ToolRequest::new("fs", "read", json!({"path": test_file.to_str().unwrap()}));
    assert!(guard.check_request(&req_ok).is_ok());

    // Allowed tool_action
    let req_ok_2 = ToolRequest::new("shell", "run", json!({"command": "ls"}));
    assert!(guard.check_request(&req_ok_2).is_ok());

    // Blocked tool
    let req_fail = ToolRequest::new("git", "commit", json!({"message": "hi"}));
    assert!(guard.check_request(&req_fail).is_err());
    
    // Blocked path (outside manifest roots)
    let req_bad_path = ToolRequest::new("fs", "read", json!({"path": "/secret/data.txt"}));
    assert!(guard.check_request(&req_bad_path).is_err());
}

#[tokio::test]
async fn test_rogue_plugin_simulation() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_str().unwrap().to_string();
    
    // Rogue plugin: claims only fs_read in /tmp, but tries to write or access /user/home
    let manifest = PluginManifest {
        id: "rogue-plugin".to_string(),
        version: "6.6.6".to_string(),
        capabilities: PluginCapabilities {
            tools: vec!["fs_read".to_string()],
            paths: vec![root.clone()],
            network: "deny".to_string(),
            max_risk_tier: RiskTier::T0,
        },
        signature: None,
    };
    
    let router = std::sync::Arc::new(ToolRouter::new());
    let mut host = PluginHost::new(router, None);
    host.load_plugin(manifest, vec![root.clone()], vec![], None).unwrap();
    
    // 1. Try unauthorized tool (fs_write)
    let mut req_write = ToolRequest::new("fs", "write", json!({"path": "test.txt", "content": "rogue"}));
    let res = host.dispatch_for_plugin("rogue-plugin", &mut req_write).await;
    assert!(!res.success);
    assert!(res.error.unwrap().message.contains("does not have capability"));
    
    // 2. Try unauthorized path
    let mut req_path = ToolRequest::new("fs", "read", json!({"path": "/etc/shadow"}));
    let res = host.dispatch_for_plugin("rogue-plugin", &mut req_path).await;
    assert!(!res.success);
    assert!(res.error.unwrap().message.contains("outside plugin manifest allowed paths"));
}

#[test]
fn test_capability_guard_risk_tier() {
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        capabilities: PluginCapabilities {
            tools: vec!["shell_run".to_string()],
            paths: vec![],
            network: "deny".to_string(),
            max_risk_tier: RiskTier::T0,
        },
        signature: None,
    };
    let guard = CapabilityGuard::new(manifest, vec![], vec![]);

    let mut req = ToolRequest::new("shell", "run", json!({"command": "ls"}));
    req.risk_tier = RiskTier::T1; // Exceeds T0
    assert!(guard.check_request(&req).is_err());
}
