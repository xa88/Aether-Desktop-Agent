# ADA Remote Worker Protocol (Draft)

This document dictates the HTTP/REST definitions orchestrating high-risk bounds execution offloaded to isolated distributed VM/Container servers inside a DMZ network.

## 1. Environment Requirements
- A server hosting `ada-worker` running standard Axum endpoints.
- All requests map generically without relying on sticky sessions.
- Transport MUST be encrypted via standard mTLS (Placeholder).

## 2. Endpoints API

### Submit Job
**`POST /api/v1/jobs`**
Submits a generated un-trusted `.yaml` plan payload off to the remote server.
*Request*:
```json
{
  "run_id": "uuid-v4",
  "plan_yaml": "..."
}
```
*Response*: `202 Accepted` | `400 Bad Request`

### Poll Status
**`GET /api/v1/jobs/:run_id/status`**
Queries the remote proxy for live completion.
*Response*:
```json
{
  "run_id": "uuid",
  "status": "Running | Completed | Failed",
  "steps_ok": 12,
  "steps_fail": 0
}
```

### Fetch Evidence Bundle
**`GET /api/v1/jobs/:run_id/bundle`**
Downloads the forensically signed `ada-audit` `.zip` footprint containing LLM prompts, filesystem logs, and state captures executed externally.
*Response*: `Content-Type: application/zip` - octet stream.
