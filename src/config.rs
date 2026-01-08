use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let proj_dirs =
        ProjectDirs::from("", "", "gity").ok_or("Could not determine configuration directory")?;
    Ok(proj_dirs.config_dir().join("config.json"))
}

pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    // 1. Try environment variables
    if let Ok(key) = env::var("GITY_GEMINI_API_KEY").or_else(|_| env::var("GEMINI_API_KEY")) {
        return Ok(key);
    }

    // 2. Try config file
    let config = Config::load()?;
    if let Some(key) = config.api_key {
        return Ok(key);
    }

    Err("API Key not found. Please set GITY_GEMINI_API_KEY environment variable or run 'gity config --api-key <KEY>'".into())
}
