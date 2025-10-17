use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;

mod config;
mod installer;

#[derive(Parser)]
#[command(name = "towa")]
#[command(about = "Towa - Development Environment Setup Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check current environment status
    Check,

    /// Install development tools
    Install {
        /// Specific tool to install (all, c, rust, cepton)
        #[arg(default_value = "all")]
        tool: String,
    },

    /// Configure Cepton SDK
    Cepton {
        /// SDK installation path
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Initialize configuration file
    Init,

    /// Show version information
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check => check_environment(),
        Commands::Install { tool } => install_tools(&tool),
        Commands::Cepton { path } => configure_cepton(path),
        Commands::Init => init_config(),
        Commands::Version => show_version(),
    }
}

fn check_environment() {
    println!("{}", "=== Checking Environment ===".bold().cyan());

    // 调用 C 编写的环境检测工具
    #[cfg(target_os = "windows")]
    let checker = "..\\..\\build\\bin\\env_checker.exe";

    #[cfg(not(target_os = "windows"))]
    let checker = "../../build/bin/env_checker";

    match Command::new(checker).output() {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("{} Running C env_checker...", "Info:".yellow());
                println!("Please build the C tools first:");
                println!("  cd src/c && mkdir build && cd build && cmake .. && make");
            }
        }
        Err(_) => {
            println!("{} C env_checker not found, running basic checks...", "Warning:".yellow());
            basic_check();
        }
    }
}

fn basic_check() {
    let tools = vec![
        ("gcc", "GCC"),
        ("clang", "Clang"),
        ("rustc", "Rust"),
        ("cargo", "Cargo"),
        ("cmake", "CMake"),
        ("git", "Git"),
    ];

    for (cmd, name) in tools {
        match Command::new(cmd).arg("--version").output() {
            Ok(output) if output.status.success() => {
                println!("{} {} is installed", "✓".green(), name);
            }
            _ => {
                println!("{} {} not found", "✗".red(), name);
            }
        }
    }
}

fn install_tools(tool: &str) {
    println!("{} Installing: {}", "=>".cyan().bold(), tool);

    match tool {
        "all" => {
            installer::install_c_toolchain();
            installer::install_rust_toolchain();
            installer::install_build_tools();
        }
        "c" => installer::install_c_toolchain(),
        "rust" => installer::install_rust_toolchain(),
        "cepton" => installer::install_cepton_sdk(None),
        _ => {
            println!("{} Unknown tool: {}", "Error:".red(), tool);
            println!("Available: all, c, rust, cepton");
        }
    }
}

fn configure_cepton(path: Option<String>) {
    println!("{}", "=== Configuring Cepton SDK ===".bold().cyan());
    installer::install_cepton_sdk(path);
}

fn init_config() {
    println!("{}", "=== Initializing Towa Configuration ===".bold().cyan());
    config::create_default_config();
}

fn show_version() {
    println!("Towa CLI v{}", env!("CARGO_PKG_VERSION"));
    println!("Development Environment Setup Tool");
}
