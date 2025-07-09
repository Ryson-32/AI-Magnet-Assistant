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