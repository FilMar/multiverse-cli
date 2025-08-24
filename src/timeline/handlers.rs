use anyhow::Result;
use crate::timeline::{TimelineCommands, TimelineDate, load_timeline_config, timeline_config_exists};

pub fn handle_timeline_command(command: TimelineCommands) -> Result<()> {
    match command {
        TimelineCommands::Info => {
            if timeline_config_exists() {
                let config = load_timeline_config()?;
                println!("📅 Timeline System: {}", config.config.calendar.name);
                println!("📊 Blocks per day: {}", config.config.day_structure.blocks_per_day);
                println!("📆 Months per year: {}", config.config.calendar.months_per_year);
                println!("🗓️  Days per month: {}", config.config.calendar.days_per_month);
                println!("⭐ Era events: {}", config.config.era_events.len());
            } else {
                println!("⚠️  No timeline configuration found (.multiverse/timeline.toml)");
                println!("Using real dates as fallback");
            }
        },
        TimelineCommands::Parse { date } => {
            match TimelineDate::parse(&date) {
                Ok(timeline_date) => {
                    println!("✅ Parsed: {}", timeline_date);
                    println!("🔢 Sort key: {}", timeline_date.sort_key());
                },
                Err(e) => {
                    println!("❌ Parse error: {}", e);
                }
            }
        }
    }
    Ok(())
}
