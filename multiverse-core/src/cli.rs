use clap::{Parser, Subcommand};
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
    /// Initialize a new multiverse project or manage world settings
    World {
        #[command(subcommand)]
        command: WorldCommands,
    },
    
    /// Manage diary series (requires being in a multiverse project)
    Diary {
        #[command(subcommand)]
        command: DiaryCommands,
    },
    
    /// Show project information
    Info,
}


#[derive(Subcommand)]
pub enum DiaryCommands {
    /// Create a new diary series
    Create {
        /// Diary series name
        name: String,
        /// Narrator name
        #[arg(long)]
        narrator: String,
    },
    
    /// List diary series in current world
    List,
}
