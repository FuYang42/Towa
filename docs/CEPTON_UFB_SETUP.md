# Cepton Unified Firmware (UFB) 开发环境配置指南

本指南详细说明如何配置 Cepton Unified Firmware Base (UFB) 开发环境。

## 目录

- [1. 环境概述](#1-环境概述)
- [2. 前置要求](#2-前置要求)
- [3. 核心工具安装](#3-核心工具安装)
- [4. ARM 工具链安装](#4-arm-工具链安装)
- [5. 可选工具](#5-可选工具)
- [6. 环境验证](#6-环境验证)
- [7. 常见问题](#7-常见问题)

---

## 1. 环境概述

### 什么是 UFB？

Unified Firmware Base (UFB) 是 Cepton 开发的固件开发框架，用于：
- 模块化嵌入式固件开发
- 跨硬件平台代码共享
- 单元测试和集成测试
- Nova 系列传感器固件编译

### 支持的产品

- Nova B2 传感器
- Ultra 系列传感器
- 其他 Cepton 激光雷达产品

---

## 2. 前置要求

### 操作系统

| 系统 | 版本 | 状态 |
|------|------|------|
| Windows | 10/11 | ✅ 推荐（需WSL2或原生） |
| Ubuntu | 20.04+ | ✅ 推荐 |
| macOS | 11+ | ✅ 支持 |

### 硬件要求

- **CPU**: x86_64 或 Apple Silicon (M1/M2/M3)
- **RAM**: 最少 8GB，推荐 16GB+
- **存储**: 至少 20GB 可用空间
- **网络**: 访问 GitHub 和其他开源仓库

---

## 3. 核心工具安装

### 3.1. CMake 和 Ninja

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install cmake ninja-build libpcap-dev
```

**Windows (使用 winget):**
```batch
winget install Kitware.CMake
winget install Ninja-build.Ninja
```

**macOS (使用 Homebrew):**
```bash
brew install cmake ninja
```

**验证安装:**
```bash
cmake --version    # 应该 >= 3.10
ninja --version
```

### 3.2. Python 3

**Ubuntu/Debian:**
```bash
sudo apt install python3 python3-pip
```

**Windows:**
```batch
# 使用 Windows 安装程序
# https://www.python.org/downloads/

# 创建 python3 符号链接
mklink "C:\Program Files\Python310\python3.exe" "C:\Program Files\Python310\python.exe"
```

**macOS:**
```bash
brew install python3
```

**验证:**
```bash
python3 --version
pip3 --version
```

### 3.3. Rust 和 Cargo

UFB builder 使用 Rust 编写，需要安装 Rust 工具链。

**所有平台 (推荐使用 rustup):**
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 rustup-init.exe
# https://rustup.rs/
```

**验证:**
```bash
rustc --version    # 应该 >= 1.70
cargo --version
```

### 3.4. Git 配置

**重要的 Git 设置:**

```bash
# 使用 rebase 而不是 merge (可选但推荐)
git config --global pull.rebase true

# Windows: 启用符号链接支持
git config --global core.symlinks true

# 关闭自动 CRLF 转换（推荐）
git config --global core.autocrlf false

# 验证配置
git config --list | grep -E "symlinks|autocrlf|rebase"
```

**Windows 用户额外步骤:**
- 启用开发者模式（Settings → Update & Security → For developers → Developer Mode）

---

## 4. ARM 工具链安装

### 4.1. LLVM+Clang Embedded Toolchain for ARM

用于编译嵌入式固件（Nova, Ultra 等）。

#### 下载

访问 ARM 工具链发布页面：
https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm/releases

下载对应平台的版本：
- **Linux**: `ATfE-20.1.0-Linux-x86_64.tar.xz`
- **Windows**: `ATfE-20.1.0-Windows-x86_64.zip`
- **macOS**: `ATfE-20.1.0-Darwin-universal.dmg`

#### 安装步骤

1. **解压缩工具链**

   ```bash
   # Linux 示例
   tar -xf ATfE-20.1.0-Linux-x86_64.tar.xz

   # Windows 示例
   # 使用 7-Zip 或 Windows 资源管理器解压
   ```

2. **重命名文件夹（重要！）**

   移除平台后缀：
   ```bash
   # 从
   ATfE-20.1.0-Windows-x86_64
   # 改为
   ATfE-20.1.0
   ```

3. **移动到标准位置**

   **Linux/macOS:**
   ```bash
   sudo mkdir -p /opt/arm
   sudo mv ATfE-20.1.0 /opt/arm/
   ```

   **Windows:**
   ```batch
   # 创建目录
   mkdir C:\opt\arm

   # 移动工具链
   move ATfE-20.1.0 C:\opt\arm\
   ```

4. **设置环境变量（可选）**

   **Linux/macOS (.bashrc 或 .zshrc):**
   ```bash
   export ARM_TOOLCHAIN_PATH=/opt/arm/ATfE-20.1.0
   export PATH=$PATH:$ARM_TOOLCHAIN_PATH/bin
   ```

   **Windows:**
   ```batch
   setx ARM_TOOLCHAIN_PATH "C:\opt\arm\ATfE-20.1.0"
   setx PATH "%PATH%;C:\opt\arm\ATfE-20.1.0\bin"
   ```

### 4.2. LLVM+Clang Toolchain (用于 Linux 应用)

如果需要为嵌入式 Linux 构建用户空间应用。

#### 下载

访问 LLVM 发布页面：
https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0

下载对应平台的预编译二进制。

#### 安装

```bash
# 解压并移动到标准位置
sudo mkdir -p /opt/arm
sudo mv LLVM-20.1.0 /opt/arm/
```

#### TI Processor SDK (如需要)

从网络共享获取：
```
\\ct2\jenkins\setup\sysroots\ti-processor-sdk-linux-adas-j722s-evm-10_01_00_04.zip
```

解压到：
```bash
/opt/arm/sysroots/ti
```

---

## 5. 可选工具

### 5.1. PCLint Plus (MISRA 检查)

PCLint 用于静态代码分析和 MISRA C 合规性检查。

#### 获取方式

从 Cepton 内部共享获取：
```
\\ct2\jenkins\setup\pclp\
```

#### 安装

1. 解压 pclp 包
2. 复制许可证文件：`CeptonTechnologiesInc-pclp-perpetual.lic`
3. 添加到 PATH 或使用 `--pc-lint` 选项

**Linux 示例:**
```bash
# 解压
tar -xzf pclp-2.0.tar.gz

# 修改权限
chmod -R 755 pclp

# 复制许可证
cp CeptonTechnologiesInc-pclp-perpetual.lic pclp/

# 添加到 PATH
export PATH=$PATH:/path/to/pclp
```

### 5.2. Pre-commit 钩子

安装 pre-commit 框架以自动检查代码质量：

```bash
# 安装 Python 依赖
pip3 install pre-commit pylint

# 安装 Rust 工具
cargo install typos-cli

# 在仓库中启用 pre-commit
cd /path/to/unified_firmware
pre-commit install
```

### 5.3. Wireshark + Cepton 插件

用于分析传感器网络数据包。

#### 安装 Wireshark

**Ubuntu:**
```bash
sudo apt install wireshark
```

**Windows/macOS:**
从官网下载安装: https://www.wireshark.org/

**重要**: Windows 安装时确保包含 Npcap。

#### 安装 Cepton 插件

从 GitHub releases 下载 `cep3_wireshark_plugin`：
https://github.com/ceptontech/cep3_wireshark_plugin/releases

按照插件说明安装。

### 5.4. libpcap (Linux)

用于 SIL (Software in the Loop) 模块：

```bash
sudo apt install libpcap-dev
```

---

## 6. 环境验证

### 6.1. 克隆 Unified Firmware 仓库

```bash
git clone <unified_firmware_repo_url>
cd unified_firmware
```

### 6.2. 使用 UFB Builder

#### 列出所有模块

```bash
cd ufb_builder
cargo run -r -- list
```

#### 构建 Nova 固件

```bash
cargo run -r -- build nova -r
```

参数说明：
- 第一个 `-r`: Cargo release 模式（编译 ufb_builder）
- `build nova`: 构建 nova 模块
- 最后的 `-r`: CMake release 模式（编译固件）

#### 构建 SIL 模块（推荐测试）

```bash
cargo run -r -- build sil -r
```

### 6.3. 使用 Towa 自动检测

如果您已经安装了 Towa：

```bash
# Windows
cd \path\to\Towa
.\scripts\setup.bat check

# Linux/macOS
cd /path/to/Towa
./scripts/setup.sh check
```

Towa 会检测：
- ✅ CMake
- ✅ Ninja
- ✅ Python
- ✅ Rust/Cargo
- ✅ Git
- ✅ Clang (如果在 PATH 中)

---

## 7. 常见问题

### Q1: Windows 符号链接问题

**症状**: `cargo run` 失败，提示符号链接错误

**解决方案**:
1. 启用开发者模式
2. 运行 `git config core.symlinks true`
3. 重新克隆仓库或运行 `git reset --hard`

### Q2: ARM 工具链未找到

**症状**: 构建失败，提示找不到 ARM 编译器

**解决方案**:
- 检查工具链路径: `/opt/arm/ATfE-20.1.0/bin/clang`
- 确认文件夹名称没有平台后缀
- Windows: 检查路径分隔符是否正确

### Q3: Python3 命令未找到 (Windows)

**症状**: `python3` 不是可识别的命令

**解决方案**:
```batch
# 创建符号链接
mklink "C:\Program Files\Python310\python3.exe" "C:\Program Files\Python310\python.exe"

# 或在 PATH 中添加 Python Scripts 目录
```

### Q4: Cargo 构建很慢

**解决方案**:
- 使用中国镜像（国内用户）:
  ```bash
  # ~/.cargo/config.toml
  [source.crates-io]
  replace-with = 'ustc'

  [source.ustc]
  registry = "https://mirrors.ustc.edu.cn/crates.io-index"
  ```

### Q5: 权限被拒绝 (Linux)

**症状**: 无法执行脚本或二进制文件

**解决方案**:
```bash
chmod +x /path/to/script
# 或
sudo chmod -R 755 /opt/arm/ATfE-20.1.0
```

### Q6: 网络路径访问失败

**症状**: 无法访问 `\\ct2\jenkins`

**解决方案**:
- 连接到 Cepton VPN
- Windows: 映射网络驱动器
- Linux: 使用 NFS 挂载:
  ```bash
  mkdir -p ~/jenkins
  sudo mount -t nfs -o resvport,nolocks,soft,intr 10.11.0.13:/mnt/ct2/jenkins ~/jenkins
  ```

---

## 8. Docker 替代方案

如果不想在本地安装所有工具，可以使用 Docker。

### 8.1. 构建 Docker 镜像

```bash
# 连接到 VPN
# 构建镜像
cd unified_firmware
docker buildx build --build-arg LICENSE_USER=your_user -t ceptontooling -f Dockerfile.tooling .
```

### 8.2. 运行 Docker 容器

```bash
docker run --privileged -it \
  -v /path/to/unified_firmware:/code \
  -v /var/cache/jenkins:/var/cache/jenkins:rw \
  --rm ceptontooling
```

### 8.3. 在容器中构建

```bash
# 容器内
cd /code/ufb_builder
cargo run -r -- build nova -r
```

---

## 9. 快速配置脚本

使用 Towa 一键配置 UFB 环境：

```bash
# 克隆 Towa
git clone https://github.com/FuYang42/Towa.git
cd Towa

# 自动安装所有依赖
./scripts/setup.sh install

# 验证环境
./scripts/setup.sh check
```

---

## 10. 下一步

环境配置完成后：

1. **阅读 UFB 文档**:
   - [UFB 语言指南](../unified_firmware/ufb/docs/guide.md)
   - [Nova 固件信息](../unified_firmware/ufb/prods/nova_firmware.md)

2. **尝试构建**:
   ```bash
   cd unified_firmware/ufb_builder
   cargo run -r -- build sil -r
   ```

3. **开始开发**:
   - 创建新模块
   - 修改现有固件
   - 运行单元测试

4. **调试**:
   - 参考 [调试文档](../unified_firmware/documentations/ultra_debugging.md)
   - 使用 `DBG_PRINTF` 输出调试信息

---

## 11. 参考资源

- **Unified Firmware 仓库**: [内部 Git 链接]
- **ARM 工具链文档**: https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm
- **LLVM 文档**: https://llvm.org/docs/
- **Cepton 内部 Wiki**: [SharePoint 链接]
- **Towa 项目**: https://github.com/FuYang42/Towa

---

## 12. 技术支持

遇到问题？

1. 检查此文档的常见问题部分
2. 查阅 unified_firmware 仓库的 README
3. 联系 Cepton 固件团队
4. 提交 Towa Issue: https://github.com/FuYang42/Towa/issues

---

**文档版本**: 1.0
**最后更新**: 2025-10-17
**维护者**: Towa Contributors
