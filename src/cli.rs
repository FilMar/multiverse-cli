use clap::{Parser, Subcommand};
use crate::{
    character::CharacterCommands,
    world::WorldCommands, 
    timeline::TimelineCommands,
    story::StoryCommands,
    episode::EpisodeCommands,
    location::LocationCommands,
    event::EventCommands,
    faction::FactionCommands,
    system::SystemCommands,
    race::RaceCommands
    // TODO: Re-enable as we implement them:
};

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
    /// Manage characters (requires being in a multiverse project)
    Character {
        #[command(subcommand)]
        command: CharacterCommands,
    },
    
    // TODO: Re-enable when entities are re-implemented:
   // /// Manage stories (requires being in a multiverse project)
   Story {
        #[command(subcommand)]
        command: StoryCommands,
    },
    Episode {
        #[command(subcommand)]
        command: EpisodeCommands,
    },
   
   /// Manage locations (requires being in a multiverse project)
   Location {
       #[command(subcommand)]
       command: LocationCommands,
   },
   
   /// Manage events (requires being in a multiverse project)
   Event {
       #[command(subcommand)]
       command: EventCommands,
   },
   
   /// Manage factions (requires being in a multiverse project)
   Faction {
       #[command(subcommand)]
       command: FactionCommands,
   },
   
   /// Manage systems (requires being in a multiverse project)
   System {
       #[command(subcommand)]
       command: SystemCommands,
   },
   // 
   // /// Manage factions (requires being in a multiverse project)
   // Faction {
   //     #[command(subcommand)]
   //     command: FactionCommands,
   // },
   // 
   // /// Manage events (requires being in a multiverse project)
   // Event {
   //     #[command(subcommand)]
   //     command: EventCommands,
   // },

   /// Manage races (requires being in a multiverse project)
   Race {
       #[command(subcommand)]
       command: RaceCommands,
   },

    /// Manage timeline configuration and dates (requires being in a multiverse project)
    Timeline {
        #[command(subcommand)]
        command: TimelineCommands,
    },

    
    /// Execute SQL SELECT queries on the database
    Query {
        /// The SQL SELECT query to execute
        sql: String,
    },

    /// Show project information
    Info,
}


