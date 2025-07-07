[English](README.md)

# AI Magnet Assistant (AI 磁力助手)

## 项目简介

**AI Magnet Assistant** 是一款基于 Rust、Tauri 和 Vue.js 构建的智能磁力链接搜索与优化工具。它利用 AI 的强大能力，通过聚合多个搜索引擎的结果、过滤无效链接并对其进行质量排名，为用户提供流畅高效的搜索体验。

## 主要功能

-   **多引擎搜索**: 在多个磁力链接提供商之间进行并行搜索，确保结果的全面性。
-   **AI 驱动优化**: 利用 AI 算法分析、评分和排序搜索结果，优先展示最相关和最可靠的链接。
-   **无效链接过滤**: 自动识别并移除失效或无效的链接，节省您的时间和精力。
-   **直观的用户界面**: 基于 Vue.js 构建的简洁、现代且用户友好的界面，提供无缝的用户体验。
-   **跨平台支持**: 使用 Tauri 构建，使应用程序能够通过单一代码库在 Windows、macOS 和 Linux 上运行。

## 如何开始

### 环境要求

-   [Node.js](https://nodejs.org/zh-cn/)
-   [Rust](https://www.rust-lang.org/zh-CN/tools/install)

### 安装步骤

1.  克隆代码仓库：
    ```bash
    git clone https://github.com/your-username/ai-magnet-assistant.git
    cd ai-magnet-assistant
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

### 构建应用

为当前平台构建可执行的应用：

```bash
npm run tauri build
```

## 许可证

本项目采用 **MIT 许可证**。详情请参阅 [LICENSE](LICENSE) 文件。