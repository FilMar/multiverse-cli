use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum RelationCommands {
    /// Manage character relations
    Character {
        #[command(subcommand)]
        action: CharacterRelationAction,
    },
    /// Manage location relations (future)
    Location {
        #[command(subcommand)]
        action: LocationRelationAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum CharacterRelationAction {
    /// Manage character-episode relations
    Episode {
        #[command(subcommand)]
        action: CharacterEpisodeAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum CharacterEpisodeAction {
    /// Add character to episode: character episode-id --story="story-name" --role="role" --importance="Main"
    Add {
        character_name: String,
        episode_id: i32,
        #[arg(long)]
        story: String,
        #[arg(long)]
        role: String,
        #[arg(long, default_value = "Supporting")]
        importance: String,
    },
    /// Remove character from episode: character episode-id --story="story-name"
    Remove {
        character_name: String,
        episode_id: i32,
        #[arg(long)]
        story: String,
    },
    /// List episodes for character or characters for episode: character --story="story" OR --episode episode-id --story="story"
    List {
        /// Character name to list episodes for
        character_name: Option<String>,
        /// Episode ID to list characters for
        #[arg(long)]
        episode: Option<i32>,
        #[arg(long)]
        story: String,
    },
    /// Update character role in episode: character episode-id --story="story" --role="new-role" --importance="Main"
    Update {
        character_name: String,
        episode_id: i32,
        #[arg(long)]
        story: String,
        #[arg(long)]
        role: Option<String>,
        #[arg(long)]
        importance: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum LocationRelationAction {
    /// Manage location-character relations
    Character {
        #[command(subcommand)]
        action: LocationCharacterAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum LocationCharacterAction {
    /// Placeholder for future location-character relations
    Add {
        character_name: String,
        location_name: String,
        #[arg(long)]
        relation_type: String,
    },
}