# AI-Magnet-Assistant - Developer Manual

Welcome to contributing to AI-Magnet-Assistant! This manual is intended to provide clear guidance for developers to ensure a smooth development process and consistent code quality.

## 1. Architecture Overview

Before you begin, please be sure to read our [Software Architecture Design Document (ARCHITECTURE.md)](ARCHITECTURE.md) to fully understand the project's technology stack, code structure, and design principles.

## 2. Development Environment Setup

This project relies on Node.js (frontend) and Rust (backend).

### 2.1. Install Prerequisites

*   **Node.js**: The latest LTS version is recommended. You can install it via [nvm](https://github.com/nvm-sh/nvm) or the official installer.
*   **Rust**: Install the latest stable Rust toolchain via [rustup](https://rustup.rs/).
*   **System Dependencies (Linux)**: If you are developing in a Linux environment, Tauri requires some additional system libraries. Please refer to the official Tauri documentation for installation, or run the setup script in the project root directory.

### 2.2. One-Click Project Dependency Configuration

We provide an automated script to install all necessary frontend and backend dependencies. Run the following in the project root directory:

```bash
./run/setup.sh
```
*Note: This script will attempt to use `sudo apt-get` to install system dependencies in a Linux environment, which may require you to enter your password.*

## 3. Coding Standards

To maintain the clarity and maintainability of the codebase, please follow these standards.

### 3.1. Frontend (Vue / TypeScript)

*   **Style**: We follow the standard [Vue Official Style Guide](https://vuejs.org/style-guide/).
*   **Formatting**: Use Prettier for code formatting. It is recommended to install the Prettier plugin in your IDE and set it to format on save.
*   **Naming**:
    *   Component files use PascalCase, e.g., `SideNavigation.vue`.
    *   Variables and functions use camelCase.
*   **Types**: Provide explicit TypeScript types for all variables, function parameters, and return values whenever possible.

### 3.2. Backend (Rust)

*   **Style**: Follow the standard [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
*   **Formatting**: Use `rustfmt` for code formatting. Run `cargo fmt` to format the entire backend project.
*   **Code Quality (Linting)**: Use `clippy` for code quality checks. Before committing code, please run `cargo clippy -- -D warnings` and fix all reported issues.
*   **Error Handling**: Prefer using `Result<T, E>` for recoverable error handling, and use the `anyhow` crate to simplify error chain propagation.

## 4. Building and Testing

### 4.1. Development Mode

To start the development server with hot-reloading, run:

```bash
npm run dev
```
This will start both the frontend development server and the backend Tauri application.

### 4.2. Production Build

To build the final, distributable desktop application, run:

```bash
npm run tauri build
```
After a successful build, the executable or installer will be located in the `src-tauri/target/release/` directory.

---
Thank you for your contribution!
---
# AI-Magnet-Assistant - 开发者手册

欢迎为 AI-Magnet-Assistant 贡献代码！本手册旨在为开发者提供清晰的指导，以确保开发过程的顺利和代码质量的一致性。

## 1. 架构概览

在开始之前，请务必阅读我们的 [软件架构设计文档 (ARCHITECTURE.md)](ARCHITECTURE.md)，以全面了解本项目的技术选型、代码结构和设计原则。

## 2. 开发环境设置

本项目依赖于 Node.js (前端) 和 Rust (后端)。

### 2.1. 安装先决条件

*   **Node.js**: 推荐使用最新的LTS版本。您可以通过 [nvm](https://github.com/nvm-sh/nvm) 或官方安装程序进行安装。
*   **Rust**: 通过 [rustup](https://rustup.rs/) 安装最新的稳定版Rust工具链。
*   **系统依赖 (Linux)**: 如果您在Linux环境下开发，Tauri需要一些额外的系统库。请参考Tauri官方文档进行安装，或运行项目根目录下的设置脚本。

### 2.2. 一键配置项目依赖

我们提供了一个自动化脚本来安装所有必需的前后端依赖。在项目根目录下运行：

```bash
./run/setup.sh
```
*注意：此脚本在Linux环境下会尝试使用 `sudo apt-get` 安装系统依赖，可能需要您输入密码。*

## 3. 编码规范

为了保持代码库的清晰和可维护性，请遵循以下规范。

### 3.1. 前端 (Vue / TypeScript)

*   **风格**: 我们遵循标准的 [Vue 官方风格指南](https://vuejs.org/style-guide/)。
*   **格式化**: 使用 Prettier 进行代码格式化。建议在您的IDE中安装Prettier插件，并设置为保存时自动格式化。
*   **命名**:
    *   组件文件使用大驼峰命名法 (PascalCase)，例如 `SideNavigation.vue`。
    *   变量和函数使用小驼峰命名法 (camelCase)。
*   **类型**: 尽可能为所有变量、函数参数和返回值提供明确的TypeScript类型。

### 3.2. 后端 (Rust)

*   **风格**: 遵循标准的 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)。
*   **格式化**: 使用 `rustfmt` 进行代码格式化。运行 `cargo fmt` 来格式化整个后端项目。
*   **代码质量 (Linting)**: 使用 `clippy` 进行代码质量检查。在提交代码前，请运行 `cargo clippy -- -D warnings` 并修复所有报告的问题。
*   **错误处理**: 优先使用 `Result<T, E>` 进行可恢复的错误处理，使用 `anyhow` crate 来简化错误链的传递。

## 4. 构建与测试

### 4.1. 开发模式

要启动带有热重载功能的开发服务器，请运行：

```bash
npm run dev
```
这将同时启动前端开发服务器和后端Tauri应用。

### 4.2. 生产构建

要构建最终的、可分发的桌面应用程序，请运行：

```bash
npm run tauri build
```
构建成功后，可执行文件或安装包将位于 `src-tauri/target/release/` 目录下。

---
感谢您的贡献！