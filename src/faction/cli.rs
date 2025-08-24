use clap::Subcommand;

#[derive(Subcommand)]
pub enum FactionCommands {
    /// Create a new faction with flexible metadata
    Create {
        /// Faction name (unique identifier)
        name: String,
        /// Faction display name (human-readable name)
        #[arg(long)]
        display_name: String,
        /// Faction type (government, guild, religion, military, etc.)
        #[arg(long, short = 't')]
        faction_type: String,
        /// Set metadata field (can be used multiple times: --set size=large --set alignment=neutral)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List factions in current world
    List,
    
    /// Show faction details
    Info {
        /// Faction name
        name: String,
    },
    
    /// Delete a faction
    Delete {
        /// Faction name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing faction
    Update {
        /// Faction name
        name: String,
        /// New faction display name
        #[arg(long)]
        display_name: Option<String>,
        /// New faction type
        #[arg(long, short = 't')]
        faction_type: Option<String>,
        /// Set metadata field (can be used multiple times: --set size=large --set alignment=neutral)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
}

/// Parse a single key-value pair for --set flag
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}