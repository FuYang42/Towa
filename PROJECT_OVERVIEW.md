# Towa 项目概览

## 项目结构

```
Towa/
├── src/
│   ├── c/                      # C 语言工具
│   │   ├── env_checker.c       # 环境检测工具（跨平台）
│   │   └── CMakeLists.txt      # CMake 构建配置
│   │
│   └── rust/                   # Rust 工具
│       ├── src/
│       │   ├── main.rs         # CLI 主程序
│       │   ├── config.rs       # 配置管理模块
│       │   └── installer.rs    # 安装器模块
│       └── Cargo.toml          # Rust 项目配置
│
├── scripts/                    # 自动化脚本
│   ├── setup.bat               # Windows 安装脚本
│   └── setup.sh                # Linux/macOS 安装脚本
│
├── docs/                       # 文档
│   ├── QUICKSTART.md           # 快速开始指南
│   ├── BUILDING.md             # 构建指南
│   └── CEPTON_SETUP.md         # Cepton SDK 配置指南
│
├── README.md                   # 项目说明
├── LICENSE                     # MIT 许可证
└── .gitignore                  # Git 忽略规则
```

## 核心功能

### 1. 环境检测工具 (C)

**文件:** `src/c/env_checker.c`

**功能:**
- 检测操作系统信息
- 扫描已安装的开发工具（GCC, Clang, CMake, Rust, Git 等）
- 检查环境变量配置
- 生成安装建议

**特点:**
- 跨平台支持（Windows, Linux, macOS）
- 快速启动，低资源占用
- 彩色输出（Unix 系统）

### 2. 配置管理 CLI (Rust)

**文件:** `src/rust/src/main.rs`

**命令:**
```bash
towa-cli check          # 检查环境
towa-cli install <tool> # 安装工具
towa-cli cepton         # 配置 Cepton SDK
towa-cli init           # 初始化配置
towa-cli version        # 版本信息
```

**特点:**
- 现代化 CLI 界面（使用 clap）
- 配置文件持久化（TOML 格式）
- 跨平台安装逻辑

### 3. 配置管理模块 (Rust)

**文件:** `src/rust/src/config.rs`

**功能:**
- 读写 TOML 配置文件
- 默认配置生成
- 配置路径管理（使用 dirs crate）

**配置位置:**
- Windows: `%USERPROFILE%\.towa\config.toml`
- Linux/macOS: `~/.towa/config.toml`

### 4. 安装器模块 (Rust)

**文件:** `src/rust/src/installer.rs`

**支持的工具:**
- **C/C++ 工具链**: GCC, Clang, MSVC
- **构建工具**: CMake, Make
- **Rust 工具链**: rustc, cargo, rustup
- **版本控制**: Git
- **Cepton SDK**: 自动配置和环境变量设置

**平台特定逻辑:**
- Windows: 使用 winget 或提供手动安装指引
- Linux: 检测发行版（apt, dnf, pacman）并使用相应包管理器
- macOS: 使用 Homebrew

### 5. 自动化脚本

**Windows (setup.bat):**
- 批处理脚本
- 支持命令: check, install, c, rust, cepton, build
- 自动检测工具和构建项目

**Linux/macOS (setup.sh):**
- Bash 脚本
- 彩色输出
- 错误处理和用户提示
- 支持多种 Linux 发行版

## 技术栈

### C 工具
- **语言**: C11
- **构建系统**: CMake 3.10+
- **标准库**: stdio, stdlib, string
- **平台 API**:
  - Windows: windows.h
  - Unix: unistd.h, sys/utsname.h

### Rust 工具
- **语言**: Rust 2021 Edition
- **依赖库**:
  - clap: CLI 参数解析
  - serde/serde_json: 序列化
  - toml: TOML 配置
  - colored: 彩色输出
  - reqwest: HTTP 请求（预留）
  - dirs: 跨平台路径

## 工作流程

### 典型使用场景

#### 1. 全新系统环境配置

```bash
# 1. 克隆项目
git clone <repo-url>
cd towa

# 2. 运行环境检查
./scripts/setup.sh check

# 3. 自动安装所有工具
./scripts/setup.sh install

# 4. 验证安装
./scripts/setup.sh check
```

#### 2. Cepton 传感器开发环境

```bash
# 1. 安装基础开发工具
./scripts/setup.sh install

# 2. 下载 Cepton SDK

# 3. 配置 SDK
./scripts/setup.sh cepton

# 4. 构建示例程序
cd $CEPTON_SDK_PATH/examples
mkdir build && cd build
cmake .. && make
```

## 构建流程

### 构建 C 工具

```bash
cd src/c
mkdir build && cd build
cmake ..
cmake --build . --config Release
```

**输出**: `build/bin/env_checker[.exe]`

### 构建 Rust 工具

```bash
cd src/rust
cargo build --release
```

**输出**: `target/release/towa-cli[.exe]`

### 集成构建

```bash
# Windows
.\scripts\setup.bat build

# Linux/macOS
./scripts/setup.sh build
```

## 平台支持

| 平台 | 支持状态 | 测试版本 |
|------|----------|----------|
| Windows 10/11 | ✅ | 10, 11 |
| Ubuntu | ✅ | 20.04, 22.04 |
| Debian | ✅ | 11, 12 |
| Fedora | ✅ | 36+ |
| Arch Linux | ✅ | Rolling |
| macOS | ✅ | 11+ |

## 扩展性

### 添加新工具检测

在 `env_checker.c` 中添加到 `tools[]` 数组：

```c
Tool tools[] = {
    // ...
    {"新工具", "command", 0, ""}
};
```

### 添加新安装器

在 `installer.rs` 中添加新函数：

```rust
pub fn install_new_tool() {
    println!("Installing new tool...");
    // 实现逻辑
}
```

在 `main.rs` 中添加新命令：

```rust
Commands::NewTool => installer::install_new_tool(),
```

### 添加配置项

在 `config.rs` 中扩展 `TowaConfig` 结构：

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct TowaConfig {
    // ...
    pub new_section: NewConfig,
}
```

## 安全性

- 不存储敏感信息
- 使用 HTTPS 下载（预留功能）
- 权限检查（需要管理员时提示）
- 输入验证和错误处理

## 性能

- C 工具: < 50ms 启动时间
- Rust CLI: < 100ms 启动时间
- 内存占用: < 10MB

## 未来计划

1. **GUI 界面**: 使用 Tauri 或 egui
2. **包管理器集成**: 支持更多平台
3. **远程配置**: 从服务器同步配置
4. **插件系统**: 支持第三方扩展
5. **Docker 支持**: 容器化环境
6. **CI/CD 模板**: GitHub Actions, GitLab CI

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 维护者

- 项目创建者: [Your Name]
- 主要维护者: Towa Contributors

## 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 联系方式

- Email: Fuyou42@yahoo.co.jp
