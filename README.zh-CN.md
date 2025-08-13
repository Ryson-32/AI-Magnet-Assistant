<!-- markdownlint-disable MD033 MD041 -->
<div align="center">
  <h1>AI Magnet Assistant（AI 磁力助手）</h1>
  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
    <img src="https://img.shields.io/badge/platform-%E8%B7%A8%E5%B9%B3%E5%8F%B0-lightgrey.svg" alt="Platform: Windows/macOS/Linux">
    <img src="https://img.shields.io/badge/version-1.2.0-green.svg" alt="Version 1.2.0">
  </p>
  <p><strong><a href="README.md">English</a> | <a href="README.zh-CN.md">中文</a></strong></p>
</div>
<!-- markdownlint-enable MD033 MD041 -->

<em>如果这个项目对你有帮助，别忘了点个 ⭐！</em>

**AI Magnet Assistant** 是一款AI磁力搜索与优化工具。它聚合多引擎结果，对页面使用 AI 提取并清洗标题/标签/分数，帮助你更快找到“干净”的结果。

## 亮点与场景 ⭐

- **解决痛点**：页面结构混乱、标题噪音多；结果质量参差；需要大量手动筛选。
- **主要技术**：Tauri + Rust（后端/系统）、Vue 3 + TypeScript（前端）。
- **核心功能**：
  - 多引擎聚合：内置引擎优先，其它引擎并发；搜索状态实时显示所用模型。
  - 双阶段 AI：阶段一 HTML 提取（面向自定义引擎）→ 阶段二内容分析（标题清洗、标签、纯净度 0–100）。
  - 搜索效率与管理：收藏与集中下载；优先级关键词；按分数/大小排序；快速复制/打开来源链接。
  - 下载集成：自定义应用路径快速下载。
- **适用场景**：聚合搜索、降噪与标签化、磁力收藏与集中下载。

## 软件截图 🖼️

<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/2297f7cb-720f-4f90-9df5-bddd94685fbd" />
<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/098f1115-a048-40cc-aab9-37e4670d446c" />
<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/153cd245-20bc-4c7c-816f-af709a591b52" />

## 软件原理 ⚙️

### 搜索编排

- **内置引擎优先**：如启用，先返回内置搜索引擎结果，再合并其他引擎；UI 持续推送状态。

### 双阶段 AI（面向自定义引擎与分析）

1. **HTML 内容提取**：后端通过 Gemini 从原始 HTML 中提取条目，返回基础字段。API 基础地址与模型均可配置。
2. **内容分析**：前端并行批量清洗标题、计算纯净度分与生成标签。失败自动回退为单项分析，过程状态实时更新。

### 持久化

- 所有配置与数据（引擎、收藏、优先级关键词、AI 配置、语言等）保存在 `app_data.json`。可在设置 → 数据 中打开目录。

## 注意事项 📝

说明：当前后端实现支持 Google Gemini。UI 中的 OpenAI 选项可见，但后端暂未接入。

- **gemini-2.5-flash**：推荐用于 HTML 提取（阶段一）。
- **gemini-2.5-flash-lite**：推荐用于内容分析（阶段二），更快更省。

实际速度取决于网络与页面复杂度；批量分析并行执行并具备自动回退机制。

## 使用流程 🧭

1. **初始化**
   - 进入设置 → AI 配置，分别填写提取与分析的 API Base、Model、API Key，并使用“测试”按钮验证；
   - 在“搜索引擎”中按模板或示例添加自定义站点；
   - 配置下载（应用路径、快速下载、自动关闭）与语言。

2. **开始搜索**
   - 在首页输入关键词，选择页数与开关项（AI 过滤、标题必须包含关键词）。
   - 内置搜索引擎结果先到，随后并入其他引擎；分析会精简标题/打标签/计算分数。

3. **整理运营**
   - 按分数/大小排序；加入收藏；维护优先级关键词提升命中；打开来源页；复制/打开磁力链接。

## 部署说明 🛠️

### 依赖

- Node.js 18+
- Rust（稳定版）
- Tauri CLI

### 克隆

```bash
git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
cd AI-Magnet-Assistant
```

### 安装

```bash
npm install
npm install -g @tauri-apps/cli
```

### 运行（开发）

```bash
npm run tauri dev
```

仅前端：

```bash
npm run vite:dev
```

### 构建

```bash
npm run tauri build
```

## 已知问题 🐞

- 后端当前未接入 OpenAI，需使用 Gemini。
- 部分站点为强 JS/反爬，HTML 可能为 JS 或乱码时会降级或产出有限结果；日志中会打印诊断预览。
- 速率限制可能导致分析失败；设置页提供速率限制信息与建议（如 gemini-balance）。
- 自定义下载程序“快速下载”功能仅在 Windows 下可用。

## 许可证 📄

MIT 许可证，详见 [LICENSE](LICENSE)。

