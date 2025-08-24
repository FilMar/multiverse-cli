use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::{Result, Context};
use crate::timeline::{DayBlocks, Months, Era};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarConfig {
    pub name: String,
    pub year_name: String,
    pub year_days: u32,
    pub months_per_year: u8,
    pub days_per_month: u8,
    pub weeks_per_month: u8,
    pub week_name: String,
    pub days_per_week: u8,
    pub day_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStructureConfig {
    pub blocks_per_day: u8,
    pub candles_per_block: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFormatsConfig {
    pub full: String,
    pub abbreviated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineConfigFile {
    pub creation_year: u32,
    pub calendar: CalendarConfig,
    pub day_structure: DayStructureConfig,
    pub day_blocks: DayBlocks,
    pub months: Months,
    pub era_events: HashMap<String, Era>,
    pub date_formats: DateFormatsConfig,
}

#[derive(Debug, Clone)]
pub struct TimelineConfig {
    pub config: TimelineConfigFile,
    pub era_events_by_abbrev: HashMap<String, Era>,
}

impl TimelineConfig {
    pub fn validate(&self) -> Result<()> {
        // Validate day blocks
        if self.config.day_blocks.names.len() != self.config.day_structure.blocks_per_day as usize {
            anyhow::bail!("Day blocks count doesn't match blocks_per_day");
        }
        
        // Validate months
        if self.config.months.names.len() != self.config.calendar.months_per_year as usize {
            anyhow::bail!("Months count doesn't match months_per_year");
        }
        
        Ok(())
    }
}

pub fn load_timeline_config() -> Result<TimelineConfig> {
    let config_path = Path::new(".multiverse/timeline.toml");
    
    if !config_path.exists() {
        anyhow::bail!("Timeline config not found at .multiverse/timeline.toml");
    }
    
    let content = std::fs::read_to_string(config_path)
        .context("Failed to read timeline config")?;
    
    let config: TimelineConfigFile = toml::from_str(&content)
        .context("Failed to parse timeline config")?;
    
    // Build era events lookup by abbreviation
    let era_events_by_abbrev = config.era_events.values()
        .map(|era| (era.abbrev.clone(), era.clone()))
        .collect();
    
    let timeline_config = TimelineConfig {
        config,
        era_events_by_abbrev,
    };
    
    timeline_config.validate()?;
    Ok(timeline_config)
}

pub fn timeline_config_exists() -> bool {
    Path::new(".multiverse/timeline.toml").exists()
}
