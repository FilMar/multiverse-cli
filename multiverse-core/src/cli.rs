use clap::{Parser, Subcommand};
use crate::{story::StoryCommands, world::WorldCommands};

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
    
    /// Manage stories (requires being in a multiverse project)
    Story {
        #[command(subcommand)]
        command: StoryCommands,
    },
    
    /// Show project information
    Info,
}


