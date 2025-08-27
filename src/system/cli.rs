use clap::Subcommand;

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Create a new system with unified --set for all fields
    Create {
        /// System name (unique identifier)
        name: String,
        /// Set any field (--set display_name="Name" --set system_type="magic" --set status="Active" --set complexity=high)
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
        /// Set any field (--set display_name="Name" --set system_type="magic" --set status="Active" --set complexity=high)
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