use clap::Subcommand;

#[derive(Subcommand)]
pub enum CharacterCommands {
    /// Create a new character with unified --set for all fields
    Create {
        /// Character name (unique identifier)
        name: String,
        /// Set any field (--set display_name="Name" --set status="Active" --set age=25 --set faction=rebels)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List characters in current world
    List,
    
    /// Show character details
    Info {
        /// Character name
        name: String,
    },
    
    /// Delete a character
    Delete {
        /// Character name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing character
    Update {
        /// Character name
        name: String,
        /// Set any field (--set display_name="Name" --set status="Active" --set age=25 --set description="...")
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
