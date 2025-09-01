use crate::define_relation;
use crate::event::models::Event;
use crate::faction::models::Faction;

define_relation!(
    EventFactionRelation,
    Event -> Faction,
    table: "event_faction_relations",
    from_table: "events",
    to_table: "factions", 
    fields: {
        faction_role: String,
    }
);

pub fn process_event_faction_relations(event_name: &str, relations: &str) -> anyhow::Result<()> {
    println!("🔗 Processing event-faction relations for '{}'", event_name);
    
    let parts: Vec<&str> = relations.split(',').collect();
    
    for part in parts {
        let relation_parts: Vec<&str> = part.split('*').collect();
        let faction_name = relation_parts[0].trim();
        let faction_role = if relation_parts.len() > 1 {
            relation_parts[1].trim().to_string()
        } else {
            "involved_in".to_string() // Default faction role
        };
        
        // Resolve event and faction names to IDs
        let event_id = Event::resolve_id(event_name)?;
        let faction_id = Faction::resolve_id(faction_name)?;
        
        // Create the relation with IDs
        let relation = EventFactionRelation::new(
            event_id,
            faction_id,
            faction_role.clone(),
        );
        
        let is_new = relation.upsert()?;
        
        if is_new {
            println!("✅ Created relation: {} -> {} ({})", event_name, faction_name, faction_role);
        } else {
            println!("🔄 Updated relation: {} <-> {} ({})", event_name, faction_name, faction_role);
        }
    }
    
    Ok(())
}