//! Character-Episode relation implementation
//! Syntax: character create john --set episode=story:2*protagonista

use crate::define_relation;
use crate::character::models::Character;
use crate::episode::models::Episode;

// Use the generic macro to define CharacterEpisodeRelation struct and table
define_relation!(
    CharacterEpisodeRelation,
    Character -> Episode,
    table: "character_episode_relations",
    from_table: "characters",
    to_table: "episodes", 
    fields: {
        role: String,
    }
);

// Additional implementation specific to this relation type
impl CharacterEpisodeRelation {
    /// Parse "story:2*protagonista" format into resolved episode ID and role
    pub fn parse_from_value(character_name: &str, value: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '*').collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid format: '{}'. Expected 'story:episode*role'", 
                value
            ));
        }

        let identifier = parts[0];
        let role = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "unknown".to_string()
        };

        // Resolve the actual episode ID
        let episode_id = Episode::resolve_id(identifier)?;

        // Resolve character ID from character name
        let character_id = Character::resolve_id(character_name)?;
        
        Ok(CharacterEpisodeRelation::new(
            character_id,
            episode_id,
            role,
        ))
    }

}

// Main processor function that uses the struct
pub fn process_character_episode_relations(character_name: &str, relations_str: &str) -> anyhow::Result<()> {
    // Parse multiple relations (comma-separated)
    for relation_part in relations_str.split(',') {
        let relation_part = relation_part.trim();
        if relation_part.is_empty() {
            continue;
        }

        // Parse and create relation using the struct
        let parts: Vec<&str> = relation_part.splitn(2, '*').collect();
        let episode_identifier = parts[0];
        
        let relation = CharacterEpisodeRelation::parse_from_value(character_name, relation_part)?;
        relation.create()?;
        
        // Print success message with original names
        println!("✅ Created relation: {} -> {}", character_name, episode_identifier);
    }

    Ok(())
}