#!/bin/bash

# 设置脚本，用于安装 AI-Magnet-Assistant 的所有依赖项。

# 1. 安装前端 (Node.js) 依赖
echo "--- Installing frontend dependencies... ---"
if [ -f "package.json" ]; then
    npm install
else
    echo "package.json not found. Skipping frontend dependencies."
fi

echo "" # 添加空行以提高可读性

# 2. 安装后端 (Rust) 依赖
echo "--- Installing backend dependencies... ---"
if [ -d "src-tauri" ] && [ -f "src-tauri/Cargo.toml" ]; then
    # 进入 tauri 目录并构建，cargo 会自动处理依赖安装
    (cd src-tauri && cargo build --release)
else
    echo "src-tauri/Cargo.toml not found. Skipping backend dependencies."
fi

echo ""
echo "--- Setup complete. ---"