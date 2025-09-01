//! Character-Faction relation implementation
//! Syntax: character create aragorn --set faction=rangers*captain

use crate::define_relation;
use crate::character::models::Character;
use crate::faction::models::Faction;

// Use the generic macro to define CharacterFactionRelation struct and table
define_relation!(
    CharacterFactionRelation,
    Character -> Faction,
    table: "character_faction_relations",
    from_table: "characters",
    to_table: "factions", 
    fields: {
        role: String,
    }
);

// Additional implementation specific to this relation type
impl CharacterFactionRelation {
    /// Parse "rangers*captain" format into resolved IDs and role
    pub fn parse_from_value(character_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'faction_name*role'", 
                value
            ));
        }

        let faction_name = parts[0];
        let role = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "member".to_string()
        };

        // Resolve character name to ID
        let character_id = Character::resolve_id(character_name)?;
        
        // Resolve faction name to ID (this also validates it exists)
        let faction_id = Faction::resolve_id(faction_name)?;

        Ok(CharacterFactionRelation::new(
            character_id,
            faction_id,
            role,
        ))
    }

}

// Main processor function that uses the struct
pub fn process_character_faction_relations(character_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let faction_name = parts[0];
        
        let relation = CharacterFactionRelation::parse_from_value(character_name, relation_part)?;
        let is_new = relation.upsert()?;
        
        // Print appropriate success message
        if is_new {
            println!("✅ Created relation: {} -> {}", character_name, faction_name);
        } else {
            println!("🔄 Updated relation: {} <-> {}", character_name, faction_name);
        }
    }

    Ok(())
}