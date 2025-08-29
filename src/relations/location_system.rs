use crate::define_relation;
use crate::location::models::Location;
use crate::system::models::System;

define_relation!(
    LocationSystemRelation,
    Location -> System,
    table: "location_system_relations",
    from_table: "locations",
    to_table: "systems", 
    fields: {
        infrastructure_type: String,
    }
);

pub fn process_location_system_relations(location_name: &str, relations: &str) -> anyhow::Result<()> {
    println!("🔗 Processing location-system relations for '{}'", location_name);
    
    let parts: Vec<&str> = relations.split(',').collect();
    
    for part in parts {
        let relation_parts: Vec<&str> = part.split('*').collect();
        let system_name = relation_parts[0].trim();
        let infrastructure_type = if relation_parts.len() > 1 {
            relation_parts[1].trim().to_string()
        } else {
            "has".to_string() // Default infrastructure type
        };
        
        // Resolve location and system names to IDs
        let location_id = Location::resolve_id(location_name)?;
        let system_id = System::resolve_id(system_name)?;
        
        // Create the relation with IDs
        let relation = LocationSystemRelation::new(
            location_id,
            system_id,
            infrastructure_type.clone(),
        );
        
        relation.create()?;
        println!("✅ Created relation: {} -> {} ({})", location_name, system_name, infrastructure_type);
    }
    
    Ok(())
}