# 决策日志
此文件用于记录项目中的关键架构和技术决策。
---
### 决策
[2025-07-06 13:42:53] - **技术栈选型**: 确定采用 Tauri + Vue 3 + Rust 作为核心技术栈。Tauri 提供高性能、小体积的跨平台能力；Vue 3 保证开发效率和现代 UI；Rust 满足性能和安全需求。此决策旨在平衡开发效率、应用性能和未来可扩展性。
---
### 代码实现 [核心后端MVP]
[2025-07-06 13:45:51] - 实现了 "MagnetLink Optimizer Pro" 项目核心后端的最小可行产品（MVP）。

**实现细节：**
- **项目初始化:** 手动创建了 Rust 二进制项目结构，位于 `magnetlink-optimizer-pro-core` 目录。
- **依赖管理:** 在 `Cargo.toml` 中添加了 `reqwest`, `scraper`, `tokio`, 和 `anyhow`。
- **模块化:**
    - `searcher.rs`: 实现了异步的 `search` 函数，用于从 `clmclm.com` 获取并解析 HTML 搜索结果。
    - `filter.rs`: 实现了 `filter_results` 函数，用于根据标题中的 `***REMOVED***.com@` 标记进行结果筛选。
    - `main.rs`: 构建了基础的命令行界面，接收搜索关键词，并协调搜索与筛选流程，最终打印结果。
- **错误处理:** 使用 `anyhow` 库进行统一的错误处理。
- **异步处理:** 使用 `tokio` 作为异步运行时。

**测试框架：**
[待定 - 将在下一步通过 `test-case-generator` 模式生成]

**测试结果：**
- 覆盖率：[待定]
- 通过率：[待定]
---
### 决策
[2025-07-06 14:28:17] - **LLM集成架构设计**: 确定了LLM集成的核心架构方案。
- **新建模块**: 创建 `llm_service.rs`，定义了 `LlmClient` trait，包含 `evaluate_ad` 和 `enrich_result` 函数，实现了服务层面的抽象。
- **数据流**: 确定了“搜索 -> 基础过滤 -> LLM广告评估 -> 返回前端 -> 按需富化”的异步数据流。
- **技术选型**: 确认使用 `reqwest` 作为HTTP客户端，并推荐 `tauri-plugin-store` 用于API Key的安全存储。
- **集成点**: 明确了 `filter.rs` 是调用广告评估功能的核心集成点。