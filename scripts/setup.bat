@echo off
REM Towa Setup Script for Windows
REM Author: Towa Contributors

setlocal enabledelayedexpansion

echo ============================================
echo    Towa Environment Setup (Windows)
echo ============================================
echo.

if "%1"=="" (
    echo Usage: setup.bat [command]
    echo.
    echo Commands:
    echo   check      - Check current environment
    echo   install    - Install all development tools
    echo   c          - Install C/C++ toolchain
    echo   rust       - Install Rust toolchain
    echo   cepton     - Configure Cepton SDK
    echo   build      - Build Towa tools
    echo   help       - Show this help message
    goto :eof
)

if "%1"=="check" goto check
if "%1"=="install" goto install_all
if "%1"=="c" goto install_c
if "%1"=="rust" goto install_rust
if "%1"=="cepton" goto install_cepton
if "%1"=="build" goto build
if "%1"=="help" goto help
goto :eof

:check
echo [*] Checking environment...
echo.

REM Check if towa-cli is built
if exist "src\rust\target\release\towa-cli.exe" (
    src\rust\target\release\towa-cli.exe check
) else if exist "src\rust\target\debug\towa-cli.exe" (
    src\rust\target\debug\towa-cli.exe check
) else (
    echo [!] Towa CLI not found. Building from source...
    call :build_rust
    if exist "src\rust\target\release\towa-cli.exe" (
        src\rust\target\release\towa-cli.exe check
    )
)
goto :eof

:install_all
echo [*] Installing all development tools...
echo.

call :install_c
call :install_rust
call :build

echo.
echo [+] Installation complete!
echo [*] Run 'setup.bat check' to verify installation
goto :eof

:install_c
echo [*] Installing C/C++ toolchain...
echo.

REM Check if winget is available
where winget >nul 2>&1
if %errorlevel% equ 0 (
    echo [*] Using winget to install tools...

    REM Install MSYS2
    echo [*] Installing MSYS2...
    winget install -e --id MSYS2.MSYS2 --silent

    REM Install CMake
    echo [*] Installing CMake...
    winget install -e --id Kitware.CMake --silent

    echo [+] Please run MSYS2 and execute:
    echo     pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-cmake mingw-w64-x86_64-make
) else (
    echo [!] winget not found. Please install:
    echo     1. MSYS2 from https://www.msys2.org/
    echo     2. CMake from https://cmake.org/download/
)
goto :eof

:install_rust
echo [*] Installing Rust toolchain...
echo.

REM Check if Rust is already installed
where rustc >nul 2>&1
if %errorlevel% equ 0 (
    echo [+] Rust is already installed. Updating...
    rustup update
) else (
    where winget >nul 2>&1
    if %errorlevel% equ 0 (
        echo [*] Installing Rust via winget...
        winget install -e --id Rustlang.Rustup --silent
    ) else (
        echo [!] Please download and install Rust from:
        echo     https://rustup.rs/
    )
)
goto :eof

:install_cepton
echo [*] Configuring Cepton SDK...
echo.

if exist "src\rust\target\release\towa-cli.exe" (
    src\rust\target\release\towa-cli.exe cepton
) else (
    echo [!] Please download Cepton SDK from:
    echo     https://www.cepton.com/downloads
    echo.
    echo [!] Then set environment variable:
    echo     setx CEPTON_SDK_PATH "C:\Program Files\CeptonSDK"
)
goto :eof

:build
echo [*] Building Towa tools...
echo.

call :build_c
call :build_rust
goto :eof

:build_c
echo [*] Building C tools...
cd src\c

if not exist "build" mkdir build
cd build

cmake ..
cmake --build . --config Release

cd ..\..\..
echo [+] C tools built successfully
goto :eof

:build_rust
echo [*] Building Rust tools...
cd src\rust

cargo build --release

cd ..\..
echo [+] Rust tools built successfully
goto :eof

:help
echo Towa Setup Script
echo.
echo Commands:
echo   check      - Check current environment status
echo   install    - Install all development tools
echo   c          - Install C/C++ toolchain only
echo   rust       - Install Rust toolchain only
echo   cepton     - Configure Cepton SDK
echo   build      - Build all Towa tools
echo   help       - Show this help message
echo.
echo Examples:
echo   setup.bat check
echo   setup.bat install
echo   setup.bat build
goto :eof

endlocal
