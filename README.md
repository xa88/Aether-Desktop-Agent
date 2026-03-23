# 🛰️ Aether Desktop Agent (ADA)

> **The first distributed, visually-aware, and self-evolving autonomous OS controller — engineered across 25 development phases.**

[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-lightgrey.svg)](https://github.com/xa88/Aether-Desktop-Agent/releases)
[![Developer: xa88](https://img.shields.io/badge/Developer-xa88-purple.svg)](https://github.com/xa88)

---

## 🌟 What is ADA?

**Aether Desktop Agent** is not just an AI assistant — it is a complete autonomous OS intelligence layer. Built on a Rust-powered core orchestrator and an Electron + React frontend, ADA can **see your screen**, **control your mouse and keyboard**, **write and execute code**, **manage secrets**, **coordinate multiple AI agents in a swarm**, and **evolve its own logic over time**.

**Over 25 development phases** of rigorous engineering have produced a system capable of running end-to-end multi-agent workflows entirely autonomously.

---

## 🚀 Feature Breakdown: Phase 1–22

| Phase | Feature |
|-------|---------|
| **1** | Core Rust Workspace — Tool API, Orchestrator, Executor, Policy Engine |
| **2** | File System Adapter — secure atomic writes, quarantine, path normalization |
| **3** | Shell Adapter — sandboxed command execution with stdout/stderr streaming |
| **4** | Git Adapter — branch, commit, diff, blame via native libgit2 bindings |
| **5** | Web Search Adapter — real-time search via Tavily/Serper APIs |
| **6** | Electron Frontend — React UI with Monaco Editor, XTerm.js, Chat, IDE |
| **7** | Dual-Model Orchestration — Director (GPT-4o) + Worker (Phi-3/Llama-3) |
| **8** | Audit & Replay Engine — complete JSONL log of every autonomous action |
| **9** | Token-Aware Context Compression — auto-trim long contexts for efficiency |
| **10** | Hierarchical Planning — Intent taxonomy, template engine, plan splitting |
| **11** | Host Integration Security — path guards, secret scanner, symlink protection |
| **12** | Identity Vault — enterprise secret management with Windows system keyring |
| **13** | Plugin System — Open VSX Marketplace (30,000+ VSCode extensions), sandboxed |
| **14** | Voice-to-Text — OS-level microphone capture → text via Rust audio pipeline |
| **15** | Swarm Orchestration — Director-Worker agent topology with real-time HUD |
| **16** | Fix-it Engine — automated playbook recovery on task failure |
| **17** | Distributed Cluster Control — P2P node discovery for multi-machine swarms |
| **18** | Perpetual Memory — semantic vector storage for long-term plan recall |
| **19** | Browser Control — autonomous Playwright-based web navigation and interaction |
| **20** | Multimodal Screen Perception — OS-level screenshot + AI-based UI element detection |
| **21** | OS Mouse/Keyboard Control — hardware-level simulation via Enigo for full desktop automation |
| **22** | Self-Evolving Engine — reads audit history, detects chronic failures, synthesizes new playbooks autonomously |

---

## 🖥️ Installation (Windows)

1. Download [`ADA-Installer.exe`](https://github.com/xa88/Aether-Desktop-Agent/releases)
2. Run the installer — choose your installation directory
3. (Optional) Select **Install Support Libraries** to auto-configure Node.js and Rust environments
4. Launch **Aether Desktop Agent** from your Desktop shortcut

---

## ⚙️ Configuration

On first launch, navigate to the **Settings** tab to configure:
- **API Key** and **Base URL** for your LLM provider (OpenAI, local LM Studio, etc.)
- **Director Model** for high-level reasoning (recommended: `gpt-4o`)
- **Worker Model** for fast execution (recommended: `phi-3-mini` or `llama-3-8b`)

---

## 🔒 License

This project is licensed under the **GNU General Public License v3.0 (GPLv3)**.

You are free to use, study, and share this software. Any derivative work **must also be open-source** under GPLv3. Commercial proprietary forks are **not permitted**.

Copyright © 2026 [xa88](https://github.com/xa88). All rights reserved.
