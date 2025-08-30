//! Relation handlers - extract and process relation fields from --set parameters

use anyhow::Result;
use crate::relations::{process_character_episode_relations, process_character_location_relations, process_character_faction_relations, process_character_race_relations, process_character_system_relations, process_race_system_relations, process_location_faction_relations, process_location_location_relations, process_location_system_relations, process_event_character_relations, process_event_location_relations, process_event_faction_relations};



pub fn separate_relation_fields(set_args: Vec<(String, String)>, relation_keys: &[&str]) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let mut relation_fields = Vec::new();
    let mut regular_fields = Vec::new();
    for (key, value) in set_args {
        if relation_keys.contains(&key.as_str()){
            relation_fields.push((key, value));
        }
        else {
            regular_fields.push((key, value));
        }
    }
    (relation_fields, regular_fields)
}

/// Parse entity spec into name and role
fn parse_entity_role(spec: &str) -> (&str, &str) {
    if spec.contains('*') {
        let parts: Vec<&str> = spec.split('*').collect();
        (parts[0], parts.get(1).unwrap_or(&"unknown"))
    } else {
        (spec, "unknown")
    }
}

/// Process reverse relations generically
fn process_reverse_relation<F>(
    from_entity: &str,
    combined: &str, 
    processor: F
) -> Result<()>
where 
    F: Fn(&str, &str) -> Result<()>
{
    for spec in combined.split(',') {
        let spec = spec.trim();
        if !spec.is_empty() {
            let (entity_name, role) = parse_entity_role(spec);
            let relation_spec = format!("{}*{}", from_entity, role);
            processor(entity_name, &relation_spec)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum EntityType {
    Character(String),
    Episode(String),
    Location(String), 
    Faction(String),
    Race(String),
    Event(String),
    Story(String),
    System(String),
}

/// Process all relation fields from --set parameters
/// Takes entity type with name and all --set parameters
/// Processes relation ones based on key name and entity type, returns the remaining non-relation parameters
pub fn process_relations(
    entity: EntityType,
    mut set_args: Vec<(String, String)>
) -> Result<Vec<(String, String)>> {
    let mut processed_relations = Vec::new();
    
    // Group relation values by type  
    let mut relation_groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    // Extract all relation fields and group them
    set_args.retain(|(key, value)| {
        let is_relation = match (key.as_str(), &entity) {
            // Character relations
            ("episode", EntityType::Character(_)) => true,
            ("location", EntityType::Character(_)) => true,
            ("faction", EntityType::Character(_)) => true,
            ("race", EntityType::Character(_)) => true,
            ("system", EntityType::Character(_)) => true,
            
            // Episode relations (reverse)
            ("character", EntityType::Episode(_)) => true,
            
            // Location relations
            ("faction", EntityType::Location(_)) => true,
            ("location", EntityType::Location(_)) => true,
            ("system", EntityType::Location(_)) => true,
            ("character", EntityType::Location(_)) => true, // reverse
            
            // Faction relations (reverse)
            ("character", EntityType::Faction(_)) => true,
            ("location", EntityType::Faction(_)) => true,
            ("event", EntityType::Faction(_)) => true,
            
            // Race relations
            ("system", EntityType::Race(_)) => true,
            ("character", EntityType::Race(_)) => true, // reverse
            
            // Event relations
            ("character", EntityType::Event(_)) => true,
            ("location", EntityType::Event(_)) => true,
            ("faction", EntityType::Event(_)) => true,
            
            // Story relations (reverse)
            ("episode", EntityType::Story(_)) => true,
            
            // System relations (reverse)
            ("character", EntityType::System(_)) => true,
            ("location", EntityType::System(_)) => true,
            ("race", EntityType::System(_)) => true,
            
            _ => false,
        };
        
        if is_relation {
            relation_groups.entry(key.clone()).or_insert_with(Vec::new).push(value.clone());
            false // Remove from original list
        } else {
            true // Keep non-relation fields
        }
    });
    
    // Process each relation type using perfect pattern matching
    for (relation_type, values) in relation_groups {
        if !values.is_empty() {
            let combined = values.join(",");
            
            match (relation_type.as_str(), &entity) {
                // Character relations (forward)
                ("episode", EntityType::Character(name)) => {
                    process_character_episode_relations(name, &combined)?;
                    processed_relations.push("episode");
                },
                ("location", EntityType::Character(name)) => {
                    process_character_location_relations(name, &combined)?;
                    processed_relations.push("location");
                },
                ("faction", EntityType::Character(name)) => {
                    process_character_faction_relations(name, &combined)?;
                    processed_relations.push("faction");
                },
                ("race", EntityType::Character(name)) => {
                    process_character_race_relations(name, &combined)?;
                    processed_relations.push("race");
                },
                ("system", EntityType::Character(name)) => {
                    process_character_system_relations(name, &combined)?;
                    processed_relations.push("system");
                },
                
                // Episode relations (reverse)
                ("character", EntityType::Episode(episode_id)) => {
                    process_reverse_relation(episode_id, &combined, process_character_episode_relations)?;
                    processed_relations.push("character");
                },
                
                // Location relations (forward and reverse)
                ("faction", EntityType::Location(name)) => {
                    process_location_faction_relations(name, &combined)?;
                    processed_relations.push("faction");
                },
                ("location", EntityType::Location(name)) => {
                    process_location_location_relations(name, &[combined])?;
                    processed_relations.push("location");
                },
                ("system", EntityType::Location(name)) => {
                    process_location_system_relations(name, &combined)?;
                    processed_relations.push("system");
                },
                ("character", EntityType::Location(location_name)) => {
                    process_reverse_relation(location_name, &combined, process_character_location_relations)?;
                    processed_relations.push("character");
                },
                
                // Faction relations (reverse)
                ("character", EntityType::Faction(faction_name)) => {
                    process_reverse_relation(faction_name, &combined, process_character_faction_relations)?;
                    processed_relations.push("character");
                },
                ("location", EntityType::Faction(faction_name)) => {
                    process_reverse_relation(faction_name, &combined, process_location_faction_relations)?;
                    processed_relations.push("location");
                },
                ("event", EntityType::Faction(faction_name)) => {
                    process_reverse_relation(faction_name, &combined, process_event_faction_relations)?;
                    processed_relations.push("event");
                },
                
                // Race relations (forward and reverse)
                ("system", EntityType::Race(name)) => {
                    process_race_system_relations(name, &combined)?;
                    processed_relations.push("system");
                },
                ("character", EntityType::Race(race_name)) => {
                    process_reverse_relation(race_name, &combined, process_character_race_relations)?;
                    processed_relations.push("character");
                },
                
                // Event relations (forward)
                ("character", EntityType::Event(name)) => {
                    process_event_character_relations(name, &combined)?;
                    processed_relations.push("character");
                },
                ("location", EntityType::Event(name)) => {
                    process_event_location_relations(name, &combined)?;
                    processed_relations.push("location");
                },
                ("faction", EntityType::Event(name)) => {
                    process_event_faction_relations(name, &combined)?;
                    processed_relations.push("faction");
                },
                
                // System relations (reverse)
                ("character", EntityType::System(system_name)) => {
                    process_reverse_relation(system_name, &combined, process_character_system_relations)?;
                    processed_relations.push("character");
                },
                ("location", EntityType::System(system_name)) => {
                    process_reverse_relation(system_name, &combined, process_location_system_relations)?;
                    processed_relations.push("location");
                },
                ("race", EntityType::System(system_name)) => {
                    process_reverse_relation(system_name, &combined, process_race_system_relations)?;
                    processed_relations.push("race");
                },
                
                // Story relations (managed automatically)
                ("episode", EntityType::Story(_story_name)) => {
                    eprintln!("Warning: Story-episode relations are managed automatically. Episodes belong to stories by design.");
                    processed_relations.push("episode");
                },
                
                _ => {
                    eprintln!("Warning: Unsupported relation '{}' for entity {:?}", relation_type, entity);
                }
            }
        }
    }
    
    if !processed_relations.is_empty() {
        println!("🔗 Processed relations: {}", processed_relations.join(", "));
    }
    
    // Return remaining non-relation fields
    Ok(set_args)
}
