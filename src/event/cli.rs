use clap::Subcommand;

#[derive(Subcommand)]
pub enum EventCommands {
    /// Create a new event with flexible metadata
    Create {
        /// Event name (unique identifier)
        name: String,
        /// Event display name (human-readable name)
        #[arg(long)]
        display_name: String,
        /// Event type (battle, political, natural, cultural, etc.)
        #[arg(long, short = 't')]
        event_type: String,
        /// Event date (timeline format like "3A/2 Lum 124 DF" or "2024-03-15", defaults to "now")
        #[arg(long, short = 'd')]
        date: Option<String>,
        /// Set metadata field (can be used multiple times: --set year=1453 --set importance=high)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List events in current world
    List,
    
    /// List events in chronological order (using timeline dates)
    Timeline,
    
    /// Show event details
    Info {
        /// Event name
        name: String,
    },
    
    /// Delete an event
    Delete {
        /// Event name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing event
    Update {
        /// Event name
        name: String,
        /// New event display name
        #[arg(long)]
        display_name: Option<String>,
        /// New event type
        #[arg(long, short = 't')]
        event_type: Option<String>,
        /// New event date
        #[arg(long, short = 'd')]
        date: Option<String>,
        /// Set metadata field (can be used multiple times: --set importance=high)
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