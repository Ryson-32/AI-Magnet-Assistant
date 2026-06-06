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

更多用法参考：https://linux.do/t/topic/851896?u=ryanvan

**AI Magnet Assistant** 是一款AI磁力搜索与优化工具。它聚合多引擎结果，对页面使用 AI 提取并清洗标题/标签/分数，帮助你更快找到“干净”的结果。

## 亮点与场景 ⭐

- **解决痛点**：页面结构混乱、标题噪音多；结果质量参差；需要大量手动筛选。
- **主要技术**：Tauri + Rust（后端/系统）、Vue 3 + TypeScript（前端）。
- **核心功能**：
  - 多引擎聚合：推荐使用 Prowlarr 正式 API，并兼容旧式 HTML 引擎。
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

1. **HTML 内容提取**：后端通过 OpenAI-compatible 提供商从原始 HTML 中提取条目，返回基础字段。
2. **内容分析**：前端并行批量清洗标题、计算纯净度分与生成标签。失败自动回退为单项分析，过程状态实时更新。

### 持久化

- 所有配置与数据（引擎、收藏、优先级关键词、AI 配置、语言等）保存在 `app_data.json`。可在设置 → 数据 中打开目录。

## 注意事项 📝

后端支持 Agnes、DeepSeek 与自定义 OpenAI-compatible 接口。

- **agnes-2.0-flash**：默认用于 HTML 提取。
- **deepseek-chat**：默认用于内容分析。

实际速度取决于网络与页面复杂度；批量分析并行执行并具备自动回退机制。

## 使用流程 🧭

1. **初始化**
   - 进入设置 → AI 配置，分别填写提取与分析的 API Base、Model、API Key，并使用“测试”按钮验证；
   - 在“搜索引擎”中添加 Prowlarr 地址与 API Key，或按需添加旧式 HTML 引擎；
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

### 克隆

```bash
git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
cd AI-Magnet-Assistant
```

### 安装

```bash
npm install
```

### 运行（开发）

```bash
npm run dev
```

仅前端：

```bash
npm run vite:dev
```

### 构建

```bash
npx tauri build
```

## 已知问题 🐞

- 部分站点为强 JS 或存在反爬挑战，旧式 HTML 搜索可能失败或产出有限结果。
- 推荐使用 Prowlarr 搜索源；成人分类是否显示由每个 Prowlarr 引擎的“允许成人分类”开关控制。
- 提供商速率限制可能导致分析失败。
- 自定义下载程序“快速下载”功能仅在 Windows 下可用。

### 发布版中的 Prowlarr

- 详细跑通教程请看：[本地 Prowlarr 配置说明](docs/PROWLARR_LOCAL_SETUP.zh-CN.md)。
- Prowlarr 不内置于应用安装包中。用户需要自行安装本地 Prowlarr，或连接可访问的远程实例。
- 首次使用时，在 Prowlarr 中添加至少一个索引源，然后从“设置 → 常规 → 安全”复制 API Key。
- 在本应用“搜索引擎 → Prowlarr”中填写地址和 API Key，并使用连接测试确认配置。
- 搜索引擎配置和 API Key 仅保存在当前用户的应用数据目录，不会进入 Git 仓库或发布包。
- 成人分类可在每个 Prowlarr 引擎中单独开启或关闭。
- 本应用只显示 Prowlarr 中已配置的索引源，并提供启用/禁用开关；添加、测试和连接失败诊断请在 Prowlarr 中完成。
- Prowlarr 返回的本地代理下载 URL 不会作为磁力链接展示；存在 info hash 时，应用会生成标准 `magnet:?xt=urn:btih:...` 链接。
- 中文资源覆盖取决于用户配置的索引源。公开源可尝试 ACG.RIP、Bangumi Moe、dmhy；部分中文影视索引源需要账号。
- 私有站凭据由 Prowlarr 管理。不同站点可能要求 API Key、Cookie 或 Passkey，本应用不会读取或保存这些站点账号凭据。

不建议同时内置 Prowlarr 与 Jackett：两者职责高度重叠，会显著增加发布包体积、后台进程管理和 GPL 再分发维护成本。Jackett/Torznab 更适合作为后续可选兼容接口。

## 许可证 📄

MIT 许可证，详见 [LICENSE](LICENSE)。
