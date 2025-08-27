use clap::Subcommand;

#[derive(Subcommand)]
pub enum EpisodeCommands {
    /// Create a new episode in a story
    Create {
        /// Story name
        #[arg(short, long)]
        story: String,
        /// Set any field (--set display_name="Name" --set status="Active" --set age=25 --set faction=rebels)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List episodes in a story
    List {
        /// Story name
        #[arg(short, long)]
        story: String,
    },
    
    /// Show episode details
    Info {
        /// Story name
        #[arg(short, long)]
        story: String,
        /// Episode number
        #[arg(short, long)]
        number: i32,
    },
    
    /// Delete an episode
    Delete {
        /// Story name
        #[arg(short, long)]
        story: String,
        /// Episode number
        #[arg(short, long)]
        number: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing episode
    Update {
        /// Story name
        #[arg(short, long)]
        story: String,
        /// Episode number
        #[arg(short, long)]
        number: i32,
        /// Set any field (--set display_name="Name" --set status="Active" --set age=25 --set faction=rebels)
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
