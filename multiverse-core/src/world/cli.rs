use clap::Subcommand;

#[derive(Subcommand)]
pub enum WorldCommands {
    /// Create a new world (local or from Git)
    Create {
        /// World name
        name: String,
        /// Optional description
        #[arg(long)]
        description: Option<String>,
        /// Visual aesthetic (fantasy, modern, historical, etc.)
        #[arg(long)]
        aesthetic: Option<String>,
        /// Clone from Git repository instead of creating local
        #[arg(long)]
        from_git: Option<String>,
    },
    
    /// List all worlds in workspace
    List,
    
    /// Show world details
    Info {
        /// World name
        name: String,
    },
    
    /// Pull updates from Git (single world or all worlds)
    Pull {
        /// World name (if not specified, pulls all worlds)
        name: Option<String>,
    },
    
    /// Push changes to Git (single world only)
    Push {
        /// World name (required for safety)
        name: String,
    },
    
    /// Show Git status (single world or all worlds)
    Status {
        /// World name (if not specified, shows all worlds)
        name: Option<String>,
    },
    
    /// Delete a world (with confirmation)
    Delete {
        /// World name
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}