//! Race-System relation implementation
//! Syntax: race update high_elves --set system=aetherial_magic*natural

use crate::define_relation;
use crate::race::models::Race;
use crate::system::models::System;

// Use the generic macro to define RaceSystemRelation struct and table
define_relation!(
    RaceSystemRelation,
    Race -> System,
    table: "race_system_relations",
    from_table: "races",
    to_table: "systems", 
    fields: {
        affinity: String,
    }
);

// Additional implementation specific to this relation type
impl RaceSystemRelation {
    /// Parse "aetherial_magic*natural" format into resolved system name and affinity
    pub fn parse_from_value(race_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'system_name*affinity'", 
                value
            ));
        }

        let system_name = parts[0];
        let affinity = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "compatible".to_string()
        };

        // Resolve race and system names to IDs
        let race_id = Race::resolve_id(race_name)?;
        let system_id = System::resolve_id(system_name)?;

        Ok(RaceSystemRelation::new(
            race_id,
            system_id,
            affinity,
        ))
    }

}

// Main processor function that uses the struct
pub fn process_race_system_relations(race_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let system_name = parts[0];
        
        let relation = RaceSystemRelation::parse_from_value(race_name, relation_part)?;
        let is_new = relation.upsert()?;
        
        // Print appropriate success message
        if is_new {
            println!("✅ Created relation: {} -> {}", race_name, system_name);
        } else {
            println!("🔄 Updated relation: {} <-> {}", race_name, system_name);
        }
    }

    Ok(())
}