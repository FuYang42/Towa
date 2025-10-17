# Cepton SDK 配置指南

本文档介绍如何在不同操作系统上配置 Cepton 传感器开发环境。

## 前置要求

- C/C++ 编译器（GCC 或 MSVC）
- CMake 3.10+
- Git

## 下载 SDK

1. 访问 Cepton 官方网站：https://www.cepton.com/downloads
2. 根据您的操作系统下载对应版本的 SDK
3. 解压到合适的位置

## Windows 配置

### 1. 安装路径

推荐安装路径：
```
C:\Program Files\CeptonSDK
```

### 2. 设置环境变量

打开 PowerShell 或命令提示符（管理员权限）：

```batch
setx CEPTON_SDK_PATH "C:\Program Files\CeptonSDK"
setx PATH "%PATH%;C:\Program Files\CeptonSDK\bin"
```

### 3. 验证安装

```batch
echo %CEPTON_SDK_PATH%
```

### 4. 使用 Towa 自动配置

```batch
.\scripts\setup.bat cepton
```

## Linux 配置

### 1. 安装路径

推荐安装路径：
```
/opt/cepton_sdk
```

### 2. 解压 SDK

```bash
sudo mkdir -p /opt/cepton_sdk
sudo tar -xzf cepton_sdk_linux.tar.gz -C /opt/cepton_sdk
```

### 3. 设置环境变量

编辑 `~/.bashrc` 或 `~/.zshrc`：

```bash
export CEPTON_SDK_PATH=/opt/cepton_sdk
export PATH=$PATH:$CEPTON_SDK_PATH/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CEPTON_SDK_PATH/lib
```

应用更改：
```bash
source ~/.bashrc
```

### 4. 使用 Towa 自动配置

```bash
./scripts/setup.sh cepton
```

## macOS 配置

### 1. 安装路径

推荐安装路径：
```
/usr/local/cepton_sdk
```

### 2. 解压 SDK

```bash
sudo mkdir -p /usr/local/cepton_sdk
sudo tar -xzf cepton_sdk_macos.tar.gz -C /usr/local/cepton_sdk
```

### 3. 设置环境变量

编辑 `~/.zshrc` 或 `~/.bash_profile`：

```bash
export CEPTON_SDK_PATH=/usr/local/cepton_sdk
export PATH=$PATH:$CEPTON_SDK_PATH/bin
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$CEPTON_SDK_PATH/lib
```

应用更改：
```bash
source ~/.zshrc
```

## 编译示例程序

### 使用 CMake

```bash
cd $CEPTON_SDK_PATH/examples
mkdir build && cd build
cmake ..
cmake --build .
```

### 运行示例

```bash
# Windows
.\build\cepton_viewer.exe

# Linux/macOS
./build/cepton_viewer
```

## 常见问题

### Q: 找不到库文件

**A:** 确保已正确设置环境变量，特别是 `LD_LIBRARY_PATH` (Linux) 或 `PATH` (Windows)。

### Q: 传感器连接失败

**A:** 检查以下几点：
1. 传感器是否正确连接到网络
2. 防火墙是否允许相应端口
3. SDK 版本是否与传感器固件版本兼容

### Q: 编译错误

**A:** 确认：
1. 已安装所有依赖项
2. CMake 版本 >= 3.10
3. 编译器支持 C++11 或更高版本

## 测试传感器连接

使用 Towa 的环境检测工具：

```bash
# Windows
.\scripts\setup.bat check

# Linux/macOS
./scripts/setup.sh check
```

## 固件更新

Cepton 传感器支持通过统一固件库进行更新：

1. 下载最新固件包
2. 使用 SDK 提供的固件更新工具
3. 按照提示完成更新过程

```bash
# 示例命令
cepton_firmware_updater --ip <传感器IP> --firmware <固件文件路径>
```

## 更多资源

- [Cepton 官方文档](https://www.cepton.com/documentation)
- [SDK API 参考](https://www.cepton.com/api-reference)
- [社区论坛](https://forum.cepton.com)

## 技术支持

如遇问题，请联系：
- Cepton 技术支持: support@cepton.com
- Towa 项目: [GitHub Issues](https://github.com/yourusername/towa/issues)
