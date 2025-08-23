mod cli;
mod database;
mod world;
mod story;
mod character;
mod location;
mod templates;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands};
use world::{handle_world_command, WorldConfig};
use story::{handle_story_command, handle_episode_command};
use character::handle_character_command;
use location::handle_location_command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::World { command } => handle_world_command(command),
        Commands::Story { command } => handle_story_command(command),
        Commands::Episode { command } => handle_episode_command(command),
        Commands::Character { command } => handle_character_command(command),
        Commands::Location { command } => handle_location_command(command),
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
                println!("   Description: {desc}");
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

