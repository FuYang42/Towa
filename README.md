# Towa - 开发环境快速搭建工具

Towa 是一个用于在全新电脑或系统中快速搭建开发环境的工具集，特别针对：
- C/C++ 开发环境
- Rust 开发环境
- Cepton 传感器调试环境（使用统一固件库）

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

## 快速开始

### Windows

```bash
# 运行环境检测
.\scripts\setup.bat check

# 自动安装所有工具
.\scripts\setup.bat install

# 只安装 Cepton SDK
.\scripts\setup.bat cepton
```

### Linux/macOS

```bash
# 运行环境检测
./scripts/setup.sh check

# 自动安装所有工具
./scripts/setup.sh install

# 只安装 Cepton SDK
./scripts/setup.sh cepton
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

## 贡献

欢迎提交 Issue 和 Pull Request!

## 许可证

MIT License
