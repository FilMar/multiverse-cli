use clap::{Parser, Subcommand};
use crate::{story::{StoryCommands, EpisodeCommands}, character::CharacterCommands, location::LocationCommands, system::SystemCommands, faction::FactionCommands, event::EventCommands, world::WorldCommands, race::RaceCommands, timeline::TimelineCommands, relations::RelationCommands};

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
    
    /// Manage episodes (requires being in a multiverse project)
    Episode {
        #[command(subcommand)]
        command: EpisodeCommands,
    },
    
    /// Manage characters (requires being in a multiverse project)
    Character {
        #[command(subcommand)]
        command: CharacterCommands,
    },
    
    /// Manage locations (requires being in a multiverse project)
    Location {
        #[command(subcommand)]
        command: LocationCommands,
    },
    
    /// Manage systems (requires being in a multiverse project)
    System {
        #[command(subcommand)]
        command: SystemCommands,
    },
    
    /// Manage factions (requires being in a multiverse project)
    Faction {
        #[command(subcommand)]
        command: FactionCommands,
    },
    
    /// Manage events (requires being in a multiverse project)
    Event {
        #[command(subcommand)]
        command: EventCommands,
    },

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

    /// Manage relations between entities (requires being in a multiverse project)
    Relation {
        #[command(subcommand)]
        command: RelationCommands,
    },
    
    /// Show project information
    Info,
}


