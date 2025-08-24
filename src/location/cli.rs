use clap::Subcommand;

#[derive(Subcommand)]
pub enum LocationCommands {
    /// Create a new location with flexible metadata
    Create {
        /// Location name (unique identifier)
        name: String,
        /// Location display name (human-readable name)
        #[arg(long)]
        display_name: String,
        /// Set metadata field (can be used multiple times: --set type=city --set population=1000 --set description="...")
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List locations in current world
    List,
    
    /// Show location details
    Info {
        /// Location name
        name: String,
    },
    
    /// Delete a location
    Delete {
        /// Location name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing location
    Update {
        /// Location name
        name: String,
        /// New location display name
        #[arg(long)]
        display_name: Option<String>,
        /// Set metadata field (can be used multiple times: --set type=city --set population=1000 --set description="...")
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