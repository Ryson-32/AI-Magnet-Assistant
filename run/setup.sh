#!/usr/bin/env bash

# 设置脚本，用于安装 AI-Magnet-Assistant 的所有依赖项。

#!/usr/bin/env bash

# 步骤 0: 安装 Linux 系统依赖 (用于 Tauri 后端)
# 检查是否在 Linux 环境中
if [[ "$(uname)" == "Linux" ]]; then
    echo "--- Installing Linux system dependencies for Tauri... ---"
    # 假设是 Debian/Ubuntu-based 系统
    # 使用 sudo 需要管理员权限
    sudo apt-get update -y && sudo apt-get install -y \
        libwebkit2gtk-4.1-dev \
        build-essential \
        curl \
        wget \
        file \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev
else
    echo "--- Skipping Linux system dependencies (not on Linux)... ---"
fi

echo "" # 添加空行以提高可读性

# 步骤 1: 安装前端 (Node.js) 依赖
echo "--- Installing frontend dependencies... ---"
if [ -f "package.json" ]; then
    npm ci || npm install
else
    echo "package.json not found. Skipping frontend dependencies."
fi

echo "" # 添加空行以提高可读性

# 步骤 2: 安装后端 (Rust) 依赖
echo "--- Installing backend dependencies... ---"
if [ -d "src-tauri" ] && [ -f "src-tauri/Cargo.toml" ]; then
    # 进入 tauri 目录并构建，cargo 会自动处理依赖安装
    (cd src-tauri && cargo build --release)
else
    echo "src-tauri/Cargo.toml not found. Skipping backend dependencies."
fi

echo ""
echo "--- Setup complete. ---"