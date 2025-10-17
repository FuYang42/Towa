# Towa - Development Environment Quick Setup Tool

> 🌏 [中文](README.zh-CN.md) | **English** | [日本語](README.ja.md)

Towa is a comprehensive toolkit for quickly setting up development environments on fresh computers or systems, specifically designed for:
- C/C++ development environment
- Rust development environment
- Cepton sensor debugging environment
- **Cepton Unified Firmware (UFB) development environment**

## Project Structure

```
Towa/
├── src/
│   ├── c/          # System tools written in C
│   └── rust/       # Configuration management tools in Rust
├── scripts/        # Automated installation scripts
├── docs/           # Documentation
└── README.md
```

## Features

### 1. Environment Detection
- Automatically detect OS type and version
- Check installed development tools and dependencies
- Generate environment reports

### 2. Development Tool Installation
- **C/C++ Toolchain**: GCC, Clang, CMake, Make
- **Rust Toolchain**: rustup, cargo, rustfmt, clippy
- **Version Control**: Git
- **Editor**: VS Code (optional)

### 3. Cepton SDK Configuration
- Auto-download and install Cepton SDK
- Configure environment variables
- Compile sample programs
- Sensor connection testing

### 4. Cepton UFB Development Environment
- **One-click UFB dependency installation**: CMake, Ninja, Python 3, Rust
- **ARM toolchain configuration guide**: LLVM+Clang Embedded Toolchain
- **Git configuration optimization**: Symlinks, CRLF handling
- **Pre-commit hooks**: Code quality checks
- **Detailed documentation**: Complete UFB environment setup guide

## Quick Start

### Windows

```bash
# Run environment check
.\scripts\setup.bat check

# Auto-install all tools
.\scripts\setup.bat install

# Install Cepton SDK only
.\scripts\setup.bat cepton

# Setup UFB development environment (recommended for firmware development)
towa-cli ufb
```

### Linux/macOS

```bash
# Run environment check
./scripts/setup.sh check

# Auto-install all tools
./scripts/setup.sh install

# Install Cepton SDK only
./scripts/setup.sh cepton

# Setup UFB development environment (recommended for firmware development)
towa-cli ufb
```

## UFB Firmware Development Quick Setup

If you need to develop Cepton firmware (Nova, Ultra, etc.), follow these steps:

```bash
# 1. Install basic tools
./scripts/setup.sh install

# 2. Setup UFB environment
cd src/rust
cargo build --release
./target/release/towa-cli ufb

# 3. Follow prompts to install ARM toolchain

# 4. Verify environment
./target/release/towa-cli check

# 5. View detailed documentation
# See docs/CEPTON_UFB_SETUP.md
```

## Tool Description

### C Tools
- **env_checker**: Environment detection tool for quick system configuration scanning
- **pkg_installer**: Package manager wrapper providing unified installation interface

### Rust Tools
- **towa-cli**: Main CLI program providing interactive configuration wizard
- **config-manager**: Configuration file manager

## Building the Project

### Build C Tools

```bash
cd src/c
mkdir build && cd build
cmake ..
make
```

### Build Rust Tools

```bash
cd src/rust
cargo build --release
```

## Supported Platforms

- Windows 10/11
- Ubuntu 20.04+
- macOS 11+

## Documentation

- **[Quick Start Guide](docs/QUICKSTART.md)** - Get started in 5 minutes
- **[Building Instructions](docs/BUILDING.md)** - Detailed compilation and installation guide
- **[Cepton SDK Setup](docs/CEPTON_SETUP.md)** - Cepton sensor SDK configuration
- **[Cepton UFB Environment Setup](docs/CEPTON_UFB_SETUP.md)** - Complete firmware development environment guide ⭐
- **[Project Technical Overview](PROJECT_OVERVIEW.md)** - Project architecture and technical details

## Common Commands (towa-cli)

After building the Rust CLI, you can use the following commands:

```bash
# Environment check
towa-cli check

# Install all tools
towa-cli install all

# Install C toolchain only
towa-cli install c

# Install Rust toolchain only
towa-cli install rust

# Configure Cepton SDK
towa-cli cepton

# Setup UFB development environment
towa-cli ufb

# Initialize configuration file
towa-cli init

# Show version
towa-cli version
```

## Contributing

Issues and Pull Requests are welcome!

## License

MIT License
