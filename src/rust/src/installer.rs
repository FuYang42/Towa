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

pub fn install_ufb_dependencies() {
    println!("{}", "Installing Cepton Unified Firmware (UFB) Dependencies...".bold());
    println!();

    println!("{}", "This will install:".cyan());
    println!("  - CMake and Ninja (build systems)");
    println!("  - Python 3 (required for UFB builder)");
    println!("  - Rust toolchain (for ufb_builder)");
    println!("  - Additional tools (libpcap, etc.)");
    println!();

    #[cfg(target_os = "windows")]
    {
        println!("{}", "Windows Installation:".yellow());
        println!();

        // CMake and Ninja
        if Command::new("cmake").arg("--version").output().is_err() {
            println!("Installing CMake...");
            run_command("winget", &["install", "-e", "--id", "Kitware.CMake"]);
        }

        if Command::new("ninja").arg("--version").output().is_err() {
            println!("Installing Ninja...");
            run_command("winget", &["install", "-e", "--id", "Ninja-build.Ninja"]);
        }

        // Python 3
        if Command::new("python3").arg("--version").output().is_err() {
            println!("Installing Python 3...");
            run_command("winget", &["install", "-e", "--id", "Python.Python.3.10"]);
            println!();
            println!("{}", "Note: After installing Python, you may need to create a symlink:".yellow());
            println!("  mklink \"C:\\Program Files\\Python310\\python3.exe\" \"C:\\Program Files\\Python310\\python.exe\"");
        }

        // Rust
        if Command::new("rustc").arg("--version").output().is_err() {
            println!("Installing Rust...");
            println!("  Please visit: https://rustup.rs/");
            println!("  Or run: winget install -e --id Rustlang.Rustup");
        }

        println!();
        println!("{}", "Important Git Configuration:".yellow());
        println!("  1. Enable Developer Mode in Windows Settings");
        println!("  2. Run: git config --global core.symlinks true");
        println!("  3. Run: git config --global core.autocrlf false");
    }

    #[cfg(target_os = "linux")]
    {
        println!("{}", "Linux Installation:".yellow());
        println!();

        if Command::new("apt").arg("--version").output().is_ok() {
            println!("Ubuntu/Debian detected. Installing dependencies...");
            println!("Running: sudo apt update && sudo apt install -y cmake ninja-build libpcap-dev python3 python3-pip");
            println!();
            println!("Please run the following command:");
            println!("  sudo apt update && sudo apt install -y cmake ninja-build libpcap-dev python3 python3-pip");
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            println!("Fedora/RHEL detected. Installing dependencies...");
            println!("  sudo dnf install -y cmake ninja-build libpcap-devel python3 python3-pip");
        } else if Command::new("pacman").arg("--version").output().is_ok() {
            println!("Arch Linux detected. Installing dependencies...");
            println!("  sudo pacman -S --noconfirm cmake ninja libpcap python python-pip");
        }

        // Rust
        if Command::new("rustc").arg("--version").output().is_err() {
            println!();
            println!("Installing Rust...");
            println!("  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("{}", "macOS Installation:".yellow());
        println!();

        if Command::new("brew").arg("--version").output().is_ok() {
            println!("Homebrew found. Installing dependencies...");
            run_command("brew", &["install", "cmake", "ninja", "python3"]);

            if Command::new("rustc").arg("--version").output().is_err() {
                println!();
                println!("Installing Rust...");
                run_command("sh", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"]);
            }
        } else {
            println!("Homebrew not found. Please install it first:");
            println!("  /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
        }
    }

    println!();
    println!("{}", "Additional Steps for UFB Development:".cyan().bold());
    println!();
    println!("1. Install ARM LLVM Toolchain:");
    println!("   - Download from: https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm/releases");
    println!("   - Get version: ATfE-20.1.0 for your platform");
    println!("   - Extract and rename to: ATfE-20.1.0 (remove platform suffix)");
    #[cfg(target_os = "windows")]
    println!("   - Move to: C:\\opt\\arm\\ATfE-20.1.0");
    #[cfg(not(target_os = "windows"))]
    println!("   - Move to: /opt/arm/ATfE-20.1.0");
    println!();

    println!("2. Install Pre-commit Tools (optional but recommended):");
    println!("   pip3 install pre-commit pylint");
    println!("   cargo install typos-cli");
    println!();

    println!("3. Configure Git:");
    println!("   git config --global pull.rebase true");
    println!("   git config --global core.symlinks true");
    println!("   git config --global core.autocrlf false");
    println!();

    println!("4. For detailed setup instructions, see:");
    println!("   docs/CEPTON_UFB_SETUP.md");
    println!();

    println!("{} UFB dependencies installation guide complete!", "✓".green());
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
