pub mod database;
pub mod world;
pub mod story;
pub mod character;
pub mod location;
pub mod system;
pub mod faction;
pub mod event;
pub mod timeline;
pub mod race;
pub mod templates;

// Re-export main types for external use
pub use world::{WorldConfig, handle_world_command, WorldCommands};
pub use story::{handle_story_command, handle_episode_command, StoryCommands, EpisodeCommands, Story, Episode};
pub use character::{handle_character_command, CharacterCommands, Character};
pub use location::{handle_location_command, LocationCommands, Location};
pub use system::{handle_system_command, SystemCommands, System};
pub use faction::{handle_faction_command, FactionCommands, Faction};
pub use event::{handle_event_command, EventCommands, Event};
pub use race::{handle_race_command, RaceCommands, Race};
pub use timeline::{TimelineDate, TimelineConfig, load_timeline_config, timeline_config_exists};
pub use database::{get_connection, init_database};

// Result type for the crate
pub type Result<T> = anyhow::Result<T>;
