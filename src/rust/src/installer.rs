use colored::*;
use std::env;
use std::process::Command;

pub fn install_c_toolchain() {
    println!("{}", "Installing C/C++ Toolchain...".bold());

    #[cfg(target_os = "windows")]
    {
        println!("{}", "For Windows, please install one of the following:".yellow());
        println!("  1. Visual Studio Build Tools");
        println!("  2. MinGW-w64 (recommended for lightweight installation)");
        println!("  3. MSYS2 with GCC");
        println!();
        println!("Recommended installation command (using winget):");
        println!("  winget install -e --id MSYS2.MSYS2");
        println!();
        println!("After installing MSYS2, run:");
        println!("  pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-cmake mingw-w64-x86_64-make");
    }

    #[cfg(target_os = "linux")]
    {
        println!("Detecting Linux distribution...");

        if Command::new("apt").arg("--version").output().is_ok() {
            println!("Ubuntu/Debian detected. Run:");
            println!("  sudo apt update");
            println!("  sudo apt install build-essential cmake git");
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            println!("Fedora/RHEL detected. Run:");
            println!("  sudo dnf groupinstall 'Development Tools'");
            println!("  sudo dnf install cmake git");
        } else if Command::new("pacman").arg("--version").output().is_ok() {
            println!("Arch Linux detected. Run:");
            println!("  sudo pacman -S base-devel cmake git");
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("macOS detected.");
        if Command::new("brew").arg("--version").output().is_ok() {
            println!("Homebrew found. Installing...");
            run_command("brew", &["install", "gcc", "cmake"]);
        } else {
            println!("Please install Homebrew first:");
            println!("  /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
            println!();
            println!("Then run: towa install c");
        }
    }
}

pub fn install_rust_toolchain() {
    println!("{}", "Installing Rust Toolchain...".bold());

    if Command::new("rustc").arg("--version").output().is_ok() {
        println!("{} Rust is already installed", "✓".green());

        println!("Updating Rust...");
        run_command("rustup", &["update"]);
        return;
    }

    println!("Rust not found. Installing via rustup...");

    #[cfg(target_os = "windows")]
    {
        println!("Please download and run rustup-init.exe from:");
        println!("  https://rustup.rs/");
        println!();
        println!("Or use winget:");
        println!("  winget install -e --id Rustlang.Rustup");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("Running rustup installer...");
        run_command("sh", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"]);

        println!("{}", "Please restart your terminal or run:".yellow());
        println!("  source $HOME/.cargo/env");
    }
}

pub fn install_build_tools() {
    println!("{}", "Installing Build Tools...".bold());

    #[cfg(target_os = "windows")]
    {
        if Command::new("cmake").arg("--version").output().is_err() {
            println!("Installing CMake via winget:");
            run_command("winget", &["install", "-e", "--id", "Kitware.CMake"]);
        }

        if Command::new("git").arg("--version").output().is_err() {
            println!("Installing Git via winget:");
            run_command("winget", &["install", "-e", "--id", "Git.Git"]);
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("Build tools installation is included in C toolchain installation.");
        println!("Run: towa install c");
    }

    #[cfg(target_os = "macos")]
    {
        if Command::new("brew").arg("--version").output().is_ok() {
            run_command("brew", &["install", "cmake", "git"]);
        }
    }
}

pub fn install_cepton_sdk(path: Option<String>) {
    println!("{}", "Setting up Cepton SDK...".bold());

    let sdk_path = if let Some(p) = path {
        p
    } else {
        // 默认路径
        #[cfg(target_os = "windows")]
        let default_path = "C:\\Program Files\\CeptonSDK";

        #[cfg(not(target_os = "windows"))]
        let default_path = "/opt/cepton_sdk";

        println!("Using default SDK path: {}", default_path);
        default_path.to_string()
    };

    println!("{}", "Important Notes:".yellow());
    println!("1. Download Cepton SDK from official website:");
    println!("   https://www.cepton.com/downloads");
    println!();
    println!("2. Extract the SDK to: {}", sdk_path);
    println!();
    println!("3. Set environment variable:");

    #[cfg(target_os = "windows")]
    {
        println!("   setx CEPTON_SDK_PATH \"{}\"", sdk_path);
        println!("   setx PATH \"%PATH%;{}\\bin\"", sdk_path);
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("   export CEPTON_SDK_PATH={}", sdk_path);
        println!("   export PATH=$PATH:$CEPTON_SDK_PATH/bin");
        println!();
        println!("   Add these to your ~/.bashrc or ~/.zshrc");
    }

    println!();
    println!("4. Verify installation:");
    println!("   towa check");

    // 保存到配置文件
    crate::config::update_cepton_path(sdk_path);
}

fn run_command(program: &str, args: &[&str]) {
    println!("{} {} {}", "Running:".cyan(), program, args.join(" "));

    match Command::new(program).args(args).status() {
        Ok(status) => {
            if status.success() {
                println!("{} Command completed successfully", "✓".green());
            } else {
                println!("{} Command failed with status: {}", "✗".red(), status);
            }
        }
        Err(e) => {
            println!("{} Failed to execute command: {}", "✗".red(), e);
        }
    }
}
