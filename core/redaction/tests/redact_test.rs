use ada_redaction::Redactor;

#[test]
fn test_redaction_email_ip() {
    let payload = r#"{"user": "alice@example.com", "ip": "192.168.1.100", "status": "ok"}"#;
    let redacted = Redactor::redact(payload);
    
    assert!(!redacted.contains("alice@example.com"));
    assert!(redacted.contains("[REDACTED_EMAIL]"));
    
    assert!(!redacted.contains("192.168.1.100"));
    assert!(redacted.contains("[REDACTED_IP]"));
}

#[test]
fn test_redaction_generic_secrets() {
    let raw = "Authorization: Bearer my_super_secret_token_123; Key=some_other_long_secret_string";
    let redacted = Redactor::redact(raw);
    
    assert!(!redacted.contains("my_super_secret_token_123"));
    assert!(!redacted.contains("some_other_long_secret_string"));
    assert!(redacted.contains("Bearer [REDACTED_SECRET]"));
    assert!(redacted.contains("Key [REDACTED_SECRET]"));
}
