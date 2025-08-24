use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TimelineCommands {
    /// Show timeline configuration
    Info,
    /// Parse and validate a date
    Parse {
        /// Date string to parse
        date: String,
    },
}