//! Verification tests for Phase 11: Host Integration

#[cfg(test)]
mod tests {
    use ada_policy::guard::{PolicyGuard, default_blocked_patterns};
    use ada_policy::secrets::SecretScanner;
    use std::path::PathBuf;

    #[test]
    fn test_path_guard_escape() {
        let guard = PolicyGuard::new(vec![".".into()], vec![]);
        
        // Allowed path
        assert!(guard.check_path("Cargo.toml").is_ok());
        
        // Escape attempt
        assert!(guard.check_path("../outside.txt").is_err());
        
        // Symlink/normalization trick
        assert!(guard.check_path("./core/../Cargo.toml").is_ok());
    }

    #[test]
    fn test_command_block() {
        let guard = PolicyGuard::new(vec![], default_blocked_patterns());
        
        assert!(guard.check_cmd("ls -la").is_ok());
        assert!(guard.check_cmd("rm -rf /").is_err());
        assert!(guard.check_cmd("powershell.exe -Command Stop-Service").is_err());
    }

    #[test]
    fn test_secret_scrubbing() {
        let scanner = SecretScanner::new();
        let text = "My key is sk-12345678901234567890 and password=mysecretpassword123";
        let redacted = scanner.redact(text);
        
        assert!(redacted.contains("[REDACTED:api_key]"));
        assert!(redacted.contains("[REDACTED:generic_secret]"));
        assert!(!redacted.contains("sk-123"));
        assert!(!redacted.contains("mysecretpassword"));
    }
}
