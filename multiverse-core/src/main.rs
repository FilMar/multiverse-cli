mod cli;
mod config;
mod database;
mod world;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands, DiaryCommands};
use config::{Config, handle_config_command};
use world::handle_world_command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Config { command } => handle_config_command(command),
        Commands::World { command } => handle_world_command(command),
        Commands::Diary { command } => handle_diary_command(command),
        Commands::Info => handle_info(),
    }
}



fn handle_diary_command(command: DiaryCommands) -> Result<()> {
    match command {
        DiaryCommands::Create { world, name, narrator } => {
            println!("📚 Creating diary '{}' in world '{}'...", name, world);
            println!("   Narrator: {}", narrator);
            println!("✅ Diary '{}' created!", name);
        }
        
        DiaryCommands::List { world } => {
            println!("📚 Diary series in world '{}':", world);
            println!("   (no diary series found)");
        }
    }
    Ok(())
}

fn handle_info() -> Result<()> {
    println!("🌌 Multiverse CLI Info");
    println!("   Version: {}", env!("CARGO_PKG_VERSION"));
    
    let (config, config_path) = Config::load_or_default();
    
    match config_path {
        Some(path) => {
            println!("   Config: {}", path.display());
            println!("   Workspace: {}", config.workspace.path.display());
        }
        None => {
            println!("   Config: (none - using defaults)");
            println!("   Workspace: {} (current directory)", config.workspace.path.display());
        }
    }
    
    Ok(())
}
