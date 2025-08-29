use crate::define_relation;
use crate::event::models::Event;
use crate::location::models::Location;

define_relation!(
    EventLocationRelation,
    Event -> Location,
    table: "event_location_relations",
    from_table: "events",
    to_table: "locations", 
    fields: {
        location_role: String,
    }
);

pub fn process_event_location_relations(event_name: &str, relations: &str) -> anyhow::Result<()> {
    println!("🔗 Processing event-location relations for '{}'", event_name);
    
    let parts: Vec<&str> = relations.split(',').collect();
    
    for part in parts {
        let relation_parts: Vec<&str> = part.split('*').collect();
        let location_name = relation_parts[0].trim();
        let location_role = if relation_parts.len() > 1 {
            relation_parts[1].trim().to_string()
        } else {
            "takes_place_at".to_string() // Default location role
        };
        
        // Resolve event and location names to IDs
        let event_id = Event::resolve_id(event_name)?;
        let location_id = Location::resolve_id(location_name)?;
        
        // Create the relation with IDs
        let relation = EventLocationRelation::new(
            event_id,
            location_id,
            location_role.clone(),
        );
        
        relation.create()?;
        println!("✅ Created relation: {} -> {} ({})", event_name, location_name, location_role);
    }
    
    Ok(())
}