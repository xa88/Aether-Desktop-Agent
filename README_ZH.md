# 🛰️ Aether Desktop Agent（ADA）

> **首个分布式、视觉感知、自我进化的自主操作系统控制器 — 历经 25 个开发阶段精心打造。**

[![许可证: GPLv3](https://img.shields.io/badge/许可证-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![平台: Windows](https://img.shields.io/badge/平台-Windows-lightgrey.svg)](https://github.com/xa88/Aether-Desktop-Agent/releases)
[![开发者: xa88](https://img.shields.io/badge/开发者-xa88-purple.svg)](https://github.com/xa88)

---

## 🌟 ADA 是什么？

**Aether Desktop Agent** 不仅仅是一个 AI 助手——它是一个完整的自主操作系统智能层。基于 Rust 驱动的核心编排器与 Electron + React 前端构建，ADA 能够**看见您的屏幕**、**控制鼠标和键盘**、**编写并执行代码**、**管理加密密钥**、**协调多个 AI 智能体群集**，以及**自主进化其逻辑**。

**历经 25 个开发阶段**的严格工程实践，造就了一个能够完全自主运行端到端多智能体工作流的系统。

---

## 🚀 功能详解：第 1–22 阶段

| 阶段 | 功能 |
|------|------|
| **1** | Rust 核心工作区 — 工具 API、编排器、执行器、策略引擎 |
| **2** | 文件系统适配器 — 安全原子写入、隔离区、路径规范化 |
| **3** | Shell 适配器 — 沙箱化命令执行，实时 stdout/stderr 流 |
| **4** | Git 适配器 — 通过 libgit2 原生绑定实现分支、提交、差异、追溯 |
| **5** | 网络搜索适配器 — 通过 Tavily/Serper API 实时搜索 |
| **6** | Electron 前端 — 含 Monaco 编辑器、XTerm.js、聊天、IDE 的 React UI |
| **7** | 双模型编排 — 总监模型（GPT-4o）+ 工人模型（Phi-3/Llama-3）|
| **8** | 审计与回放引擎 — 完整 JSONL 格式每次自主操作日志 |
| **9** | 令牌感知上下文压缩 — 自动裁剪长上下文以提高效率 |
| **10** | 层级规划 — 意图分类法、模板引擎、计划拆分 |
| **11** | 主机集成安全 — 路径守卫、密钥扫描器、符号链接保护 |
| **12** | 身份库 — 使用 Windows 系统密钥环的企业级密钥管理 |
| **13** | 插件系统 — Open VSX 市场（30,000+ VSCode 扩展），沙箱化运行 |
| **14** | 语音转文字 — OS 级麦克风采集 → 通过 Rust 音频管道转文本 |
| **15** | 群集编排 — 总监-工人智能体拓扑，实时 HUD 显示 |
| **16** | 修复引擎 — 任务失败时的自动剧本恢复 |
| **17** | 分布式集群控制 — 用于多机群集的 P2P 节点发现 |
| **18** | 永久记忆 — 用于长期计划回忆的语义向量存储 |
| **19** | 浏览器控制 — 基于 Playwright 的自主网页导航和交互 |
| **20** | 多模态屏幕感知 — OS 级截图 + 基于 AI 的 UI 元素检测 |
| **21** | OS 鼠标/键盘控制 — 通过 Enigo 进行硬件级模拟，实现完整桌面自动化 |
| **22** | 自我进化引擎 — 读取审计历史，检测慢性故障，自主合成新剧本 |

---

## 🖥️ 安装（Windows）

1. 下载 [`ADA-Installer.exe`](https://github.com/xa88/Aether-Desktop-Agent/releases)
2. 运行安装程序 — 选择您的安装目录
3. （可选）选择**安装支持库**，自动配置 Node.js 和 Rust 环境
4. 从桌面快捷方式启动 **Aether Desktop Agent**

---

## ⚙️ 配置

首次启动时，导航至**设置**选项卡进行配置：
- **API 密钥**和 LLM 提供商的**基础 URL**（OpenAI、本地 LM Studio 等）
- 高级推理的**总监模型**（推荐：`gpt-4o`）
- 快速执行的**工人模型**（推荐：`phi-3-mini` 或 `llama-3-8b`）

---

## 🔒 许可证

本项目采用 **GNU 通用公共许可证 v3.0（GPLv3）**授权。

您可以自由使用、学习和分享本软件。任何衍生作品**必须同样在 GPLv3 下开源**。**不允许**商业专有分支。

版权所有 © 2026 [xa88](https://github.com/xa88)。保留所有权利。
