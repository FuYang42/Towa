# Towa 构建指南

本文档介绍如何从源代码构建 Towa 项目。

## 系统要求

### Windows
- Windows 10 或更高版本
- Visual Studio 2019+ 或 MinGW-w64
- CMake 3.10+
- Rust 1.70+

### Linux
- Ubuntu 20.04+ / Debian 11+ / Fedora 36+ / Arch Linux
- GCC 9+ 或 Clang 10+
- CMake 3.10+
- Rust 1.70+

### macOS
- macOS 11 (Big Sur) 或更高版本
- Xcode Command Line Tools
- CMake 3.10+
- Rust 1.70+

## 快速构建

### 使用自动化脚本

**Windows:**
```batch
.\scripts\setup.bat build
```

**Linux/macOS:**
```bash
./scripts/setup.sh build
```

## 手动构建

### 1. 构建 C 工具

```bash
cd src/c
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

构建产物位于 `src/c/build/bin/` 目录：
- `env_checker` - 环境检测工具

### 2. 构建 Rust 工具

```bash
cd src/rust
cargo build --release
```

构建产物位于 `src/rust/target/release/` 目录：
- `towa-cli` - 主命令行工具

## 测试构建

### 运行环境检测

**Windows:**
```batch
src\c\build\bin\Release\env_checker.exe
```

**Linux/macOS:**
```bash
./src/c/build/bin/env_checker
```

### 运行 Towa CLI

**Windows:**
```batch
src\rust\target\release\towa-cli.exe check
```

**Linux/macOS:**
```bash
./src/rust/target/release/towa-cli check
```

## 开发构建

### Debug 模式

**C 工具:**
```bash
cd src/c/build
cmake -DCMAKE_BUILD_TYPE=Debug ..
cmake --build .
```

**Rust 工具:**
```bash
cd src/rust
cargo build  # 默认为 debug 模式
```

### 运行测试

**Rust 测试:**
```bash
cd src/rust
cargo test
```

## 交叉编译

### 为 Linux 构建（在 Windows 上）

使用 WSL2 或 Docker：

```bash
# 使用 Docker
docker run --rm -v $(pwd):/workspace -w /workspace rust:latest \
  bash -c "cd src/rust && cargo build --release --target x86_64-unknown-linux-gnu"
```

### 为 Windows 构建（在 Linux 上）

```bash
# 安装 Windows 工具链
rustup target add x86_64-pc-windows-gnu

cd src/rust
cargo build --release --target x86_64-pc-windows-gnu
```

## 安装到系统

### Linux/macOS

```bash
# 安装 C 工具
cd src/c/build
sudo cmake --install .

# 安装 Rust 工具
cd src/rust
cargo install --path .
```

### Windows

将构建产物复制到 PATH 中的目录，或添加到系统 PATH：

```batch
# 添加到 PATH
setx PATH "%PATH%;C:\path\to\Towa\src\rust\target\release"
```

## 常见构建问题

### CMake 找不到编译器

**解决方法:**
```bash
# 指定编译器
cmake -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++ ..
```

### Rust 依赖下载失败

**解决方法:**
```bash
# 使用国内镜像（中国用户）
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 配置 cargo 镜像
cat >> ~/.cargo/config.toml <<EOF
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

### Windows 上链接错误

**解决方法:**
- 确保已安装 Visual Studio Build Tools
- 或使用 MSYS2/MinGW-w64

### macOS 上权限问题

**解决方法:**
```bash
# 给予执行权限
chmod +x scripts/setup.sh
chmod +x src/c/build/bin/env_checker
```

## 清理构建

### C 工具
```bash
rm -rf src/c/build
```

### Rust 工具
```bash
cd src/rust
cargo clean
```

## 性能优化

### 优化编译选项

**CMake:**
```bash
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_FLAGS="-O3 -march=native" ..
```

**Rust:**
在 `Cargo.toml` 中添加：
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Build C tools
      run: |
        cd src/c
        mkdir build && cd build
        cmake ..
        cmake --build . --config Release

    - name: Build Rust tools
      run: |
        cd src/rust
        cargo build --release
```

## 更多帮助

如果遇到构建问题，请：
1. 查看 [README.md](../README.md)
2. 访问 [GitHub Issues](https://github.com/yourusername/towa/issues)
3. 确保所有依赖项已正确安装
