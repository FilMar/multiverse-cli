use clap::Subcommand;

#[derive(Subcommand)]
pub enum RaceCommands {
    /// Create a new race with unified --set for all fields
    Create {
        /// Race name (unique identifier)
        name: String,
        /// Set any field (--set display_name="Name" --set description="Description" --set status="Active" --set lifespan=1000)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
    
    /// List all races in the current world
    List,
    
    /// Show details for a specific race
    Info {
        /// The unique name of the race
        name: String,
    },
    
    /// Delete a race
    Delete {
        /// The unique name of the race to delete
        name: String,
        /// Skip the confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update an existing race
    Update {
        /// Race name
        name: String,
        /// Set any field (--set display_name="Name" --set description="Description" --set status="Active" --set lifespan=1000)
        #[arg(long, value_parser = parse_key_val)]
        set: Vec<(String, String)>,
    },
}

/// Parse a single key-value pair for the --set flag
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
