mod cli;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands};
use multiverse::*;
use multiverse::timeline::handle_timeline_command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::World { command } => handle_world_command(command),
        Commands::Character { command } => handle_character_command(command),
        Commands::Timeline { command } => handle_timeline_command(command),
        Commands::Info => handle_info(),
        Commands::Story { command } => handle_story_command(command),
        Commands::Episode { command } => handle_episode_command(command),
        Commands::Location { command } => handle_location_command(command),
        Commands::Event { command } => handle_event_command(command),
        Commands::Faction { command } => handle_faction_command(command),
        Commands::System { command } => handle_system_command(command),
        Commands::Race { command } => handle_race_command(command),
        Commands::Query { sql } => handle_query_command(sql),
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

fn handle_query_command(sql: String) -> Result<()> {
    println!("🔍 Executing query...");
    database::execute_query(&sql)
}

