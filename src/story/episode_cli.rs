use clap::Subcommand;

#[derive(Subcommand)]
pub enum EpisodeCommands {
    /// Create a new episode in a story
    Create {
        /// Story name
        #[arg(short, long)]
        story: String,
        /// Episode title (optional)
        #[arg(short, long)]
        title: Option<String>,
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
        /// New episode title
        #[arg(short, long)]
        title: Option<String>,
        /// New episode status
        #[arg(long)]
        status: Option<String>,
        /// New word count
        #[arg(long)]
        word_count: Option<i32>,
    },
}
