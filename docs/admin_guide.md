# ADA Desktop - IT Administrator Hardening Runbook

This compendium outlines the deep internal Security configurations explicitly enforcing Least-Privileged Access across ADA Orchestrator runtimes. 

## 1. Plugin Signatures & Validation (Ed25519)
ADA plugins (`.aether` bundles) must be cryptographically signed via `ada-plugins` signature suite. To mandate only authorized modules run across your fleet:
- Distribute your organization's `public_key.pem` to `~/.ada/keys/trusted/`.
- During compilation/CI of an internal App, execute the `ada-cli sign-plugin --private-key private.pem my-plugin.zip`.

## 2. Personal Identifiable Info (PII) Redaction & Audit Pruning
By default, the `ada-audit` backend pipes raw stdout into `.jsonl` runs. ADA enforces two hard limits explicitly mapping data loss bounds.
- **Auto-Redaction**: Every serialized struct passes through `./core/redaction/`. IT can enforce static expressions filtering Keys, OAuth Tokens, and Credit Card ranges. 
- **Auto-Purging**: By explicitly adjusting `StoragePolicy::Capacity(1024 * 50 /* 50MB */)`, older un-compressed audit telemetry is silently pruned chronologically reducing endpoint bloat.

## 3. Concurrency Protection & Memory Bounds
Without limits, an erratic local Playbook orchestrating `find / -type f` loops could exhaust SSD locks entirely.
Within `profile.yaml`, Administrators lock concurrent boundaries directly:
- **`max_output_kb`**: The JSON memory truncate footprint. If set to 500, strings inherently truncate to `[TRUNCATED_BY_BUDGET_ENGINE]`.
- **`max_wall_time_s`**: The maximum seconds execution can block natively.
- By binding these features, tasks gracefully return `PARTIAL` rather than silently locking endpoints.

## 4. Cloud Isolation Profiles
Any endpoint generating high-risk secrets should configure `cloud_enabled: false` natively within their domain group policies. ADA gracefully halts upstream context sharing, leaning exclusively unto Local Intent taxonomy definitions and `ada-cli` static YAML plan ingestions.
