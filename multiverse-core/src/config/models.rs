use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workspace: WorkspaceConfig,
    pub git: Option<GitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub path: PathBuf,
    pub default_world: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub auto_commit: bool,
    pub auto_push: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workspace: WorkspaceConfig {
                path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                default_world: None,
            },
            git: Some(GitConfig {
                auto_commit: true,
                auto_push: false,
            }),
        }
    }
}

impl Config {
    /// Load config from .multiverse.toml file
    pub fn load(config_path: &PathBuf) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .context("Failed to read .multiverse.toml file")?;
        
        let config: Config = toml::from_str(&config_content)
            .context("Failed to parse .multiverse.toml file")?;
        
        Ok(config)
    }
    
    /// Save config to .multiverse.toml file
    pub fn save(&self, config_path: &PathBuf) -> Result<()> {
        let config_content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(config_path, config_content)
            .context("Failed to write .multiverse.toml file")?;
        
        Ok(())
    }
    
    /// Find config file by walking up directories
    pub fn find_config_file() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        
        loop {
            let config_path = current.join(".multiverse.toml");
            if config_path.exists() {
                return Some(config_path);
            }
            
            if !current.pop() {
                break;
            }
        }
        
        None
    }
    
    /// Load config with fallback strategy
    pub fn load_or_default() -> (Self, Option<PathBuf>) {
        // 1. Try to find .multiverse.toml in current dir or parents
        if let Some(config_path) = Self::find_config_file() {
            match Self::load(&config_path) {
                Ok(config) => return (config, Some(config_path)),
                Err(_) => {
                    // Config file exists but is invalid, continue to default
                }
            }
        }
        
        // 2. Return default config
        (Self::default(), None)
    }
}