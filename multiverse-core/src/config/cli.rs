use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize workspace in current directory
    Init {
        /// Workspace path (defaults to current directory)
        #[arg(long)]
        workspace: Option<String>,
    },
    
    /// Show current configuration
    Show,
    
    /// Set configuration value
    Set {
        /// Configuration key (e.g., workspace.default_world)
        key: String,
        /// Configuration value
        value: String,
    },
}