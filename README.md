# Towa - 开发环境快速搭建工具

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)](https://github.com/FuYang42/Towa)
[![GitHub release](https://img.shields.io/github/v/release/FuYang42/Towa)](https://github.com/FuYang42/Towa/releases)

### 🌏 语言 / Language / 言語

**[中文](README.md)** | [English](README.en.md) | [日本語](README.ja.md)

---

让开发环境配置变得简单、快速、友好

</div>

---

## ✨ 简介

Towa 是一个用于在全新电脑或系统中快速搭建开发环境的工具集，特别针对：

- 🔧 C/C++ 开发环境
- 🦀 Rust 开发环境
- 📡 Cepton 传感器调试环境
- ⚙️ **Cepton Unified Firmware (UFB) 开发环境**

## 🎯 3 步开始使用

```bash
# 1️⃣ 克隆项目
git clone https://github.com/FuYang42/Towa.git
cd Towa

# 2️⃣ 运行安装脚本
./scripts/setup.sh install    # Linux/macOS
# 或
.\scripts\setup.bat install    # Windows

# 3️⃣ 验证安装
./scripts/setup.sh check
```

✅ **就这么简单！5 分钟完成环境配置**

📖 **详细教程:** [快速上手指南 →](GETTING_STARTED.md)

## 📁 项目结构

```
Towa/
├── src/
│   ├── c/          # C 语言编写的系统工具
│   └── rust/       # Rust 编写的配置管理工具
├── scripts/        # 自动化安装脚本
├── docs/           # 文档
│   ├── QUICKSTART.md           # 快速开始
│   ├── CEPTON_UFB_SETUP.md     # UFB 完整配置指南
│   └── UFB_QUICKSTART.md       # UFB 快速上手
└── README.md
```

## 🎯 功能特性

### 1. 🔍 环境检测
- ✅ 自动检测操作系统类型和版本
- ✅ 检查已安装的开发工具和依赖
- ✅ 生成详细的环境报告

### 2. 🛠️ 开发工具安装
- **C/C++ 工具链**: GCC, Clang, CMake, Make
- **Rust 工具链**: rustup, cargo, rustfmt, clippy
- **版本控制**: Git
- **编辑器**: VS Code (可选)

### 3. 📡 Cepton SDK 配置
- 自动下载和安装 Cepton SDK
- 配置环境变量
- 编译示例程序
- 传感器连接测试

### 4. ⚙️ Cepton UFB 开发环境
- **一键安装 UFB 所需依赖**：CMake, Ninja, Python 3, Rust
- **ARM 工具链配置指导**：LLVM+Clang Embedded Toolchain
- **Git 配置优化**：符号链接、CRLF 处理
- **Pre-commit 钩子**：代码质量检查
- **详细文档**：完整的 UFB 环境配置指南

## 🚀 快速开始

### Windows 用户

```bash
# 1️⃣ 运行环境检测
.\scripts\setup.bat check

# 2️⃣ 自动安装所有工具
.\scripts\setup.bat install

# 3️⃣ （可选）只安装 Cepton SDK
.\scripts\setup.bat cepton

# 4️⃣ （推荐）设置 UFB 开发环境
towa-cli ufb
```

### Linux/macOS 用户

```bash
# 1️⃣ 运行环境检测
./scripts/setup.sh check

# 2️⃣ 自动安装所有工具
./scripts/setup.sh install

# 3️⃣ （可选）只安装 Cepton SDK
./scripts/setup.sh cepton

# 4️⃣ （推荐）设置 UFB 开发环境
towa-cli ufb
```

## 💡 UFB 固件开发快速配置

如果您需要开发 Cepton 固件（Nova, Ultra 等），请使用以下步骤：

```bash
# 1. 安装基础工具
./scripts/setup.sh install

# 2. 设置 UFB 环境
cd src/rust
cargo build --release
./target/release/towa-cli ufb

# 3. 按照提示安装 ARM 工具链

# 4. 验证环境
./target/release/towa-cli check

# 5. 查看详细文档
# 参考 docs/CEPTON_UFB_SETUP.md
```

💡 **提示**: 使用 `towa-cli ufb` 可以自动检测并安装 UFB 所需的所有依赖！

## 🔧 工具说明

### C 工具
- **env_checker**: 环境检测工具，快速扫描系统配置
- **pkg_installer**: 包管理器封装，统一安装接口

### Rust 工具
- **towa-cli**: 命令行主程序，提供交互式配置向导
- **config-manager**: 配置文件管理器

## 🏗️ 构建项目

### 构建 C 工具

```bash
cd src/c
mkdir build && cd build
cmake ..
make
```

### 构建 Rust 工具

```bash
cd src/rust
cargo build --release
```

## 💻 支持的平台

| 平台 | 版本 | 状态 |
|------|------|------|
| 🪟 Windows | 10/11 | ✅ 完全支持 |
| 🐧 Ubuntu | 20.04+ | ✅ 完全支持 |
| 🍎 macOS | 11+ | ✅ 完全支持 |

## 📚 文档

- **[快速开始指南](docs/QUICKSTART.md)** - 5分钟快速上手 ⚡
- **[构建说明](docs/BUILDING.md)** - 详细的编译和安装说明 📖
- **[Cepton SDK 配置](docs/CEPTON_SETUP.md)** - Cepton 传感器 SDK 配置 📡
- **[Cepton UFB 环境配置](docs/CEPTON_UFB_SETUP.md)** - 固件开发环境完整指南 ⭐
- **[UFB 快速开始](docs/UFB_QUICKSTART.md)** - UFB 开发 5-10 分钟上手 🚀
- **[项目技术概览](PROJECT_OVERVIEW.md)** - 项目架构和技术细节 🏛️

## 🎮 常用命令（towa-cli）

构建 Rust CLI 后可使用以下命令：

```bash
# 环境检测
towa-cli check

# 安装所有工具
towa-cli install all

# 只安装 C 工具链
towa-cli install c

# 只安装 Rust 工具链
towa-cli install rust

# 配置 Cepton SDK
towa-cli cepton

# 设置 UFB 开发环境 ⭐
towa-cli ufb

# 初始化配置文件
towa-cli init

# 查看版本
towa-cli version
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request!

我们欢迎各种形式的贡献：
- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码修复

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

---

<div align="center">

**Made with ❤️ by Towa Contributors**

⭐ 如果这个项目对您有帮助，请给我们一个 Star！

[GitHub](https://github.com/FuYang42/Towa) · [Issues](https://github.com/FuYang42/Towa/issues) · [Releases](https://github.com/FuYang42/Towa/releases)

</div>
