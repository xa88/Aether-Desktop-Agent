# 🛰️ Aether Desktop Agent (ADA)

**A next-generation autonomous desktop AI agent — built to perceive, control, and automate your entire OS environment.**

[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-lightgrey.svg)](https://github.com/xa88/Aether-Desktop-Agent/releases)
[![Developer: xa88](https://img.shields.io/badge/Developer-xa88-purple.svg)](https://github.com/xa88)

> 🇨🇳 [查看中文介绍 (Chinese README)](./README_ZH.md)

---

## What is ADA?

**Aether Desktop Agent** is a standalone AI-powered autonomous agent that runs directly on your Windows desktop. Unlike cloud-based copilots, ADA works locally, coordinates multiple AI models, and can control your entire OS — from file operations and code editing to browser automation and visual screen control.

ADA combines a high-performance **Rust core** with a **beautiful React + Electron UI** to provide a seamless experience for developers, power users, and AI enthusiasts.

---

## ✨ Key Features

### 🧠 Intelligent Planning & Execution
- **Dual AI Model Architecture** — Separate "Director" model for planning and "Worker" model for fast execution
- **Hierarchical Task Planning** — Breaks complex goals into structured sub-tasks automatically
- **Automated Error Recovery** — Detects failures and retries with alternative strategies

### 🖥️ Full Desktop Control
- **Screen Vision** — Captures and visually analyzes your screen to identify UI elements
- **Mouse & Keyboard Control** — Simulates hardware-level input for complete desktop automation
- **Browser Automation** — Navigates websites, fills forms, and extracts information autonomously

### 💻 Integrated Development Environment
- **Built-in Monaco Code Editor** — Full syntax highlighting and multi-language support
- **Sandboxed Code Execution** — Run scripts safely in an isolated execution environment
- **Integrated Terminal** — Real-time command execution with streaming output

### 🤝 Multi-Agent Swarm
- **Agent Orchestration** — Coordinate multiple specialized AI agents in parallel
- **Real-time Swarm HUD** — Visual dashboard showing all active agents and task progress
- **Director/Worker Protocol** — Intelligent routing between high-capability and fast models

### 🔒 Security & Privacy
- **Identity Vault** — Enterprise-grade secret management with Windows system keyring integration
- **Audit Log** — Full timeline of every autonomous action for complete transparency
- **Local-First** — All processing happens on your machine; no data sent to external servers

### 🧩 Extensibility
- **Plugin Marketplace** — Compatible with 30,000+ VSCode extensions
- **Voice Control** — Microphone capture and voice-to-text for hands-free operation
- **Perpetual Memory** — Learns from past operations to improve future performance
- **Self-Evolution** — Analyzes its own performance and generates improved strategies

---

## 📦 Installation

1. Download the latest [`ADA-Installer.exe`](https://github.com/xa88/Aether-Desktop-Agent/releases)
2. Run the installer and select your installation directory
3. Optionally enable **Install Support Libraries** to auto-configure required environments
4. Launch **Aether Desktop Agent** from your Desktop shortcut

---

## ⚙️ First Launch Setup

On first launch, open **Settings** to configure:
- Your **LLM Provider** (OpenAI, Anthropic, or any OpenAI-compatible local server)
- **Planning Model** — for deep reasoning tasks (e.g., GPT-4o)
- **Execution Model** — for fast responses (e.g., GPT-4o-mini, Phi-3)

---

## 🔒 License

This project is licensed under the **GNU General Public License v3.0 (GPLv3)**.

Free to use, study, and share. Any derivative work **must also remain open-source under GPLv3**. Commercial proprietary forks are **not permitted**.

Copyright © 2026 [xa88](https://github.com/xa88). All rights reserved.
