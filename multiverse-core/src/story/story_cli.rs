use clap::Subcommand;

#[derive(Subcommand)]
pub enum StoryCommands {
    /// Create a new story
    Create {
        /// Story name
        name: String,
        /// Narrator name
        #[arg(long)]
        narrator: String,
        /// Story type (diary, extra, etc.)
        #[arg(long)]
        story_type: Option<String>,
    },
    
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
}
