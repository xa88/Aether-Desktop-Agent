use ada_policy::profile::EnterpriseProfile;

#[test]
fn test_enterprise_profile_network_bounds() {
    let profile = EnterpriseProfile {
        allowed_domains: vec!["corp.local".to_string(), "api.github.com".to_string()],
        restricted_paths: vec!["/tmp/ada".to_string()],
        max_risk_tier: 2,
    };

    assert!(profile.is_network_allowed("api.github.com"));
    assert!(profile.is_network_allowed("auth.corp.local"));
    assert!(profile.is_network_allowed("corp.local"));

    assert!(!profile.is_network_allowed("google.com"));
    assert!(!profile.is_network_allowed("malicious.biz"));
}

#[test]
fn test_enterprise_profile_fs_bounds() {
    let profile = EnterpriseProfile {
        allowed_domains: vec![],
        restricted_paths: vec!["/home/user/workspace".to_string()],
        max_risk_tier: 1,
    };

    assert!(profile.is_path_allowed("/home/user/workspace/project/main.rs"));
    assert!(profile.is_path_allowed("/home/user/workspace"));

    // Guard rejects external path
    assert!(!profile.is_path_allowed("/etc/passwd"));
    assert!(!profile.is_path_allowed("/var/log/syslog"));
}

#[test]
fn test_enterprise_profile_empty_allows_all() {
    let profile = EnterpriseProfile::default();
    
    assert!(profile.is_network_allowed("google.com"));
    assert!(profile.is_path_allowed("/etc/shadow")); // If empty, guard relies on standard executor sandboxing
}
