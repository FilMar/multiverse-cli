mod cli;
mod database;
mod world;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands, DiaryCommands};
use world::{handle_world_command, WorldConfig};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::World { command } => handle_world_command(command),
        Commands::Diary { command } => todo!(),
        Commands::Info => handle_info(),
    }
}

fn handle_info() -> Result<()> {
    println!("🌌 Multiverse CLI Info");
    println!("   Version: {}", env!("CARGO_PKG_VERSION"));
    
    match WorldConfig::load() {
        Ok(config) => {
            println!("   World: {}", config.world.name);
            if let Some(desc) = &config.world.description {
                println!("   Description: {}", desc);
            }
            println!("   Config: .multiverse/config.toml");
        }
        Err(_) => {
            println!("   Status: Not in a multiverse project directory");
            println!("   Run 'multiverse init' to create a new project");
        }
    }
    
    Ok(())
}
