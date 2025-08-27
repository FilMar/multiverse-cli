use clap::Subcommand;

#[derive(Subcommand)]
pub enum StoryCommands {
    /// Create a new story with flexible metadata
    Create {
        /// Story name (used for directory naming)
        name: String,
        /// Set metadata field (can be used multiple times: --set title="Story Title" --set type=fantasy --set author=John)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List available story types with their required fields
    Types,
    
    /// List stories in current world
    List,
    
    /// Show story details
    Info {
        /// Story name
        name: String,
    },
    
    /// Delete a story
    Delete {
        /// Story name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing story
    Update {
        /// Story name
        name: String,
        /// Set metadata field (can be used multiple times: --set title="New Title" --set type=fantasy --set author=John)
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
