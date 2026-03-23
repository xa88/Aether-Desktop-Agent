# Open VSX & Plugin Compatibility Guide

ADA (Aether Desktop Agent) implements a **Sandboxed Plugin Architecture** designed for security and cross-platform reliability.

## Open VSX Integration
ADA supports installing extensions from the **Open VSX Registry** (the open-source alternative to the VS Code Marketplace). 

### Compatibility Matrix
| Extension Type | Compatibility | Notes |
|----------------|---------------|-------|
| Language Servers | **Full** | Via LSP (Language Server Protocol) integration in ADA IDE. |
| UI Decorations | **Partial** | ADA uses a custom React renderer; most VS Code CSS/Theming requires mapping. |
| Tool Adapters | **Full** | ADA can wrap VS Code command extensions into its hierarchical planner. |

## Antigravity Compatibility
Applications like **Antigravity** (IDE-integrated AI panels) are first-class citizens in ADA.
- **IPC Bridge**: ADA provides an internal IPC bridge that mimics the VS Code extension API, allowing Antigravity to run with minimal modification.
- **Shared Context**: Antigravity can consume ADA's unified `ContextBundle`, providing it with the agent's current "Thought Process" and audit results.

## Security Model
Unlike traditional IDEs, ADA enforces **Capability-Based Permissions** for all plugins:
1. `fs_read / fs_write`: Restricted to the workspace path.
2. `shell_exec`: Only allowed within the Docker Sandbox.
3. `network`: Requires explicit user approval per domain.

## Implementation Details
The `PluginManager` in ADA core orchestrates these extensions using a specialized **WASM-based Host** for maximum isolation and performance.
