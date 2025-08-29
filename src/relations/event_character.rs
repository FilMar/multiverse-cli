use crate::define_relation;
use crate::event::models::Event;
use crate::character::models::Character;

define_relation!(
    EventCharacterRelation,
    Event -> Character,
    table: "event_character_relations",
    from_table: "events",
    to_table: "characters", 
    fields: {
        participation_type: String,
    }
);

pub fn process_event_character_relations(event_name: &str, relations: &str) -> anyhow::Result<()> {
    println!("🔗 Processing event-character relations for '{}'", event_name);
    
    let parts: Vec<&str> = relations.split(',').collect();
    
    for part in parts {
        let relation_parts: Vec<&str> = part.split('*').collect();
        let character_name = relation_parts[0].trim();
        let participation_type = if relation_parts.len() > 1 {
            relation_parts[1].trim().to_string()
        } else {
            "participant".to_string() // Default participation type
        };
        
        // Resolve event and character names to IDs
        let event_id = Event::resolve_id(event_name)?;
        let character_id = Character::resolve_id(character_name)?;
        
        // Create the relation with IDs
        let relation = EventCharacterRelation::new(
            event_id,
            character_id,
            participation_type.clone(),
        );
        
        relation.create()?;
        println!("✅ Created relation: {} -> {} ({})", event_name, character_name, participation_type);
    }
    
    Ok(())
}