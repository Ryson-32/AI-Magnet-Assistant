# AI-Magnet-Assistant - Software Architecture Document

## 1. Overall Project Architecture

This project, **AI-Magnet-Assistant**, is a desktop application built on the **Tauri** framework. Tauri's core architecture allows for building the user interface with web frontend technologies (**Vue.js 3**) while leveraging a high-performance backend language (**Rust**) to handle core business logic.

This architectural pattern offers the following key features:

*   **High Performance and Low Resource Consumption**: The Rust backend ensures high efficiency and memory safety for computationally intensive tasks (such as concurrent searching and data processing). Tauri packages the frontend into a native Webview, resulting in a smaller application size and lower resource usage compared to solutions like Electron.
*   **Clear Frontend-Backend Separation**: The frontend code (in `src/`) and backend code (in `src-tauri/`) are physically and logically separated, with distinct responsibilities. The frontend focuses on UI/UX, while the backend concentrates on business logic, data persistence, and interaction with external services.
*   **Cross-Platform Capability**: Tauri supports packaging the application into native apps for Windows, macOS, and Linux, providing a cross-platform foundation for the project.
*   **Security**: Frontend-backend interaction is conducted through Tauri's `invoke` mechanism, a secure message-passing bridge that avoids the risks of directly exposing backend service ports.

**Core Data Flow**: The user initiates an action on the Vue.js-built frontend interface (e.g., clicking a search button) -> The frontend calls a command registered in the Rust backend via Tauri's `invoke` function -> The Rust backend executes the corresponding logic (e.g., concurrently accessing multiple search engines, calling an LLM API for analysis) -> Rust returns the processed results to the frontend -> The frontend receives the data and updates the UI.

---

## 2. Root Directory Structure

The project's root directory structure follows the standard layout for a Tauri project, ensuring code organization and maintainability. All core documents, including this architecture document, are stored in the `docs/` directory.

*   `.gitignore`: A list of files to be ignored by Git version control.
*   `docs/`: Stores all core project documents, such as the Developer Manual (`DEVELOPER_MANUAL.md`) and this Software Architecture Document (`ARCHITECTURE.md`).
*   `index.html`: The HTML entry point for the frontend application.
*   `LICENSE`: The project's open-source license file.
*   `package.json`: **Core frontend configuration file**. Defines the project name, version, npm package dependencies, and executable scripts.
*   `package-lock.json`: **Frontend dependency lock file**. Ensures consistency of dependency versions.
*   `public/`: Stores static assets that do not require compilation.
*   `README.md` & `README.zh-CN.md`: The project's readme files.
*   `run/`: A directory for executable scripts, such as automated environment setup scripts.
*   `src/`: **Frontend source code directory**.
*   `src-tauri/`: **Backend source code directory**.
*   `tsconfig.json` & `tsconfig.node.json`: TypeScript configuration files.
*   `vite.config.ts`: Configuration file for the Vite frontend build tool.

---

## 3. Frontend Architecture (`src/`)

The frontend code structure follows the typical organization of a Vue 3 project.

*   **`main.ts`**: The project's entry point, responsible for creating the Vue application instance and mounting it to the DOM.
*   **`App.vue`**: The root component, containing the router view and global components.
*   **`assets/`**: Stores static assets, such as SVG icons.
*   **`components/`**: Stores reusable Vue components, forming the core of the frontend.

### Key Component Design

*   **`HomePage.vue`**:
    *   **Responsibility**: The application's core functional page, responsible for the search feature.
    *   **UI**: Includes a search input box, filter options (AI filtering, page count selection), sorting controls, and a results display area.
    *   **State Management**: Shares search state via the `inject` and `provide` mechanism.
    *   **Core Flow**:
        1.  `search()`: Validates input, sets the search state, and **concurrently calls the backend** commands `search_clmclm_first` and `search_other_engines` via `invoke`.
        2.  **Progressive Loading**: First processes and displays results from the primary engine, then merges results from other engines.
        3.  `analyzeResults()`: If smart filtering is enabled, it calls `invoke('batch_analyze_resources', ...)` to send results to the backend for analysis in **parallel batches**, updating the UI with progress in real-time.
        4.  `sortResults()`: Sorts the results on the client-side based on the user-selected sorting rule.

*   **`SettingsPage.vue`**:
    *   **Responsibility**: Manages the application's configuration.
    *   **UI**: Provides a form to configure AI APIs (for HTML content extraction and content analysis).
    *   **Core Flow**:
        1.  `loadLlmConfig()`: Loads configuration from the backend via `invoke("get_llm_config")` when the component is mounted.
        2.  `saveLlmConfig()`: When the user saves, it persists the configuration to the backend via `invoke("update_llm_config", ...)`.
        3.  `test...Connection()`: Calls the backend's test connection commands to provide immediate feedback to the user.
        4.  `openConfigFolder()`: Calls Tauri's `opener` plugin to open the configuration file's location in the file manager.

---

## 4. Backend Architecture (`src-tauri/`)

The backend Rust code adopts a highly modular structure with clear responsibilities.

*   **`main.rs`**:
    *   **Responsibility**: The application's entry point and command routing center.
    *   **Design**:
        1.  **State Management**: In the `setup` hook, it initializes the application state via `app_state::init_app_state` and registers it as Tauri's managed state using `app.manage()`.
        2.  **Command Registration**: Uses the `tauri::generate_handler!` macro to register all `#[tauri::command]` functions as commands callable from the frontend, forming a clear API boundary.
        3.  **Core Commands**: Provides CRUD (Create, Read, Update, Delete) functionalities for search, batch analysis, favorites, search engines, etc., and calls `app_state::save_app_state` for persistence after modifications.

*   **`searcher.rs`**:
    *   **Responsibility**: Defines the abstraction and concrete implementations for the search functionality.
    *   **Design**:
        1.  **`SearchProvider` Trait**: Defines a common search engine interface (`search` method), following the Dependency Inversion Principle for easy extension.
        2.  **`ClmclmProvider`**: A concrete implementation of `SearchProvider` that uses `reqwest` and `scraper` to scrape data from a specific website.
        3.  **`GenericProvider`**: Another implementation of `SearchProvider` for handling custom search engines. It integrates `LlmClient` and, when an LLM is configured, sends the entire HTML to the AI for intelligent extraction, rather than relying on fixed CSS selectors.
        4.  **`SearchCore`**: Encapsulates multiple `SearchProvider`s and is responsible for coordinating concurrent searches.
        5.  **`create_ai_enhanced_search_core`**: Acts as a factory function to dynamically create `SearchCore` instances based on the configuration.

*   **`llm_service.rs`**:
    *   **Responsibility**: Encapsulates all logic for interacting with Large Language Models (LLMs).
    *   **Design**:
        1.  **`LlmClient` Trait**: Defines a unified interface for interacting with LLM services, allowing for future replacement or addition of new LLM providers.
        2.  **`GeminiClient`**: A concrete implementation of `LlmClient` responsible for communicating with the Google Gemini API.
        3.  **Prompt Engineering**: The core value of the module lies in meticulously designed prompts that ensure the stability and reliability of the AI's output through clear role-playing, strict JSON output formats, and task decomposition.
        4.  **Error Handling and Retries**: Includes automatic retry logic for failed API calls, enhancing system robustness.

### Inter-Module Relationships

`main.rs` acts as the coordinator, receiving frontend requests, fetching state from `app_state`, and calling `searcher.rs` to perform searches. The `GenericProvider` within `searcher.rs` will call `llm_service.rs` to interact with the LLM API when needed. This flow clearly demonstrates separation of concerns and module collaboration.

---

## 5. Frontend-Backend Interaction

Frontend-backend interaction is handled exclusively through Tauri's `invoke` API, which is a secure and efficient mechanism.

1.  **Frontend Call**: In a Vue component, use the `invoke` function, passing the command name (e.g., `"search_clmclm_first"`) and a parameter object.
2.  **Backend Processing**: Tauri receives the call, matches it to the corresponding `#[tauri::command]` function in `main.rs`, and deserializes the parameters into Rust types.
3.  **Data Return**: The return value of the Rust function is serialized to JSON and returned to the frontend via a `Promise`.
4.  **Frontend Reception**: The frontend's `invoke` call returns a `Promise`, which asynchronously receives the backend data via `await`.

**Example**:
```typescript
// Frontend (HomePage.vue)
const results = await invoke("search_clmclm_first", {
  keyword: keyword.value,
  maxPages: maxPages.value,
});
```
This call will trigger the execution of the Rust backend's `search_clmclm_first` function and return the result to the frontend's `results` variable.
---
# AI-Magnet-Assistant - 软件架构设计文档 (Software Architecture Document)

## 1. 项目整体架构

本项目 **AI-Magnet-Assistant** 是一个基于 **Tauri** 框架构建的桌面应用程序。Tauri 的核心架构允许使用 Web 前端技术（**Vue.js 3**）构建用户界面，同时利用高性能的后端语言（**Rust**）处理核心业务逻辑。

此架构模式具备以下关键特性：

*   **高性能与低资源占用**：Rust 后端确保计算密集型任务（如并发搜索、数据处理）的高效率和内存安全。Tauri 将前端打包为本地 Webview，相比于 Electron 等方案，最终的应用体积更小，资源占用更低。
*   **清晰的前后端分离**：前端代码（位于 `src/`）和后端代码（位于 `src-tauri/`）在物理上和逻辑上分离，职责分明。前端专注于 UI/UX，后端专注于业务逻辑、数据持久化和与外部服务的交互。
*   **跨平台能力**：Tauri 支持将应用打包为 Windows、macOS 和 Linux 的原生应用，为项目提供了跨平台基础。
*   **安全性**：前后端的交互通过 Tauri 的 `invoke` 机制进行，这是一个安全的消息传递桥梁，避免了直接暴露后端服务端口的风险。

**核心数据流**：用户在 Vue.js 构建的前端界面上发起操作（如点击搜索按钮） -> 前端通过 Tauri 的 `invoke` 函数调用一个在 Rust 后端注册的命令 -> Rust 后端执行相应逻辑（如并发访问多个搜索引擎、调用 LLM API 进行分析） -> Rust 将处理结果返回给前端 -> 前端接收数据并更新 UI。

---

## 2. 根目录结构

项目的根目录结构遵循 Tauri 项目的标准布局，确保了代码的组织性和可维护性。所有核心文档，包括本架构文档，均存放于 `docs/` 目录。

*   `.gitignore`: Git版本控制的忽略文件列表。
*   `docs/`: 存放项目所有核心文档，如开发者手册 (`DEVELOPER_MANUAL.md`) 和本架构设计文档 (`ARCHITECTURE.md`)。
*   `index.html`: 前端应用的HTML入口文件。
*   `LICENSE`: 项目的开源许可证文件。
*   `package.json`: **前端核心配置文件**。定义项目名称、版本、依赖的npm包及可执行脚本。
*   `package-lock.json`: **前端依赖锁定文件**。确保依赖版本的一致性。
*   `public/`: 存放无需编译的静态资源。
*   `README.md` & `README.zh-CN.md`: 项目说明书。
*   `run/`: 存放可执行脚本的目录，如自动化环境配置脚本。
*   `src/`: **前端源代码目录**。
*   `src-tauri/`: **后端源代码目录**。
*   `tsconfig.json` & `tsconfig.node.json`: TypeScript的配置文件。
*   `vite.config.ts`: 前端构建工具Vite的配置文件。

---

## 3. 前端架构 (`src/`)

前端代码结构遵循典型的 Vue 3 项目组织方式。

*   **`main.ts`**: 项目的入口文件，负责创建 Vue 应用实例并将其挂载到 DOM 上。
*   **`App.vue`**: 根组件，包含路由视图和全局性组件。
*   **`assets/`**: 存放静态资源，如 SVG 图标。
*   **`components/`**: 存放可复用的 Vue 组件，是前端的核心。

### 关键组件设计

*   **`HomePage.vue`**:
    *   **职责**: 应用的核心功能页面，负责搜索功能。
    *   **UI**: 包含搜索输入框、过滤选项（AI 过滤、页数选择）、排序控件和结果展示区域。
    *   **状态管理**: 通过 `inject` 和 `provide` 机制共享搜索状态。
    *   **核心流程**:
        1.  `search()`: 校验输入，设置搜索状态，并通过 `invoke` **并行调用后端**的 `search_clmclm_first` 和 `search_other_engines` 命令。
        2.  **渐进式加载**: 首先处理并展示主要引擎的结果，然后合并其他引擎的结果。
        3.  `analyzeResults()`: 若启用智能过滤，则调用 `invoke('batch_analyze_resources', ...)`，将结果**并行批量**发送至后端进行分析，并实时更新UI反馈进度。
        4.  `sortResults()`: 根据用户选择的排序规则对结果进行客户端排序。

*   **`SettingsPage.vue`**:
    *   **职责**: 负责应用的配置管理。
    *   **UI**: 提供表单以配置 AI API（HTML内容提取与内容分析）。
    *   **核心流程**:
        1.  `loadLlmConfig()`: 组件挂载时通过 `invoke("get_llm_config")` 从后端加载配置。
        2.  `saveLlmConfig()`: 用户保存时，通过 `invoke("update_llm_config", ...)` 将配置持久化到后端。
        3.  `test...Connection()`: 调用后端的测试连接命令，为用户提供即时反馈。
        4.  `openConfigFolder()`: 调用 Tauri 的 `opener` 插件，在文件管理器中打开配置文件所在位置。

---

## 4. 后端架构 (`src-tauri/`)

后端 Rust 代码采用高度模块化的结构，职责清晰。

*   **`main.rs`**:
    *   **职责**: 应用的入口点和命令路由中心。
    *   **设计**:
        1.  **状态管理**: 在 `setup` 钩子中，通过 `app_state::init_app_state` 初始化应用状态，并使用 `app.manage()` 将其注册为 Tauri 的托管状态。
        2.  **命令注册**: 使用 `tauri::generate_handler!` 宏将所有 `#[tauri::command]` 函数注册为可供前端调用的命令，形成清晰的 API 边界。
        3.  **核心命令**: 提供搜索、批量分析以及对收藏夹、搜索引擎等的增删改查（CRUD）功能，并在修改后调用 `app_state::save_app_state` 进行持久化。

*   **`searcher.rs`**:
    *   **职责**: 定义搜索功能的抽象和具体实现。
    *   **设计**:
        1.  **`SearchProvider` Trait**: 定义了通用的搜索引擎接口（`search` 方法），遵循依赖倒置原则，便于扩展。
        2.  **`ClmclmProvider`**: `SearchProvider` 的具体实现，使用 `reqwest` 和 `scraper` 从特定网站抓取数据。
        3.  **`GenericProvider`**: `SearchProvider` 的另一个实现，用于处理自定义搜索引擎。它集成了 `LlmClient`，当配置了 LLM 时，将整个 HTML 发送给 AI 进行智能提取，而非依赖固定的 CSS 选择器。
        4.  **`SearchCore`**: 封装多个 `SearchProvider`，负责协调并发搜索。
        5.  **`create_ai_enhanced_search_core`**: 作为工厂函数，根据配置动态创建 `SearchCore` 实例。

*   **`llm_service.rs`**:
    *   **职责**: 封装与大型语言模型（LLM）交互的所有逻辑。
    *   **设计**:
        1.  **`LlmClient` Trait**: 定义了与 LLM 服务交互的统一接口，使得未来可以替换或增加新的 LLM 提供商。
        2.  **`GeminiClient`**: `LlmClient` 的具体实现，负责与 Google Gemini API 通信。
        3.  **Prompt Engineering**: 模块的核心价值在于精心设计的 Prompt，通过明确的角色扮演、严格的JSON输出格式和任务分解，确保AI输出的稳定性和可靠性。
        4.  **错误处理与重试**: 包含 API 调用失败时的自动重试逻辑，增强系统健壮性。

### 模块间关系

`main.rs` 作为协调者，接收前端请求，从 `app_state` 获取状态，调用 `searcher.rs` 执行搜索。`searcher.rs` 中的 `GenericProvider` 在需要时会调用 `llm_service.rs` 与 LLM API 交互。此流程清晰地展示了责任分离和模块协作。

---

## 5. 前后端交互

前后端交互完全通过 Tauri 的 `invoke` API 实现，这是一个安全且高效的机制。

1.  **前端调用**: 在 Vue 组件中，使用 `invoke` 函数，并传入命令名（如 `"search_clmclm_first"`）和参数对象。
2.  **后端处理**: Tauri 接收调用，匹配到 `main.rs` 中对应的 `#[tauri::command]` 函数，并将参数反序列化为 Rust 类型。
3.  **数据返回**: Rust 函数的返回值被序列化为 JSON，并通过 `Promise` 返回给前端。
4.  **前端接收**: 前端的 `invoke` 调用返回一个 `Promise`，通过 `await` 异步接收后端数据。

**示例**:
```typescript
// 前端 (HomePage.vue)
const results = await invoke("search_clmclm_first", {
  keyword: keyword.value,
  maxPages: maxPages.value,
});
```
此调用将触发 Rust 后端 `search_clmclm_first` 函数的执行，并将结果返回给前端的 `results` 变量。