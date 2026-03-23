# ADA Security Checklist & Baseline

This document dictates the non-negotiable security requirements for the ADA ecosystem. All continuous integration pipelines enforce these rules automatically.

## 1. Electron Hardening (Frontend)
The Web UI acts as an untrusted view layer communicating with the Rust Core via IPC. As such:
- `nodeIntegration: false` MUST ALWAYS be set in all `BrowserWindow` configurations. The renderer has zero access to standard Node APIs or the filesystem.
- `contextIsolation: true` MUST ALWAYS be set to prevent prototype pollution and preload leakage.
- `sandbox: true` MUST ALWAYS be set to ensure Chromium's multi-process OS-level sandboxing restricts the renderer.
- `Content-Security-Policy (CSP)`: Must forbid inline scripts where possible and prevent loading unauthorized external media or remote iframes.

## 2. Supply Chain Scanning (SBOM)
- The project utilizes `cargo audit` to scan the Rust core dependencies against the generic advisory database.
- The project utilizes `npm audit` to scan Electron and UI dependencies.
- A Software Bill of Materials (SBOM) `sbom.json` is packaged and appended to all local artifacts. Any vulnerability above `High` causes the build to fail-fast.
