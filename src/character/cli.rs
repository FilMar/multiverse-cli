use clap::Subcommand;

#[derive(Subcommand)]
pub enum CharacterCommands {
    /// Create a new character with flexible metadata
    Create {
        /// Character name (unique identifier)
        name: String,
        /// Character display name (human-readable name)
        #[arg(long)]
        display_name: String,
        /// Set metadata field (can be used multiple times: --set age=25 --set faction=rebels)
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
}

/// Parse a single key-value pair for --set flag
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}