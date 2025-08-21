use super::models::{Config, WorkspaceConfig, GitConfig};
use super::cli::ConfigCommands;
use anyhow::Result;
use std::path::PathBuf;

pub fn handle_config_command(command: ConfigCommands) -> Result<()> {
    match command {
        ConfigCommands::Init { workspace } => handle_init(workspace),
        ConfigCommands::Show => handle_show(),
        ConfigCommands::Set { key, value } => handle_set(key, value),
    }
}

fn handle_init(workspace: Option<String>) -> Result<()> {
    println!("🌌 Initializing Multiverse workspace...");
    
    let workspace_path = match workspace {
        Some(path) => {
            let p = PathBuf::from(path);
            println!("   Workspace: {}", p.display());
            p
        }
        None => {
            let current = std::env::current_dir()?;
            println!("   Workspace: {} (current directory)", current.display());
            current
        }
    };
    
    // Check if .multiverse.toml already exists
    let config_path = workspace_path.join(".multiverse.toml");
    if config_path.exists() {
        println!("⚠️  Workspace already initialized (.multiverse.toml exists)");
        return Ok(());
    }
    
    // Create config with workspace path
    let config = Config {
        workspace: WorkspaceConfig {
            path: workspace_path.clone(),
            default_world: None,
        },
        git: Some(GitConfig {
            auto_commit: true,
            auto_push: false,
        }),
    };
    
    // Save config file
    config.save(&config_path)?;
    
    println!("✅ Workspace initialized!");
    println!("   Created: {}", config_path.display());
    
    Ok(())
}

fn handle_show() -> Result<()> {
    println!("🌌 Multiverse Configuration");
    
    let (config, config_path) = Config::load_or_default();
    
    match config_path {
        Some(path) => {
            println!("   Config file: {}", path.display());
            println!("   Workspace: {}", config.workspace.path.display());
            
            if let Some(default_world) = &config.workspace.default_world {
                println!("   Default world: {}", default_world);
            } else {
                println!("   Default world: (none)");
            }
            
            if let Some(git_config) = &config.git {
                println!("   Git auto-commit: {}", git_config.auto_commit);
                println!("   Git auto-push: {}", git_config.auto_push);
            }
        }
        None => {
            println!("   Config file: (none - using defaults)");
            println!("   Workspace: {} (current directory)", config.workspace.path.display());
        }
    }
    
    Ok(())
}

fn handle_set(key: String, value: String) -> Result<()> {
    println!("🔧 Setting configuration: {} = {}", key, value);
    
    let (mut config, config_path) = Config::load_or_default();
    
    // Parse key and set value
    match key.as_str() {
        "workspace.default_world" => {
            config.workspace.default_world = Some(value);
        }
        "git.auto_commit" => {
            config.git.get_or_insert_with(GitConfig::default).auto_commit = value.parse()?;
        }
        "git.auto_push" => {
            config.git.get_or_insert_with(GitConfig::default).auto_push = value.parse()?;
        }
        _ => {
            println!("❌ Unknown configuration key: {}", key);
            return Ok(());
        }
    }
    
    // Save config
    let save_path = config_path.unwrap_or_else(|| PathBuf::from(".multiverse.toml"));
    config.save(&save_path)?;
    
    println!("✅ Configuration updated!");
    
    Ok(())
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            auto_commit: true,
            auto_push: false,
        }
    }
}