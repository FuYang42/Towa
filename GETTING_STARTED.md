# 🚀 Towa 使用指南 - 3 步开始

> 💡 **超级简单！只需 3 步，5 分钟完成环境配置**

---

## 📦 使用方式

### 方式一：快速安装（推荐）⭐

#### 第 1 步：下载 Towa

```bash
git clone https://github.com/FuYang42/Towa.git
cd Towa
```

#### 第 2 步：运行安装脚本

**Windows 用户:**
```batch
.\scripts\setup.bat install
```

**Linux/macOS 用户:**
```bash
./scripts/setup.sh install
```

#### 第 3 步：验证安装

```bash
# Windows
.\scripts\setup.bat check

# Linux/macOS
./scripts/setup.sh check
```

✅ **完成！** 现在您的系统已经配置好开发环境了！

---

### 方式二：按需安装

如果您只需要特定工具：

```bash
# 只检查环境（不安装任何东西）
./scripts/setup.sh check

# 只安装 C/C++ 工具
./scripts/setup.sh c

# 只安装 Rust 工具
./scripts/setup.sh rust

# 只配置 Cepton SDK
./scripts/setup.sh cepton

# 配置 UFB 开发环境
cd src/rust && cargo build --release
./target/release/towa-cli ufb
```

---

## 🎯 常见使用场景

### 场景 1: 新电脑配置开发环境

```bash
# 一键配置
git clone https://github.com/FuYang42/Towa.git
cd Towa
./scripts/setup.sh install
```

**安装内容:**
- ✅ C/C++ 编译器 (GCC/Clang)
- ✅ CMake 和 Make
- ✅ Rust 工具链
- ✅ Git
- ✅ 其他开发工具

---

### 场景 2: Cepton 固件开发

```bash
# 1. 基础安装
git clone https://github.com/FuYang42/Towa.git
cd Towa
./scripts/setup.sh install

# 2. UFB 环境配置
cd src/rust
cargo build --release
./target/release/towa-cli ufb

# 3. 按照提示安装 ARM 工具链

# 4. 验证
./target/release/towa-cli check
```

**查看详细文档:** [docs/CEPTON_UFB_SETUP.md](docs/CEPTON_UFB_SETUP.md)

---

### 场景 3: 只需要环境检测

```bash
git clone https://github.com/FuYang42/Towa.git
cd Towa
./scripts/setup.sh check
```

这会显示您系统上已安装和缺失的工具。

---

## 📖 Towa 能做什么？

| 功能 | 说明 | 命令 |
|------|------|------|
| 🔍 **环境检测** | 检查已安装的开发工具 | `./scripts/setup.sh check` |
| 🛠️ **安装工具** | 自动安装 C/C++, Rust 等工具 | `./scripts/setup.sh install` |
| 📡 **配置 Cepton SDK** | 设置 Cepton 传感器开发环境 | `./scripts/setup.sh cepton` |
| ⚙️ **UFB 环境** | 配置固件开发环境 | `towa-cli ufb` |

---

## 🎮 使用 Towa CLI

构建 Rust 工具后，您可以使用更强大的 `towa-cli`:

```bash
# 构建 CLI
cd src/rust
cargo build --release

# 使用 CLI
./target/release/towa-cli check              # 环境检测
./target/release/towa-cli install all        # 安装所有工具
./target/release/towa-cli install c          # 只安装 C 工具
./target/release/towa-cli install rust       # 只安装 Rust 工具
./target/release/towa-cli cepton             # 配置 Cepton SDK
./target/release/towa-cli ufb                # 配置 UFB 环境
./target/release/towa-cli init               # 初始化配置文件
./target/release/towa-cli version            # 查看版本
```

---

## ⏱️ 时间估算

| 任务 | 时间 |
|------|------|
| 克隆项目 | < 1 分钟 |
| 运行安装脚本 | 2-5 分钟 |
| 验证环境 | < 1 分钟 |
| **总计** | **约 5 分钟** |

---

## 🆘 遇到问题？

### 问题 1: 脚本无法执行

**Windows:**
```batch
# 确保使用管理员权限
# 右键 → 以管理员身份运行
```

**Linux/macOS:**
```bash
chmod +x scripts/setup.sh
./scripts/setup.sh install
```

### 问题 2: 命令未找到

```bash
# 重启终端
# 或运行
source ~/.bashrc  # Linux
source ~/.zshrc   # macOS
```

### 问题 3: 需要帮助

- 📖 [完整文档](docs/QUICKSTART.md)
- 🐛 [报告问题](https://github.com/FuYang42/Towa/issues)
- 💬 [GitHub Discussions](https://github.com/FuYang42/Towa/discussions)

---

## 📚 进一步学习

- **[README.md](README.md)** - 项目总览
- **[docs/QUICKSTART.md](docs/QUICKSTART.md)** - 详细快速开始指南
- **[docs/CEPTON_UFB_SETUP.md](docs/CEPTON_UFB_SETUP.md)** - UFB 开发环境配置
- **[docs/BUILDING.md](docs/BUILDING.md)** - 从源代码构建

---

## 🎉 就是这么简单！

```
克隆项目 → 运行脚本 → 开始使用
   1分钟       5分钟       立即
```

<div align="center">

**有问题？[提交 Issue](https://github.com/FuYang42/Towa/issues)**

**觉得有用？[给个 Star ⭐](https://github.com/FuYang42/Towa)**

</div>
