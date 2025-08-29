use super::cli::RaceCommands;
use super::models::{Race, RaceStatus};
use crate::relations::{process_relations, EntityType};
use anyhow::{Result, Context};

pub fn handle_race_command(command: RaceCommands) -> Result<()> {
    match command {
        RaceCommands::Create { name, set } => {
            handle_create(name, set)
        }
        RaceCommands::List => handle_list(),
        RaceCommands::Info { name } => handle_info(name),
        RaceCommands::Delete { name, force } => handle_delete(name, force),
        RaceCommands::Update { name, set } => {
            handle_update(name, set)
        }
    }
}

fn handle_create(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("✨ Creating race: {}", name);

    // Separate relation fields from regular fields  
    let (relation_fields, regular_fields) = separate_relation_fields(set_args);
    
    // Create race with regular fields FIRST
    let mut race = Race::create_new(name.clone(), regular_fields)?;
    race.create().context("Failed to create race in database")?;
    
    // THEN process relations after race exists in database
    if !relation_fields.is_empty() {
        process_relations(EntityType::Race(name.clone()), relation_fields)?;
    }

    show_created_race(&race)?;
    Ok(())
}

fn handle_list() -> Result<()> {
    let races = Race::list().context("Failed to list races")?;
    
    if races.is_empty() {
        println!("✨ No races found in this world");
        println!("   Use 'multiverse race create <name> --set display_name=\"<name>\"' to create one");
        return Ok(());
    }

    println!("✨ Races in current world:");

    for race in races {
        let status_emoji = match race.status {
            RaceStatus::Active => "🟢",
            RaceStatus::Inactive => "🟡",
            RaceStatus::Extinct => "🔴",
            RaceStatus::Legendary => "⭐",
            RaceStatus::Mythical => "✨",
        };

        println!(
            "   {} {} - \"{}\"",
            status_emoji, race.name, race.display_name
        );

        // Show key metadata fields
        if let Some(origin) = race.metadata.get("origin") {
            println!("      Origin: {}", origin.as_str().unwrap_or("Unknown"));
        }
        if let Some(lifespan) = race.metadata.get("lifespan") {
            println!("      Lifespan: {}", lifespan.as_str().unwrap_or("Unknown"));
        }

        if let Some(desc) = race.metadata.get("description") {
            println!("      {}", desc.as_str().unwrap_or(""));
        }
    }

    Ok(())
}

fn separate_relation_fields(set_args: Vec<(String, String)>) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let mut relation_fields = Vec::new();
    let mut regular_fields = Vec::new();
    
    for (key, value) in set_args {
        match key.as_str() {
            "system" => relation_fields.push((key, value)),
            // Add more relation types here as we implement them
            _ => regular_fields.push((key, value)),
        }
    }
    
    (relation_fields, regular_fields)
}

fn handle_info(name: String) -> Result<()> {
    let race = Race::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;

    println!("✨ Race: {} - \"{}\"", race.name, race.display_name);
    println!("   Status: {:?}", race.status);
    println!("   Created: {}", race.created_at.format("%Y-%m-%d %H:%M"));

    if let Some(desc) = race.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
    }

    // Show metadata
    if !race.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &race.metadata {
            if key != "description" {
                println!("     {}: {}", key, value);
            }
        }
    }

    // TODO: Show characters of this race
    println!("   Usage: (to be implemented)");

    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let _race = Race::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;

    if !force {
        println!("⚠️  Are you sure you want to delete race '{name}'?");
        println!(
            "   This will permanently delete the race and remove it from all character references"
        );
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }

    println!("🗑️  Deleting race '{name}'...");

    let race = Race::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;
    race.delete(force)?;

    println!("✅ Race '{name}' deleted!");

    Ok(())
}

fn handle_update(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating race '{}'", name);

    let mut race = Race::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;

    // Process relations and get back non-relation fields
    let regular_fields = process_relations(EntityType::Race(name.clone()), set_args)?;
    
    // Update regular fields
    race.update(regular_fields)?;

    println!("✅ Race '{}' updated!", name);
    show_created_race(&race)?;

    Ok(())
}

fn show_created_race(race: &Race) -> Result<()> {
    println!("   Display name: {}", race.display_name);
    println!("   Status: {:?}", race.status);
    
    if let Some(description) = race.metadata.get("description") {
        println!("   Description: {}", description.as_str().unwrap_or(""));
    }
    
    // Show metadata
    if !race.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &race.metadata {
            if key != "description" {
                println!("     {}: {}", key, value);
            }
        }
    }
    
    Ok(())
}
