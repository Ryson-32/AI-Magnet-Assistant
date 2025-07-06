# 错误报告：`npm run tauri dev` 启动失败

## 1. 问题摘要

在尝试恢复UI驱动的LLM配置后，应用无法通过 `npm run tauri dev` 命令正常启动。尽管前端Vite服务器有时可以单独启动，但Tauri主进程无法与之正确连接，导致应用整体功能失效。核心症状表现为持续的端口占用错误和Tauri IPC通信失败。

## 2. 观察到的症状

1.  **端口占用错误**: 运行 `npm run tauri dev` 时，Vite开发服务器反复报告端口已被占用 (`Error: Port XXXX is already in use`)。即使在清理缓存并将端口号从 `1521` -> `1420` -> `1422` 之后，此问题依然存在。
2.  **Tauri IPC 失败**: 当应用窗口出现时（通常是在Tauri CLI仍在等待连接的情况下），任何需要与后端通信的操作（如保存设置）都会失败，并弹出错误：`TypeError: window.__TAURI_IPC__ is not a function`。这明确表示前端没有在Tauri的上下文中正确运行，无法调用后端命令。
3.  **Tauri CLI 卡死**: Tauri CLI 经常卡在 `Waiting for your frontend dev server to start on http://localhost:XXXX/...` 这一步，即使Vite服务器已经在另一个端口上成功启动。

## 3. 已执行的调试步骤

我们系统地进行了以下调试和修复尝试：

1.  **后端代码修复**:
    *   从 `aether-magnet-ui/src-tauri/Cargo.toml` 中完全移除了 `dotenvy` 依赖。
    *   重构了 `aether-magnet-ui/src-tauri/src/main.rs` 和 `aether-magnet-ui/src-tauri/src/llm_service.rs`，移除了所有对环境变量的直接读取，使 `analyze_resource` 和 `test_connection` 命令通过函数参数接收 `LlmConfig`。

2.  **前端功能恢复**:
    *   在 `aether-magnet-ui/src/App.vue` 中，使用 `tauri-plugin-store-api` 重新实现了配置的保存 (`saveLlmConfig`) 和加载 (`loadLlmConfig`) 功能，以替代之前无效的 `invoke` 调用。
    *   通过 `npm install tauri-plugin-store-api` 为前端项目添加了缺失的依赖。

3.  **端口和配置同步**:
    *   **尝试1 (自动端口)**: 修改 `vite.config.ts` 以允许Vite自动选择可用端口。这导致Vite与`tauri.conf.json`中的`devUrl`不同步，使Tauri CLI卡死。
    *   **尝试2 (固定端口)**: 将 `vite.config.ts` 和 `tauri.conf.json` 中的端口统一设置为 `1420`，然后是 `1422`，以确保两者配置一致。

4.  **深度清理**:
    *   删除了后端的 `aether-magnet-ui/src-tauri/target` 目录，以强制进行全新的Rust编译。
    *   删除了前端的 `aether-magnet-ui/node_modules` 目录和 `package-lock.json` 文件。
    *   执行了 `npm install` 以重新安装所有前端依赖。

5.  **进程终止尝试**:
    *   使用 `netstat -ano` 成功识别出占用端口的进程PID。
    *   由于您的终端是Bash环境，多次尝试使用 `taskkill` 和 PowerShell 的 `Stop-Process` 命令均告失败，无法通过命令行可靠地终止占用端口的进程。

## 4. 当前代码状态

*   **后端**: 已准备好接收来自前端的参数化配置，不依赖任何 `.env` 文件。
*   **前端**: UI功能完整，使用 `tauri-plugin-store` 进行设置持久化。
*   **配置文件**:
    *   `aether-magnet-ui/vite.config.ts`: 配置为在端口 `1422` 上以严格模式 (`strictPort: true`) 运行。
    *   `aether-magnet-ui/src-tauri/tauri.conf.json`: `devUrl` 已同步设置为 `http://localhost:1422`。

## 5. 最终推断

尽管错误信息表面上是“端口占用”，但我认为根本原因可能更深层。持续的失败（即使在更换端口和深度清理后）暗示这可能是一个与环境相关的问题，而不是简单的代码错误。

**核心假设**: 您的开发环境（可能是Windows上的Bash、特定的Node.js/npm版本或全局安装的某些工具）与Tauri CLI或Vite的进程管理方式存在冲突。当 `npm run tauri dev` 尝试启动其子进程 (`npm run dev`) 时，旧的Vite或Node.js进程没有被正确终止，导致新进程启动时发现端口已被“僵尸进程”占用。我们无法用脚本杀死这个进程，这使得问题陷入循环。

建议您从环境角度进行排查，例如：
*   使用Windows任务管理器手动查找并终止所有 `node.exe` 进程。
*   尝试在不同的终端（如原生的CMD或PowerShell）中运行 `npm run tauri dev`。
*   检查并更新Node.js、npm以及任何全局安装的Tauri工具链。