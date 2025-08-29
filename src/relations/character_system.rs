use crate::define_relation;
use crate::character::models::Character;
use crate::system::models::System;

define_relation!(
    CharacterSystemRelation,
    Character -> System,
    table: "character_system_relations",
    from_table: "characters",
    to_table: "systems", 
    fields: {
        usage_type: String,
    }
);

pub fn process_character_system_relations(character_name: &str, relations: &str) -> anyhow::Result<()> {
    println!("🔗 Processing character-system relations for '{}'", character_name);
    
    let parts: Vec<&str> = relations.split(',').collect();
    
    for part in parts {
        let relation_parts: Vec<&str> = part.split('*').collect();
        let system_name = relation_parts[0].trim();
        let usage_type = if relation_parts.len() > 1 {
            relation_parts[1].trim().to_string()
        } else {
            "uses".to_string() // Default usage type
        };
        
        // Resolve character and system names to IDs
        let character_id = Character::resolve_id(character_name)?;
        let system_id = System::resolve_id(system_name)?;
        
        // Create the relation with IDs
        let relation = CharacterSystemRelation::new(
            character_id,
            system_id,
            usage_type.clone(),
        );
        
        relation.create()?;
        println!("✅ Created relation: {} -> {} ({})", character_name, system_name, usage_type);
    }
    
    Ok(())
}