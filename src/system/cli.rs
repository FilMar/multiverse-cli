use clap::Subcommand;

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Create a new system with flexible metadata
    Create {
        /// System name (unique identifier)
        name: String,
        /// System display name (human-readable name)
        #[arg(long)]
        display_name: String,
        /// System type (magic, technology, cosmology, etc.)
        #[arg(long, short = 't')]
        system_type: String,
        /// Set metadata field (can be used multiple times: --set complexity=high --set origin=ancient)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List systems in current world
    List,
    
    /// Show system details
    Info {
        /// System name
        name: String,
    },
    
    /// Delete a system
    Delete {
        /// System name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing system
    Update {
        /// System name
        name: String,
        /// New system display name
        #[arg(long)]
        display_name: Option<String>,
        /// New system type
        #[arg(long, short = 't')]
        system_type: Option<String>,
        /// Set metadata field (can be used multiple times: --set complexity=high --set origin=ancient)
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