# Version 1.2.0

## English

### ✨ Highlights
- Full i18n for frontend and backend with runtime language switching and persistence
- Settings page overhaul: two-phase AI configuration (Extraction/Analysis), API connectivity tests, rate-limit tooltip, download handler options
- Debug Area with a user-controlled toggle (off by default)
- Improved search and analysis: batch analysis with parallel execution and graceful fallback, better progress states, and priority-keyword boosting
- UI polish: larger fonts for Chinese locale in side navigation and Settings title

### 🔧 Developer & Architecture
- Consolidated SearchCore creation and providers
- Introduced backend i18n module and locale commands; app state persisted to app_data.json
- Added `show_debug_area` to `SearchSettings` with serde default for backward compatibility
- New docs: Architecture, I18N design, and implementation reports

### 📦 Misc
- Vite/Tauri config tweaks, new setup script under `run/`
- `.gitignore` updates, ignoring `memory_bank/` and `private_docs/`

---

## 简体中文

### ✨ 亮点
- 前后端完整国际化，运行时语言切换并持久化
- 设置页重构：AI 提取/分析双阶段配置、API 连通性测试、速率限制浮窗、下载处理选项
- 新增“调试区域”且提供用户开关（默认关闭）
- 搜索与分析提升：并行/分批分析与优雅回退、进度状态优化、优先关键词结果置顶
- UI 优化：中文下左侧导航与“设置”标题字号提升

### 🔧 架构与开发
- 统一 SearchCore 创建与提供者管理
- 新增后端 i18n 模块与语言命令；应用状态持久化至 app_data.json
- `SearchSettings` 新增 `show_debug_area`（serde 默认值，兼容旧数据）
- 新增架构与国际化设计/落地报告等文档

### 📦 其他
- 调整 Vite/Tauri 配置，新增 `run/setup.sh`
- 更新 `.gitignore`，忽略 `memory_bank/` 与 `private_docs/`

---

# Version 1.1.0

## English

### 🚀 Performance & Architecture Overhaul
This release introduces a significant architectural shift for a faster, more responsive experience.
- **Progressive Search:** Search results now appear as they are found, eliminating wait times.
- **Frontend Analysis:** The resource-intensive analysis process has been moved from the backend to the frontend, improving backend performance and scalability.
- **True Batch Analysis:** LLM processing is now handled in true batches, dramatically speeding up analysis for large datasets.

### ✨ New Features & Enhancements
- **Separated AI Configurations:** You can now use different models, endpoints, and API keys for the "Extraction" and "Analysis" phases of AI processing. This provides greater flexibility and cost control.
- **Developer Manual Overhaul:** The developer manual has been completely rewritten for better clarity, providing more detailed and accessible documentation.

### ⚠️ Breaking Changes
- **Configuration File:** The old `app_config.json` is no longer compatible due to the new separated AI configurations. You will need to re-configure the application settings.

---

## 简体中文

### 🚀 性能与架构革新
此版本引入了重大的架构调整，旨在提供更快、更流畅的用户体验。
- **渐进式搜索:** 搜索结果将在找到时立即显示，无需等待。
- **前端分析:** 资源密集型的分析过程已从后端移至前端，提升了后端性能和可伸缩性。
- **真正的批处理分析:** LLM 处理现在以真正的批处理方式进行，极大地加快了对大型数据集的分析速度。

### ✨ 新功能与改进
- **分离的 AI 配置:** 您现在可以为 AI 处理的“提取”和“分析”阶段使用不同的模型、端点和 API 密钥。这提供了更大的灵活性和成本控制能力。
- **开发者手册修订:** 我们对开发者手册进行了全面重写，使其更清晰、更详尽，提供了更易于理解的文档。

### ⚠️ 重要变更
- **配置文件:** 由于引入了新的分离式 AI 配置，旧的 `app_config.json` 文件已不再兼容。您需要重新配置应用程序的设置。