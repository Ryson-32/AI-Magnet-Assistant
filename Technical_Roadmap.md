# MagnetLink Optimizer Pro 技术路线图

本文档为 "MagnetLink Optimizer Pro" 智能磁力链接搜索与优化软件提供详细的技术规划，涵盖技术选型、系统架构和开发里程碑。

---

## 1. 技术选型与论证

为了实现跨平台、高性能和现代化的用户体验，我们推荐以下技术栈：

| 类别 | 技术 | 论证 |
| :--- | :--- | :--- |
| **桌面应用框架** | **Tauri** | 性能卓越，打包体积小，安全性高。使用 Rust 作为后端，完美契合项目对高性能和并发处理的需求，是替代 Electron 的现代化方案。 |
| **前端框架** | **Vue 3** | 采用组合式 API，开发体验优秀，代码组织清晰。其生态系统成熟，与 Tauri 集成良好，能快速构建现代化、响应式的用户界面。 |
| **后端语言** | **Rust** | 内存安全、无畏并发。其高性能特性是实现“搜索耗时 < 10秒”和多线程搜索引擎核心的关键保障。 |
| **LLM 服务接口** | **可配置接口 (Ollama + OpenAI/Gemini)** | 提供双重选择：**Ollama** 用于部署本地模型（如 Llama 3），实现低成本、高隐私的广告识别；同时预留 **OpenAI/Gemini** 等云端 API 接口，用于高质量的结果富化。 |
| **数据持久化** | **SQLite** | 轻量级、嵌入式、无服务器的数据库。非常适合存储历史记录、收藏和自定义规则等结构化数据，无需额外配置。 |
| **安全存储** | **keyring-rs** | 一个跨平台的 Rust 库，用于访问操作系统的安全凭证管理器（如 Windows Credential Manager, macOS Keychain），确保 API Key 等敏感信息的安全。 |

---

## 2. 系统架构设计

系统将采用模块化的分层架构，确保高内聚、低耦合，便于扩展和维护。

### 2.1 高层架构图 (Mermaid)

```mermaid
graph TD
    subgraph GUI Layer (Tauri + Vue 3)
        A[用户界面] --> B{状态管理 (Pinia)};
        A --> C{交互逻辑};
    end

    subgraph Core Service Layer (Rust)
        D[API 接口 (Tauri Commands)];
        E[Search Core];
        F[Filter Engine];
        G[LLM Service];
        H[Preview Service];
    end

    subgraph Data Persistence Layer (Rust)
        I[Database Module (SQLite)];
        J[Secure Storage (keyring)];
    end

    C -- 发起搜索/操作 --> D;
    D -- 调用 --> E;
    D -- 调用 --> F;
    D -- 调用 --> G;
    D -- 调用 --> H;
    D -- 调用 --> I;
    D -- 调用 --> J;

    E -- 原始结果 --> F;
    F -- 调用评估 --> G;
    H -- 调用 --> E;
    G -- 结果返回 --> F;
    F -- 最终结果 --> D;
    D -- 更新UI --> A;
```

### 2.2 模块职责描述

*   **GUI Layer:**
    *   **用户界面 (Vue 3):** 负责所有可视化元素的渲染，包括搜索框、结果卡片、设置页面等。支持深/浅色模式切换。
    *   **状态管理 (Pinia):** 集中管理应用状态，如搜索结果、用户设置、加载状态等。
    *   **交互逻辑:** 处理用户输入，并通过 Tauri 的 `invoke` API 调用后端 Rust 命令。

*   **Core Service Layer (Rust):**
    *   **API 接口:** 定义供前端调用的所有 Rust 函数，作为前后端通信的桥梁。
    *   **Search Core:** 核心搜索模块。管理一个可扩展的搜索引擎插件列表（基于 `SearchProvider` trait），并使用多线程并发执行搜索任务。
    *   **Filter Engine:** 筛选引擎。接收原始搜索结果，执行双轨并行筛选：1) 基于用户自定义规则（如文件大小、特定标记）进行快速过滤；2) 调用 `LLM Service` 对标题和描述进行广告评估。
    *   **LLM Service:** 抽象的语言模型服务。提供统一接口，内部可切换调用本地 Ollama 或云端 API。
    *   **Preview Service:** 内容预览服务。负责解析磁力链接指向的种子文件或压缩包，提取文件列表和元数据。

*   **Data Persistence Layer (Rust):**
    *   **Database Module:** 封装 `rusqlite` 库，提供对 SQLite 数据库的增删改查操作，用于管理历史、收藏和规则。
    *   **Secure Storage:** 封装 `keyring-rs` 库，安全地存取用户提供的 API Key 等敏感信息。

---

## 3. 开发里程碑

项目将分阶段进行，确保每个阶段都有明确的可交付成果。

*   **里程碑 1: 核心后端与 CLI (MVP)**
    *   **目标:** 验证核心搜索与筛选逻辑。
    *   **任务:**
        1.  实现 `Search Core` 模块，集成首个搜索引擎 `clmclm.com`。
        2.  实现 `Filter Engine` 的基础规则筛选功能。
        3.  创建一个简单的命令行界面 (CLI) 用于测试和演示核心功能。

*   **里程碑 2: 基础 GUI 与集成**
    *   **目标:** 构建基本的用户界面并与后端连接。
    *   **任务:**
        1.  搭建 Tauri + Vue 3 项目框架。
        2.  实现主搜索界面、结果卡片式布局。
        3.  通过 Tauri `invoke` API 将 GUI 与后端搜索功能连接。

*   **Milestone 3: LLM Integration**
    *   **目标:** 设计并实现一个健壮、可扩展的LLM集成方案，用于智能筛选和结果富化。
    *   **任务:**
        1.  **API接口设计 (`llm_service.rs`):**
            *   创建新的 `aether-magnet-ui/src-tauri/src/llm_service.rs` 文件。
            *   定义 `LlmClient` trait，包含 `evaluate_ad` 和 `enrich_result` 两个核心异步函数。
            *   `evaluate_ad`: 输入为 `&str` (标题)，输出为 `Result<AdEvaluationResponse>`，其中包含 `ad_score: f32`。
            *   `enrich_result`: 输入为 `&str` (标题) 和 `Option<&str>` (描述)，输出为 `Result<EnrichmentResponse>`，包含 `tags: Vec<String>` 和 `content_type: String`。
        2.  **数据流与集成策略:**
            *   **更新数据流:**
                1.  `searcher.rs` 获取原始搜索结果列表。
                2.  `filter.rs` 接收原始结果，首先执行基础规则过滤（关键词、文件大小等）。
                3.  对于通过初步过滤的结果，`filter.rs` 将并发调用 `llm_service::evaluate_ad` 函数，为每个结果计算广告分数。
                4.  `filter.rs` 根据广告分数阈值（可配置）进行二次筛选。
                5.  筛选后的优质结果返回给前端。
                6.  前端在展示结果卡片时，可以按需（例如，用户点击“智能分析”按钮）调用一个新的Tauri指令，该指令背后是 `llm_service::enrich_result` 函数，用于获取智能标签并更新UI。
            *   **修改 `filter.rs`:**
                *   引入 `llm_service` 模块。
                *   在 `filter_results` 函数中增加调用 `evaluate_ad` 的逻辑。
        3.  **技术选型建议:**
            *   **HTTP客户端:** 继续使用 `reqwest` 库与LLM API进行异步HTTP通信，其稳定性和功能丰富性足以满足需求。
            *   **安全存储:** 推荐使用 `tauri-plugin-store` 或 `keyring-rs` 来安全地存储和管理LLM服务的API Key。`tauri-plugin-store` 对于Tauri应用更易于集成。

*   **里程碑 4: 个性化与高级功能**
    *   **目标:** 完善用户体验和核心功能。
    *   **任务:**
        1.  使用 SQLite 实现历史记录、收藏夹功能。
        2.  实现用户自定义筛选规则的界面与存储。
        3.  开发内容预览功能（压缩包文件列表、视频元数据）。

*   **里程碑 5: 优化、测试与发布**
    *   **目标:** 确保软件质量，准备首次发布。
    *   **任务:**
        1.  进行全面的性能分析与优化。
        2.  完善错误处理和日志记录。
        3.  加固敏感信息存储的安全性。
        4.  编写单元测试和集成测试。
        5.  打包生成各平台（Windows, macOS, Linux）的安装程序并发布 v1.0.0。