[English](README.md)
**声明:** 本项目的大部分代码是在 AI Development Tools 的辅助下完成的。如果您对这些 AI 工具有兴趣，欢迎访问 [https://ai.pu.edu.kg/](https://ai.pu.edu.kg/) 为它们打分和评价。

# AI Magnet Assistant (AI 磁力助手)

## 项目简介

**AI Magnet Assistant** 是一款基于 Rust、Tauri 和 Vue.js 构建的智能磁力链接搜索与优化工具。它利用 AI 的强大能力，通过聚合多个搜索引擎的结果、过滤无效链接并对其进行质量排名。

## 主要功能

-   **多引擎聚合搜索**: 同时检索多个资源站点，自动去重与合并结果。
-   **AI 结果优化**: 接入 LLM 对标题/摘要/大小等进行分析、评分与打标签，按相关度排序。
-   **无效链接过滤**: 自动剔除失效、重复、低质量链接。
-   **关键词优先**: 含指定关键词的条目优先展示（如 4K、中文字幕）。
-   **收藏与集中下载**: 一键收藏搜索结果，统一查看与下载。
-   **一键下载**: 支持跳转到你的常用下载器或 115 离线下载页面，可配置自动关闭。
-   **可扩展搜索引擎**: 内置稳定引擎，支持添加自定义站点；已优化自定义站点的网页抓取逻辑。
-   **批量分析与回退**: 批量并行分析，失败时自动降级为单项分析，并实时展示进度。
-   **国际化（i18n）**: 支持中英双语，运行时切换并持久化到后端。
-   **跨平台**: Windows、Linux、macOS（Linux/macOS 构建可用，暂未在真机全面测试）。
-   **双阶段 AI 配置**: 将 HTML 提取与内容分析的模型/地址/API Key 分离配置，并提供连通性测试按钮。

## 如何开始

### 环境要求

-   [Node.js](https://nodejs.org/zh-cn/)
-   [Rust](https://www.rust-lang.org/zh-CN/tools/install)

### 安装步骤

1.  克隆代码仓库：
    ```bash
    git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
    cd AI-Magnet-Assistant
    ```

2.  安装依赖：
    ```bash
    npm install
    ```

### 开发模式运行

在开发模式下运行应用，支持热重载：

```bash
npm run tauri dev
```

仅启动前端用于 UI 开发：

```bash
npm run vite:dev
```

### 构建应用

为当前平台构建可执行的应用：

```bash
npm run tauri build
```

## 国际化

- 默认语言：简体中文（`zh-CN`）
- 支持语言：`zh-CN`、`en`
- 可在“设置”（或开启“调试区域”后在页面顶部）切换语言；选择会持久化并在启动时恢复。

## 版本与更新

详见 `docs/RELEASES.md`。当前版本：1.2.0。

## 许可证

本项目采用 **MIT 许可证**。详情请参阅 [LICENSE](LICENSE) 文件。