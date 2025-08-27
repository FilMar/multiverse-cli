use clap::Subcommand;

#[derive(Subcommand)]
pub enum EventCommands {
    /// Create a new event with flexible metadata
    Create {
        /// Event name (unique identifier)
        name: String,
        /// Set metadata field (can be used multiple times: --set title="Event Name" --set date="3A/2 Lum 124 DF")
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
        /// Set metadata field (can be used multiple times: --set title="New Name" --set date="new date")
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