//! Location-Faction relation implementation
//! Syntax: location update minas_tirith --set faction=gondor*capital

use crate::define_relation;
use crate::location::models::Location;
use crate::faction::models::Faction;

// Use the generic macro to define LocationFactionRelation struct and table
define_relation!(
    LocationFactionRelation,
    Location -> Faction,
    table: "location_faction_relations",
    from_table: "locations",
    to_table: "factions", 
    fields: {
        control_type: String,
    }
);

// Additional implementation specific to this relation type
impl LocationFactionRelation {
    /// Parse "gondor*capital" format into resolved IDs and control type
    pub fn parse_from_value(location_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'faction_name*control_type'", 
                value
            ));
        }

        let faction_name = parts[0];
        let control_type = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "controlled".to_string()
        };

        // Resolve location and faction names to IDs
        let location_id = Location::resolve_id(location_name)?;
        let faction_id = Faction::resolve_id(faction_name)?;

        Ok(LocationFactionRelation::new(
            location_id,
            faction_id,
            control_type,
        ))
    }
}

// Main processor function that uses the struct
pub fn process_location_faction_relations(location_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let faction_name = parts[0];
        
        let relation = LocationFactionRelation::parse_from_value(location_name, relation_part)?;
        relation.create()?;
        
        // Print success message with original names
        println!("✅ Created relation: {} -> {}", location_name, faction_name);
    }

    Ok(())
}