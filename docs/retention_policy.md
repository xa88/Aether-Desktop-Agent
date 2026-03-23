# ADA Data Retention & Privacy Policy

ADA operates with a strong Local-First, Privacy-First architecture. To guarantee this, the following automatic governance policies are strictly enforced at the Core Runtime level.

## 1. Primary Data Redaction
All execution logs (`audit.jsonl`), trace telemetry, and crash diagnostics pass through a secondary Regex scanner (`ada-redaction`) before ever being saved to the local disk or optionally synced up to an enterprise telemetry endpoint.
- **Scrubbing Targets**: Email Addresses, IPv4/IPv6 points, MAC Addresses, common Secret keys (AWS, Stripe, generic auth tokens).
- **Format**: All matches are aggressively replaced entirely with `[REDACTED]`.

## 2. Storage Boundaries & Auto-Cleanup
Extensive artifacts (Sandbox snapshots, LLM Prompt caches, UI build outputs, Logs) naturally bloat storage over long utilization periods.
- ADA institutes a strict background Job `cleanup_old_runs()` triggered actively by the Orchestrator runtime.
- **Default Hard Limitation**: No run evidence directory will persist beyond **7 Days**.
- **Default Quota**: Un-managed traces will actively block allocations if the local workspace exceeds `500MB`. 

Users are encouraged to manually snapshot important `run_report.md` entries if extended retention is critical mapping.
