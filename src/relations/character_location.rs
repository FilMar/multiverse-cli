//! Character-Location relation implementation
//! Syntax: character create john --set location=glass_gardens*resident

use crate::define_relation;
use crate::character::models::Character;
use crate::location::models::Location;

// Use the generic macro to define CharacterLocationRelation struct and table
define_relation!(
    CharacterLocationRelation,
    Character -> Location,
    table: "character_location_relations",
    from_table: "characters",
    to_table: "locations", 
    fields: {
        relationship_type: String,
    }
);

// Additional implementation specific to this relation type
impl CharacterLocationRelation {
    /// Parse "glass_gardens*resident" format into resolved IDs and relationship type
    pub fn parse_from_value(character_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'location_name*relationship_type'", 
                value
            ));
        }

        let location_name = parts[0];
        let relationship_type = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "unknown".to_string()
        };

        // Resolve character name to ID
        let character_id = Character::resolve_id(character_name)?;
        
        // Resolve location name to ID (this also validates it exists)
        let location_id = Location::resolve_id(location_name)?;

        Ok(CharacterLocationRelation::new(
            character_id,
            location_id,
            relationship_type,
        ))
    }

}

// Main processor function that uses the struct
pub fn process_character_location_relations(character_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let location_name = parts[0];
        
        let relation = CharacterLocationRelation::parse_from_value(character_name, relation_part)?;
        let is_new = relation.upsert()?;
        
        // Print appropriate success message
        if is_new {
            println!("✅ Created relation: {} -> {}", character_name, location_name);
        } else {
            println!("🔄 Updated relation: {} <-> {}", character_name, location_name);
        }
    }

    Ok(())
}