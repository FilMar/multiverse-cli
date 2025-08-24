use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Era {
    pub name: String,
    pub abbrev: String,
    pub year: u32,  // Absolute year when this era starts
}

#[derive(Debug, Clone)]
pub struct Year {
    pub value: u32,
    pub era_abbrev: String,
    pub absolute_year: u32,
}

impl Year {
    pub fn new(value: u32, era_abbrev: &str, era_events: &HashMap<String, Era>) -> Option<Self> {
        let era = era_events.values().find(|e| e.abbrev == era_abbrev)?;
        
        let absolute_year = match era_abbrev.ends_with('F') {
            true => era.year + value,  // DF: after era event
            false => era.year - value, // AF: before era event  
        };
        
        Some(Year {
            value,
            era_abbrev: era_abbrev.to_string(),
            absolute_year,
        })
    }
    
    pub fn from_absolute(absolute_year: u32, era_events: &HashMap<String, Era>) -> Self {
        // Find the most recent era event before or at this absolute year
        let mut applicable_eras: Vec<_> = era_events.values()
            .filter(|era| era.year <= absolute_year)
            .collect();
        applicable_eras.sort_by_key(|era| era.year);
        
        if let Some(era) = applicable_eras.last() {
            let value = absolute_year - era.year;
            Year {
                value,
                era_abbrev: era.abbrev.clone(),
                absolute_year,
            }
        } else {
            // Before any era events, use creation year
            Year {
                value: absolute_year,
                era_abbrev: "AC".to_string(), // After Creation
                absolute_year,
            }
        }
    }
}