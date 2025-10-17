# Cepton UFB 开发环境快速配置

本指南帮助您在 5-10 分钟内完成 Cepton Unified Firmware (UFB) 开发环境的基本配置。

## 前提条件

- 已安装 Git
- 有网络连接（用于下载工具）
- 管理员/sudo 权限（用于安装软件）

---

## 方法 1: 使用 Towa 自动配置（推荐）

### 步骤 1: 获取 Towa

```bash
git clone https://github.com/FuYang42/Towa.git
cd Towa
```

### 步骤 2: 安装基础工具

**Windows:**
```batch
.\scripts\setup.bat install
```

**Linux/macOS:**
```bash
./scripts/setup.sh install
```

### 步骤 3: 构建 Towa CLI

```bash
cd src/rust
cargo build --release
cd ../..
```

### 步骤 4: 配置 UFB 环境

```bash
# Windows
src\rust\target\release\towa-cli.exe ufb

# Linux/macOS
./src/rust/target/release/towa-cli ufb
```

这个命令会：
- ✅ 检测并安装 CMake 和 Ninja
- ✅ 检测并安装 Python 3
- ✅ 检测并安装 Rust 工具链
- ✅ 提供 ARM 工具链安装指导
- ✅ 显示 Git 配置建议

### 步骤 5: 安装 ARM 工具链（手动）

#### 5.1 下载

访问: https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm/releases

下载 `ATfE-20.1.0` 对应您平台的版本：
- Windows: `ATfE-20.1.0-Windows-x86_64.zip`
- Linux: `ATfE-20.1.0-Linux-x86_64.tar.xz`
- macOS: `ATfE-20.1.0-Darwin-universal.dmg`

#### 5.2 安装

**Windows:**
```batch
# 解压到桌面
# 重命名文件夹为: ATfE-20.1.0 (移除 -Windows-x86_64)

# 创建目标目录
mkdir C:\opt\arm

# 移动文件夹
move Desktop\ATfE-20.1.0 C:\opt\arm\
```

**Linux/macOS:**
```bash
# 解压
tar -xf ATfE-20.1.0-Linux-x86_64.tar.xz  # Linux
# 或挂载 DMG (macOS)

# 重命名并移动
sudo mkdir -p /opt/arm
sudo mv ATfE-20.1.0-* ATfE-20.1.0
sudo mv ATfE-20.1.0 /opt/arm/
```

### 步骤 6: 配置 Git

```bash
# 推荐配置
git config --global pull.rebase true
git config --global core.symlinks true
git config --global core.autocrlf false

# 验证
git config --list | grep -E "symlinks|autocrlf|rebase"
```

**Windows 额外步骤**: 在系统设置中启用开发者模式。

### 步骤 7: 验证安装

```bash
# 检查所有工具
towa-cli check

# 应该看到:
# [✓] CMake
# [✓] Ninja  (可能需要重启终端)
# [✓] Python
# [✓] Rust
# [✓] Cargo
# [✓] Git
```

### 步骤 8: 克隆并构建 UFB

```bash
# 克隆 unified_firmware 仓库
git clone <your_ufb_repo_url>
cd unified_firmware/ufb_builder

# 列出所有可用模块
cargo run -r -- list

# 构建 SIL 模块（测试）
cargo run -r -- build sil -r

# 构建 Nova 固件
cargo run -r -- build nova -r
```

### 完成！

如果 SIL 构建成功，说明您的环境已正确配置。

---

## 方法 2: 手动配置

如果您不想使用 Towa，可以手动安装：

### Windows

```batch
# 1. 安装 CMake
winget install Kitware.CMake

# 2. 安装 Ninja
winget install Ninja-build.Ninja

# 3. 安装 Python 3
winget install Python.Python.3.10

# 4. 安装 Rust
winget install Rustlang.Rustup

# 5. 重启终端

# 6. 创建 python3 符号链接
mklink "C:\Program Files\Python310\python3.exe" "C:\Program Files\Python310\python.exe"

# 7. 配置 Git
git config --global core.symlinks true
git config --global core.autocrlf false
```

### Ubuntu/Debian

```bash
# 1. 安装所有依赖
sudo apt update
sudo apt install -y cmake ninja-build libpcap-dev python3 python3-pip git

# 2. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. 配置 Git
git config --global pull.rebase true
git config --global core.symlinks true
```

### macOS

```bash
# 1. 安装 Homebrew (如果没有)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. 安装依赖
brew install cmake ninja python3 git

# 3. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. 配置 Git
git config --global pull.rebase true
git config --global core.symlinks true
```

然后按照方法 1 的步骤 5 安装 ARM 工具链。

---

## 常见命令

### UFB Builder 常用命令

```bash
cd unified_firmware/ufb_builder

# 列出所有模块
cargo run -r -- list

# 构建模块（release 模式）
cargo run -r -- build <module> -r

# 清理构建
cargo run -r -- build <module> -r --clean

# 跳过 setup（快速重新编译）
cargo run -r -- build <module> -r --skip-setup
```

### 环境检查

```bash
# 使用 Towa
towa-cli check

# 或手动检查
cmake --version
ninja --version
python3 --version
rustc --version
cargo --version
git --version
```

---

## 故障排除

### Python3 命令未找到 (Windows)

```batch
# 检查 Python 是否安装
python --version

# 创建符号链接
mklink "C:\Program Files\Python310\python3.exe" "C:\Program Files\Python310\python.exe"
```

### 符号链接错误 (Windows)

1. 启用开发者模式: Settings → Update & Security → For developers → Developer Mode
2. 运行: `git config --global core.symlinks true`
3. 删除并重新克隆仓库

### ARM 工具链未找到

检查路径：
- Windows: `C:\opt\arm\ATfE-20.1.0\bin\clang.exe`
- Linux/macOS: `/opt/arm/ATfE-20.1.0/bin/clang`

确保文件夹名称正确（无平台后缀）。

### Ninja 未找到

重启终端以加载新的 PATH 环境变量。

---

## 下一步

1. **阅读 UFB 文档**:
   - [UFB README](../unified_firmware/ufb/README.md)
   - [UFB 语言指南](../unified_firmware/ufb/docs/guide.md)

2. **尝试构建不同模块**:
   ```bash
   cargo run -r -- build sil -r      # Software in the Loop
   cargo run -r -- build nova -r     # Nova firmware
   cargo run -r -- build ultra_clang -r  # Ultra with Clang
   ```

3. **设置代码检查工具**:
   ```bash
   pip3 install pre-commit pylint
   cargo install typos-cli
   cd unified_firmware
   pre-commit install
   ```

4. **参考详细文档**:
   - [完整 UFB 配置指南](CEPTON_UFB_SETUP.md)

---

## 时间估算

- **使用 Towa 自动配置**: 5-10 分钟（不含 ARM 工具链下载）
- **手动配置**: 15-30 分钟
- **首次构建 Nova**: 5-10 分钟（取决于硬件）

---

## 获取帮助

- 查看 [CEPTON_UFB_SETUP.md](CEPTON_UFB_SETUP.md) 获取详细说明
- 提交 Issue: https://github.com/FuYang42/Towa/issues
- 联系 Cepton 固件团队

---

**提示**: 强烈建议使用 Towa 自动配置，它会自动检测您的系统并安装正确的工具。
