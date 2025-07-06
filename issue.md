# Tauri Command Registration Issue Report

## 问题描述

在Link Optimizer Pro项目的开发过程中，遇到了Tauri命令无法正确注册和调用的问题。前端调用`invoke`函数时始终报错"Command not found"或"Cannot read properties of undefined (reading 'invoke')"。

## 环境信息

- **项目**: Link Optimizer Pro (Aether Magnet UI)
- **框架**: Tauri v2 + Vue 3 + TypeScript
- **操作系统**: Windows 10
- **Node.js**: 最新版本
- **Rust**: 最新版本

## 问题症状

### 1. 命令未找到错误
```
Error: Command test_connection not found
Error: Command search_with_filter not found
Error: Command search_multi_page not found
```

### 2. invoke函数未定义错误
```
TypeError: Cannot read properties of undefined (reading 'invoke')
```

### 3. 前端热重载问题
- 后端Rust代码修改后能正确编译
- 前端Vue代码修改后HMR更新正常
- 但新注册的Tauri命令无法被前端识别

## 已尝试的解决方案

### 1. 后端命令注册
✅ **已确认正确**: 在`lib.rs`中正确注册了所有命令
```rust
.invoke_handler(tauri::generate_handler![
    greet,
    simple_test,
    test_connection,
    search_command,
    search_multi_page,
    save_llm_config,
    load_llm_config,
    search_with_filter
])
```

### 2. 前端导入方式
❌ **问题**: 尝试了多种导入方式都失败
```typescript
// 标准导入
import { invoke } from "@tauri-apps/api/core";

// 动态导入
const tauriCore = await import("@tauri-apps/api/core");

// 全局对象访问
(window as any).__TAURI__.core.invoke;
```

### 3. 应用重启和缓存清理
❌ **无效**:
- 完全重启开发服务器
- 清理Cargo构建缓存 (`cargo clean`)
- 杀掉占用端口的进程
- 强制刷新浏览器

### 4. 环境检测和错误处理
✅ **已实现**: 添加了完善的错误处理和环境检测
```typescript
if (typeof invoke === 'undefined' || !invoke) {
  alert('Tauri invoke function is not available. Please run in Tauri app.');
  return;
}
```

## 根本原因分析

### 发现的关键问题
在`Cargo.toml`中，Tauri的features配置为空数组：
```toml
tauri = { version = "2", features = [] }  # ❌ 问题所在
```

### 解决方案
需要添加必要的features来启用API功能：
```toml
tauri = { version = "2", features = ["shell-open"] }  # ✅ 修复
```

## 技术细节

### 项目结构
```
aether-magnet-ui/
├── src/                    # Vue前端代码
│   ├── App.vue            # 主应用组件
│   └── components/        # 组件目录
├── src-tauri/             # Tauri后端代码
│   ├── src/
│   │   ├── lib.rs         # 主库文件，命令注册
│   │   ├── searcher.rs    # 搜索引擎模块
│   │   ├── filter.rs      # 筛选引擎模块
│   │   └── llm_service.rs # LLM服务模块
│   ├── Cargo.toml         # Rust依赖配置
│   └── tauri.conf.json    # Tauri配置
└── package.json           # Node.js依赖配置
```

### 已实现的功能模块
1. **搜索引擎核心** - 多提供商支持，并发搜索
2. **双轨筛选引擎** - 优先级筛选 + LLM智能筛选
3. **LLM服务集成** - OpenAI和Gemini支持
4. **前端界面** - 现代化Vue 3组件
5. **配置管理** - 持久化存储

### 命令列表
- `greet` - 测试命令
- `simple_test` - 简单测试命令
- `test_connection` - 网络连接测试
- `search_command` - 基础搜索
- `search_multi_page` - 多页搜索
- `save_llm_config` - 保存LLM配置
- `load_llm_config` - 加载LLM配置
- `search_with_filter` - 智能筛选搜索

## 当前状态

### ✅ 已完成
- 后端Rust代码架构完整
- 前端Vue组件开发完成
- 错误处理和调试日志完善
- 识别并修复了Cargo.toml配置问题

### 🔄 待验证
- Tauri features修复后的功能测试
- 所有命令的正常调用
- 完整的搜索和筛选流程

### 📋 后续任务
1. 验证修复后的命令调用
2. 恢复真实网络搜索功能（当前使用测试数据）
3. 完善LLM集成和智能筛选
4. 性能优化和用户体验改进

## 经验教训

1. **Tauri配置的重要性**: features配置直接影响API可用性
2. **调试策略**: 从简单的测试命令开始，逐步排查问题
3. **环境隔离**: 浏览器环境和Tauri环境的API差异
4. **热重载限制**: 某些配置更改需要完全重启应用

## 参考资源

- [Tauri v2 Documentation](https://tauri.app/v1/guides/)
- [Tauri Command System](https://tauri.app/v1/guides/features/command)
- [Vue 3 + Tauri Integration](https://tauri.app/v1/guides/getting-started/setup/vite)