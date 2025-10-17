use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct TowaConfig {
    pub version: String,
    pub tools: ToolsConfig,
    pub cepton: CeptonConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolsConfig {
    pub c_compiler: String,
    pub rust_toolchain: String,
    pub cmake_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CeptonConfig {
    pub sdk_path: Option<String>,
    pub sdk_version: String,
    pub auto_update: bool,
}

impl Default for TowaConfig {
    fn default() -> Self {
        TowaConfig {
            version: "1.0".to_string(),
            tools: ToolsConfig {
                c_compiler: "gcc".to_string(),
                rust_toolchain: "stable".to_string(),
                cmake_version: "latest".to_string(),
            },
            cepton: CeptonConfig {
                sdk_path: None,
                sdk_version: "latest".to_string(),
                auto_update: false,
            },
        }
    }
}

pub fn create_default_config() {
    let config = TowaConfig::default();
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create config directory");
        }
    }

    let toml_string = toml::to_string_pretty(&config).expect("Failed to serialize config");

    match fs::write(&config_path, toml_string) {
        Ok(_) => {
            println!("{} Configuration file created at:", "✓".green());
            println!("  {}", config_path.display());
        }
        Err(e) => {
            println!("{} Failed to create config: {}", "✗".red(), e);
        }
    }
}

pub fn load_config() -> Option<TowaConfig> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return None;
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                println!("{} Failed to parse config: {}", "Warning:".yellow(), e);
                None
            }
        },
        Err(e) => {
            println!("{} Failed to read config: {}", "Warning:".yellow(), e);
            None
        }
    }
}

pub fn save_config(config: &TowaConfig) {
    let config_path = get_config_path();
    let toml_string = toml::to_string_pretty(config).expect("Failed to serialize config");

    match fs::write(&config_path, toml_string) {
        Ok(_) => {
            println!("{} Configuration saved", "✓".green());
        }
        Err(e) => {
            println!("{} Failed to save config: {}", "✗".red(), e);
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push(".towa");
    path.push("config.toml");
    path
}

pub fn update_cepton_path(sdk_path: String) {
    let mut config = load_config().unwrap_or_default();
    config.cepton.sdk_path = Some(sdk_path);
    save_config(&config);
}
