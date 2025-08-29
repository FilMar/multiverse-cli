use crate::define_relation;
use crate::location::models::Location;

define_relation!(
    LocationLocationRelation,
    Location -> Location,
    table: "location_location_relations",
    from_table: "locations", 
    to_table: "locations",
    fields: {
        relationship_type: String,
    }
);

pub fn process_location_location_relations(from_location: &str, relations: &[String]) -> anyhow::Result<()> {
    println!("🔗 Processing location-location relations for '{}'", from_location);
    
    for relation_str in relations {
        let parts: Vec<&str> = relation_str.split(',').collect();
        
        for part in parts {
            let relation_parts: Vec<&str> = part.split('*').collect();
            let to_location = relation_parts[0].trim();
            let relationship_type = if relation_parts.len() > 1 {
                relation_parts[1].trim().to_string()
            } else {
                "neighbor".to_string() // Default relationship type
            };
            
            // Resolve both location names to IDs
            let from_id = Location::resolve_id(from_location)?;
            let to_id = Location::resolve_id(to_location)?;
            
            // Create the relation with IDs
            let relation = LocationLocationRelation::new(
                from_id,
                to_id, 
                relationship_type.clone(),
            );
            
            relation.create()?;
            println!("✅ Created relation: {} -> {} ({})", from_location, to_location, relationship_type);
        }
    }
    
    Ok(())
}