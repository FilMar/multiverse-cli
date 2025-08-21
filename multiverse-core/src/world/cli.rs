use clap::Subcommand;

#[derive(Subcommand)]
pub enum WorldCommands {
    /// Initialize a new multiverse project in current directory
    Init {
        /// Project name
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
    
    /// Show project details
    Info,
    
    /// Pull updates from Git
    Pull,
    
    /// Push changes to Git
    Push,
    
    /// Show Git status
    Status,
    
    /// Configure world settings
    Config {
        /// Configuration key to set (e.g., "world.description")
        #[arg(long)]
        set: Option<String>,
        /// Configuration value
        #[arg(long)]
        value: Option<String>,
    },
}