# 🧲 MagnetLink Optimizer Pro

**MagnetLink Optimizer Pro** 是一款智能磁力链接搜索与优化桌面软件，旨在解决用户在传统磁力搜索网站中面临的信息过载、广告繁多、优质资源筛选困难等痛点。

## ✨ 核心特性

- 🔍 **智能搜索** - 集成多个磁力搜索引擎，提供全面的搜索结果
- 🤖 **AI 驱动筛选** - 使用大语言模型（LLM）智能分析和筛选高质量资源
- ⚡ **多线程加速** - 并发搜索多个数据源，大幅提升搜索速度
- 🎯 **精准过滤** - 智能去除广告和低质量内容，提供纯净搜索体验
- 🎨 **现代化界面** - 基于 Tauri + Vue 3 构建的优雅桌面应用
- 📊 **质量评分** - 为每个搜索结果提供智能质量评分
- 🔧 **可扩展架构** - 支持插件化搜索引擎扩展

## 🏗️ 技术架构

### 前端技术栈
- **框架**: Tauri 2.x + Vue 3 + TypeScript
- **构建工具**: Vite
- **UI 组件**: 自定义组件库
- **状态管理**: Vue 3 Composition API

### 后端技术栈
- **核心语言**: Rust
- **异步运行时**: Tokio
- **HTTP 客户端**: Reqwest
- **HTML 解析**: Scraper
- **并发处理**: Rayon

### AI 集成
- **LLM 服务**: 支持多种大语言模型
- **内容分析**: 智能质量评估和内容分类
- **批量处理**: 高效的批量分析能力

## 🚀 快速开始

### 环境要求

- **Rust**: 1.70.0 或更高版本
- **Node.js**: 18.0.0 或更高版本
- **npm**: 9.0.0 或更高版本

### 安装依赖

1. **克隆项目**
   ```bash
   git clone https://github.com/your-username/MagnetLink-Optimizer-Pro.git
   cd MagnetLink-Optimizer-Pro
   ```

2. **安装前端依赖**
   ```bash
   cd magnetlink-optimizer-pro-ui
   npm install
   ```

3. **构建 Rust 核心**
   ```bash
   cd ../magnetlink-optimizer-pro-core
   cargo build --release
   ```

### 开发模式

```bash
cd magnetlink-optimizer-pro-ui
npm run tauri dev
```

### 构建发布版本

```bash
cd magnetlink-optimizer-pro-ui
npm run tauri build
```

## 📁 项目结构

```
MagnetLink-Optimizer-Pro/
├── magnetlink-optimizer-pro-core/     # Rust 核心库
│   ├── src/
│   │   ├── main.rs                    # 主程序入口
│   │   ├── searcher.rs                # 搜索引擎实现
│   │   ├── filter.rs                  # 内容过滤器
│   │   └── llm_service.rs             # LLM 服务集成
│   └── Cargo.toml                     # Rust 项目配置
├── magnetlink-optimizer-pro-ui/       # Tauri 前端应用
│   ├── src/                           # Vue 源代码
│   ├── src-tauri/                     # Tauri 后端
│   ├── public/                        # 静态资源
│   └── package.json                   # Node.js 项目配置
├── memory_bank/                       # 项目文档和记录
├── Technical_Roadmap.md               # 技术路线图
├── Link Optimizer Pro PRD.md         # 产品需求文档
└── README.md                          # 项目说明文档
```

## 🔧 配置说明

### LLM 配置

在使用 AI 功能前，需要配置相应的 LLM 服务：

1. 复制配置模板：`config.example.toml` → `config.toml`
2. 填入您的 API 密钥和服务端点
3. 选择合适的模型参数

### 搜索引擎配置

支持配置多个磁力搜索引擎：
- 内置常用搜索引擎
- 支持自定义搜索引擎
- 可配置搜索优先级和超时设置

## 🤝 贡献指南

我们欢迎所有形式的贡献！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- 所有贡献者和支持者

## 📞 联系我们

- 项目主页: [GitHub Repository](https://github.com/your-username/MagnetLink-Optimizer-Pro)
- 问题反馈: [Issues](https://github.com/your-username/MagnetLink-Optimizer-Pro/issues)
- 功能建议: [Discussions](https://github.com/your-username/MagnetLink-Optimizer-Pro/discussions)

---

**MagnetLink Optimizer Pro** - 让磁力搜索更智能、更高效！ 🚀
