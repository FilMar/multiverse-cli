use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use regex::Regex;
use std::fmt;
use crate::timeline::{TimelineConfig, Year, load_timeline_config, timeline_config_exists};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimelineDate {
    Fantasy {
        candela: Option<u8>,
        alba: Option<u8>,  
        penta: Option<u8>,
        month: Option<String>,
        year: u32,
        era: String,
        absolute_year: u32,
        raw_input: String,
    },
    Real {
        datetime: DateTime<Utc>,
    },
}

impl TimelineDate {
    pub fn now() -> Self {
        if timeline_config_exists() {
            // Try to create a fantasy date for "now" - simplified
            match load_timeline_config() {
                Ok(_config) => {
                    // For now, just use real date even if config exists
                    // TODO: implement "current world date" mapping
                    TimelineDate::Real { 
                        datetime: Utc::now()
                    }
                },
                Err(_) => TimelineDate::Real { 
                    datetime: Utc::now()
                }
            }
        } else {
            TimelineDate::Real { 
                datetime: Utc::now()
            }
        }
    }
    
    pub fn parse(input: &str) -> Result<Self> {
        if !timeline_config_exists() {
            // Try to parse as ISO datetime, fallback to now
            match DateTime::parse_from_rfc3339(input) {
                Ok(dt) => Ok(TimelineDate::Real { 
                    datetime: dt.with_timezone(&Utc) 
                }),
                Err(_) => Ok(TimelineDate::Real { 
                    datetime: Utc::now() 
                })
            }
        } else {
            let config = load_timeline_config()
                .context("Failed to load timeline config")?;
            Self::parse_fantasy(input, &config)
        }
    }
    
    fn parse_fantasy(input: &str, config: &TimelineConfig) -> Result<Self> {
        // Parse abbreviated format: "4C 3A/2 Lum 124 DF"
        let abbrev_re = Regex::new(r"^(?:(\d+)C\s+)?(?:(\d+)A/(\d+)\s+)?(\w+)\s+(\d+)\s*(\w+)$")
            .context("Failed to compile regex")?;
            
        if let Some(captures) = abbrev_re.captures(input) {
            let candela = captures.get(1).and_then(|m| m.as_str().parse().ok());
            let alba = captures.get(2).and_then(|m| m.as_str().parse().ok());
            let penta = captures.get(3).and_then(|m| m.as_str().parse().ok());
            let month_abbrev = captures.get(4).unwrap().as_str();
            let year_value: u32 = captures.get(5).unwrap().as_str().parse()
                .context("Invalid year")?;
            let era_abbrev = captures.get(6).unwrap().as_str();
            
            // Validate components
            if let Some(c) = candela {
                if c < 1 || c > config.config.day_structure.candles_per_block {
                    anyhow::bail!("Candela must be 1-{}", config.config.day_structure.candles_per_block);
                }
            }
            
            if let Some(a) = alba {
                if a < 1 || a > config.config.calendar.days_per_week {
                    anyhow::bail!("Alba must be 1-{}", config.config.calendar.days_per_week);
                }
            }
            
            if let Some(p) = penta {
                if p < 1 || p > config.config.calendar.weeks_per_month {
                    anyhow::bail!("Penta must be 1-{}", config.config.calendar.weeks_per_month);
                }
            }
            
            // Validate month exists
            let month_name = config.config.months.get_name_by_abbrev(month_abbrev)
                .context("Unknown month abbreviation")?;
            
            // Create year and get absolute year
            let year = Year::new(year_value, era_abbrev, &config.era_events_by_abbrev)
                .context("Invalid era or year")?;
                
            Ok(TimelineDate::Fantasy {
                candela,
                alba,
                penta,
                month: Some(month_name.to_string()),
                year: year_value,
                era: era_abbrev.to_string(),
                absolute_year: year.absolute_year,
                raw_input: input.to_string(),
            })
        } else {
            // Try full format parsing (more complex, simplified for now)
            anyhow::bail!("Could not parse date format: {}", input);
        }
    }
    
    pub fn sort_key(&self) -> u64 {
        match self {
            TimelineDate::Fantasy { absolute_year, alba, penta, candela, .. } => {
                // Create sortable key: year * 1000000 + month * 10000 + penta * 1000 + alba * 100 + candela
                let mut key = (*absolute_year as u64) * 1_000_000;
                
                // Add month, penta, alba, candela if available
                if let Some(p) = penta {
                    key += (*p as u64) * 1000;
                }
                if let Some(a) = alba {
                    key += (*a as u64) * 100;
                }
                if let Some(c) = candela {
                    key += *c as u64;
                }
                
                key
            },
            TimelineDate::Real { datetime } => {
                // Convert to timestamp for sorting
                datetime.timestamp() as u64
            }
        }
    }
}

impl fmt::Display for TimelineDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimelineDate::Fantasy { raw_input, .. } => {
                write!(f, "{}", raw_input)
            },
            TimelineDate::Real { datetime } => {
                write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S UTC"))
            }
        }
    }
}

impl PartialEq for TimelineDate {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl Eq for TimelineDate {}

impl PartialOrd for TimelineDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimelineDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}