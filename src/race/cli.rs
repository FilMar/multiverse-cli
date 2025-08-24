use clap::Subcommand;

#[derive(Subcommand)]
pub enum RaceCommands {
    /// Create a new race with flexible metadata
    Create {
        /// Race name (unique identifier, e.g., 'elfo_silvano')
        name: String,
        /// Race display name (human-readable, e.g., 'Elfo Silvano')
        #[arg(long)]
        display_name: String,
        /// A brief description of the race
        #[arg(long)]
        description: Option<String>,
        /// Set metadata field (e.g., --set avg_lifespan=1000 --set abilities=["night_vision"])
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List all races in the current world
    List,
    
    /// Show details for a specific race
    Info {
        /// The unique name of the race
        name: String,
    },
    
    /// Delete a race
    Delete {
        /// The unique name of the race to delete
        name: String,
        /// Skip the confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing race
    Update {
        /// Race name
        name: String,
        /// New race display name
        #[arg(long)]
        display_name: Option<String>,
        /// New race description
        #[arg(long)]
        description: Option<String>,
        /// Set metadata field (e.g., --set avg_lifespan=1000 --set abilities=["night_vision"])
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
}

/// Parse a single key-value pair for the --set flag
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
