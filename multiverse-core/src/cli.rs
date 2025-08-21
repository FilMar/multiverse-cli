use clap::{Parser, Subcommand};
use crate::config::ConfigCommands;
use crate::world::WorldCommands;

#[derive(Parser)]
#[command(name = "multiverse")]
#[command(about = "Professional tooling for complex narrative universes")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    
    /// Manage worlds
    World {
        #[command(subcommand)]
        command: WorldCommands,
    },
    
    /// Manage diary series
    Diary {
        #[command(subcommand)]
        command: DiaryCommands,
    },
    
    /// Show workspace information
    Info,
}


#[derive(Subcommand)]
pub enum DiaryCommands {
    /// Create a new diary series
    Create {
        /// World name
        #[arg(long)]
        world: String,
        /// Diary series name
        #[arg(long)]
        name: String,
        /// Narrator name
        #[arg(long)]
        narrator: String,
    },
    
    /// List diary series in a world
    List {
        /// World name
        #[arg(long)]
        world: String,
    },
}