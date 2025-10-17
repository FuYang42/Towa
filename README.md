# Towa - 开发环境快速搭建工具

Towa 是一个用于在全新电脑或系统中快速搭建开发环境的工具集，特别针对：
- C/C++ 开发环境
- Rust 开发环境
- Cepton 传感器调试环境
- **Cepton Unified Firmware (UFB) 开发环境**

## 项目结构

```
Towa/
├── src/
│   ├── c/          # C 语言编写的系统工具
│   └── rust/       # Rust 编写的配置管理工具
├── scripts/        # 自动化安装脚本
├── docs/           # 文档
└── README.md
```

## 功能特性

### 1. 环境检测
- 自动检测操作系统类型和版本
- 检查已安装的开发工具和依赖
- 生成环境报告

### 2. 开发工具安装
- **C/C++ 工具链**: GCC, Clang, CMake, Make
- **Rust 工具链**: rustup, cargo, rustfmt, clippy
- **版本控制**: Git
- **编辑器**: VS Code (可选)

### 3. Cepton SDK 配置
- 自动下载和安装 Cepton SDK
- 配置环境变量
- 编译示例程序
- 传感器连接测试

### 4. Cepton UFB 开发环境
- **一键安装 UFB 所需依赖**：CMake, Ninja, Python 3, Rust
- **ARM 工具链配置指导**：LLVM+Clang Embedded Toolchain
- **Git 配置优化**：符号链接、CRLF 处理
- **Pre-commit 钩子**：代码质量检查
- **详细文档**：完整的 UFB 环境配置指南

## 快速开始

### Windows

```bash
# 运行环境检测
.\scripts\setup.bat check

# 自动安装所有工具
.\scripts\setup.bat install

# 只安装 Cepton SDK
.\scripts\setup.bat cepton

# 设置 UFB 开发环境（推荐用于固件开发）
towa-cli ufb
```

### Linux/macOS

```bash
# 运行环境检测
./scripts/setup.sh check

# 自动安装所有工具
./scripts/setup.sh install

# 只安装 Cepton SDK
./scripts/setup.sh cepton

# 设置 UFB 开发环境（推荐用于固件开发）
towa-cli ufb
```

## UFB 固件开发快速配置

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

## 工具说明

### C 工具
- **env_checker**: 环境检测工具，快速扫描系统配置
- **pkg_installer**: 包管理器封装，统一安装接口

### Rust 工具
- **towa-cli**: 命令行主程序，提供交互式配置向导
- **config-manager**: 配置文件管理器

## 构建项目

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

## 支持的平台

- Windows 10/11
- Ubuntu 20.04+
- macOS 11+

## 文档

- **[快速开始指南](docs/QUICKSTART.md)** - 5分钟快速上手
- **[构建说明](docs/BUILDING.md)** - 详细的编译和安装说明
- **[Cepton SDK 配置](docs/CEPTON_SETUP.md)** - Cepton 传感器 SDK 配置
- **[Cepton UFB 环境配置](docs/CEPTON_UFB_SETUP.md)** - 固件开发环境完整指南 ⭐
- **[项目技术概览](PROJECT_OVERVIEW.md)** - 项目架构和技术细节

## 常用命令（towa-cli）

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

# 设置 UFB 开发环境
towa-cli ufb

# 初始化配置文件
towa-cli init

# 查看版本
towa-cli version
```

## 贡献

欢迎提交 Issue 和 Pull Request!

## 许可证

MIT License
