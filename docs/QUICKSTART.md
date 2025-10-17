# Towa 快速开始指南

欢迎使用 Towa！本指南将帮助您在 5 分钟内设置好开发环境。

## 概述

Towa 是一个自动化工具，用于在全新系统上快速搭建：
- C/C++ 开发环境
- Rust 开发环境
- Cepton 传感器调试环境

## 第一步：克隆项目

```bash
git clone https://github.com/yourusername/towa.git
cd towa
```

## 第二步：检查当前环境

### Windows
```batch
.\scripts\setup.bat check
```

### Linux/macOS
```bash
./scripts/setup.sh check
```

这会显示您系统上已安装的工具和缺失的依赖。

## 第三步：自动安装

### 完整安装（推荐）

**Windows:**
```batch
.\scripts\setup.bat install
```

**Linux/macOS:**
```bash
./scripts/setup.sh install
```

这将安装：
- C/C++ 编译器（GCC/Clang）
- CMake 和 Make
- Rust 工具链
- Git
- 构建所有 Towa 工具

### 按需安装

**只安装 C 工具链:**
```bash
# Windows
.\scripts\setup.bat c

# Linux/macOS
./scripts/setup.sh c
```

**只安装 Rust 工具链:**
```bash
# Windows
.\scripts\setup.bat rust

# Linux/macOS
./scripts/setup.sh rust
```

## 第四步：配置 Cepton SDK（如需要）

### 1. 下载 SDK

访问 https://www.cepton.com/downloads 下载适合您系统的 SDK。

### 2. 自动配置

**Windows:**
```batch
.\scripts\setup.bat cepton
```

**Linux/macOS:**
```bash
./scripts/setup.sh cepton
```

### 3. 手动配置

**Windows:**
```batch
# 解压 SDK 到 C:\Program Files\CeptonSDK
setx CEPTON_SDK_PATH "C:\Program Files\CeptonSDK"
```

**Linux:**
```bash
# 解压 SDK 到 /opt/cepton_sdk
sudo tar -xzf cepton_sdk.tar.gz -C /opt/cepton_sdk

# 添加到环境变量
echo 'export CEPTON_SDK_PATH=/opt/cepton_sdk' >> ~/.bashrc
echo 'export PATH=$PATH:$CEPTON_SDK_PATH/bin' >> ~/.bashrc
source ~/.bashrc
```

## 第五步：验证安装

再次运行环境检查：

```bash
# Windows
.\scripts\setup.bat check

# Linux/macOS
./scripts/setup.sh check
```

所有工具应该显示 [✓] 标记。

## 使用 Towa CLI

构建完成后，您可以使用 `towa-cli` 命令：

```bash
# 检查环境
towa-cli check

# 安装工具
towa-cli install <tool>  # tool: all, c, rust, cepton

# 配置 Cepton
towa-cli cepton --path /path/to/sdk

# 初始化配置
towa-cli init

# 查看版本
towa-cli version
```

## 示例工作流

### 场景 1：新电脑设置 C++ 开发环境

```bash
git clone https://github.com/yourusername/towa.git
cd towa
./scripts/setup.sh install
./scripts/setup.sh check
```

### 场景 2：为 Rust 项目配置环境

```bash
cd towa
./scripts/setup.sh rust
./scripts/setup.sh build
```

### 场景 3：配置 Cepton 传感器开发

```bash
# 1. 安装基础工具
./scripts/setup.sh install

# 2. 下载 Cepton SDK 并解压

# 3. 配置 SDK
./scripts/setup.sh cepton

# 4. 验证
./scripts/setup.sh check
```

## 常见使用场景

### 测试环境是否就绪

```bash
# 使用 C 工具检测
./src/c/build/bin/env_checker

# 使用 Rust CLI
./src/rust/target/release/towa-cli check
```

### 更新工具

```bash
# 更新 Rust
rustup update

# 重新构建 Towa
./scripts/setup.sh build
```

### 卸载

```bash
# 删除构建产物
rm -rf src/c/build
rm -rf src/rust/target

# 删除配置
rm -rf ~/.towa
```

## 故障排除

### 问题：命令未找到

**解决:**
- Windows: 重启终端以加载新的 PATH
- Linux/macOS: 运行 `source ~/.bashrc` 或重启终端

### 问题：权限被拒绝

**解决:**
```bash
# Linux/macOS
chmod +x scripts/setup.sh
sudo ./scripts/setup.sh install
```

### 问题：网络连接失败

**解决:**
- 使用代理或 VPN
- 中国用户可配置镜像源（见 [BUILDING.md](BUILDING.md)）

## 下一步

- 阅读完整文档: [README.md](../README.md)
- 构建说明: [BUILDING.md](BUILDING.md)
- Cepton 配置: [CEPTON_SETUP.md](CEPTON_SETUP.md)

## 获取帮助

- GitHub Issues: https://github.com/yourusername/towa/issues
- 文档: https://github.com/yourusername/towa/docs
- Email: your.email@example.com

祝您使用愉快！
