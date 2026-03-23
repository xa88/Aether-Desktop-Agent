# ADA Artifacts & Evidence Bundle Specification

This specification dictates the structure and semantic meaning of an ADA Run Bundle. True autonomy necessitates reproducible tracing; each execution round exports a guaranteed, immutable snapshot of what the agent observed, planned, and produced.

## The Evidence Bundle (`.zip`)
At the end of every agent run, a bundle is exported to `runs/bundles/<run_id>.zip`.

```text
runs/bundles/<run_id>.zip
├── audit.jsonl             # High-fidelity event log (with `is_cached`, `is_self_heal` flags)
├── metrics_summary.json    # Timing, token usage, and cache-hit metrics
├── fingerprint.json        # OS/Env state (Rustc, Python, Node versions, arch)
└── artifacts.json          # Verifiable record of what files the agent produced
```

## `artifacts.json` Schema
The agent is capable of producing binaries, installers, and reports. Each produced artifact MUST have an entry in `artifacts.json` containing a SHA-256 fingerprint.

```json
{
  "run_id": "urn:uuid:f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "generated_at": "2026-03-24T10:20:30Z",
  "artifacts": [
    {
      "id": "A15A",
      "path": "/absolute/path/to/desktop/dist/Aether-Setup.exe",
      "size_bytes": 104857600,
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "producer_recipe": "electron_build.yaml"
    }
  ]
}
```

## `fingerprint.json` Schema
Captured at startup by the host engine.

```json
{
  "os": "Windows 11",
  "arch": "x86_64",
  "build_toolchains": {
    "rustc": "1.80.0",
    "node": "v22.0.0"
  }
}
```
