//! Character-Race relation implementation
//! Syntax: character create legolas --set race=high_elves*pureblooded

use crate::define_relation;
use crate::character::models::Character;
use crate::race::models::Race;

// Use the generic macro to define CharacterRaceRelation struct and table
define_relation!(
    CharacterRaceRelation,
    Character -> Race,
    table: "character_race_relations",
    from_table: "characters",
    to_table: "races", 
    fields: {
        heritage: String,
    }
);

// Additional implementation specific to this relation type
impl CharacterRaceRelation {
    /// Parse "high_elves*pureblooded" format into resolved IDs and heritage
    pub fn parse_from_value(character_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'race_name*heritage'", 
                value
            ));
        }

        let race_name = parts[0];
        let heritage = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "standard".to_string()
        };

        // Resolve character name to ID
        let character_id = Character::resolve_id(character_name)?;
        
        // Resolve race name to ID (this also validates it exists)
        let race_id = Race::resolve_id(race_name)?;

        Ok(CharacterRaceRelation::new(
            character_id,
            race_id,
            heritage,
        ))
    }

}

// Main processor function that uses the struct
pub fn process_character_race_relations(character_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let race_name = parts[0];
        
        let relation = CharacterRaceRelation::parse_from_value(character_name, relation_part)?;
        let is_new = relation.upsert()?;
        
        // Print appropriate success message
        if is_new {
            println!("✅ Created relation: {} -> {}", character_name, race_name);
        } else {
            println!("🔄 Updated relation: {} <-> {}", character_name, race_name);
        }
    }

    Ok(())
}