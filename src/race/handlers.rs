use super::cli::RaceCommands;
use super::models::Race;
use super::database;
use crate::database::get_connection;
use crate::world::config::WorldConfig;
use anyhow::{Result, Context};
use std::collections::HashMap;

pub fn handle_race_command(command: RaceCommands) -> Result<()> {
    match command {
        RaceCommands::Create { name, display_name, description, set } => {
            handle_create(name, display_name, description, set)
        }
        RaceCommands::List => handle_list(),
        RaceCommands::Info { name } => handle_info(name),
        RaceCommands::Delete { name, force } => handle_delete(name, force),
        RaceCommands::Update { name, display_name, description, set } => {
            handle_update(name, display_name, description, set)
        }
    }
}

fn handle_create(name: String, display_name: String, description: Option<String>, set_args: Vec<(String, String)>) -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;

    println!("✨ Creating race: {}", display_name);

    let mut metadata = HashMap::new();
    for (key, value) in set_args {
        let parsed_value = serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value));
        metadata.insert(key, parsed_value);
    }

    let race = Race {
        name: name.clone(),
        display_name,
        description,
        metadata,
        created_at: chrono::Utc::now(),
    };

    database::create_race(&conn, &race).context("Failed to create race in database")?;
    println!("✅ Race '{}' created successfully.", name);
    Ok(())
}

fn handle_list() -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;
    let races = database::list_races(&conn).context("Failed to list races")?;
    if races.is_empty() {
        println!("No races found in this world.");
    } else {
        println!("Races in this world:");
        for race in races {
            println!("- {} ({})", race.display_name, race.name);
        }
    }
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;
    match database::get_race(&conn, &name)? {
        Some(race) => {
            println!("Race Information:");
            println!("  Name: {}", race.name);
            println!("  Display Name: {}", race.display_name);
            if let Some(desc) = race.description {
                println!("  Description: {}", desc);
            }
            println!("  Created At: {}", race.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
            if !race.metadata.is_empty() {
                println!("  Metadata:");
                for (key, value) in race.metadata {
                    println!("    - {}: {}", key, value);
                }
            }
        }
        None => {
            println!("Race '{}' not found.", name);
        }
    }
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;
    if !force {
        println!("Are you sure you want to delete the race '{}'? This cannot be undone.", name);
        println!("Run the command again with --force to confirm.");
        return Ok(());
    }
    database::delete_race(&conn, &name).context("Failed to delete race")?;
    println!("🗑️ Race '{}' has been deleted.", name);
    Ok(())
}

fn handle_update(name: String, display_name: Option<String>, description: Option<String>, set_args: Vec<(String, String)>) -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;

    println!("🔄 Updating race '{}'", name);

    let mut race = database::get_race(&conn, &name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;

    if let Some(display_name) = display_name {
        race.display_name = display_name;
    }

    if let Some(description) = description {
        race.description = Some(description);
    }

    for (key, value) in set_args {
        race.metadata.insert(key, serde_json::Value::String(value));
    }

    database::update_race(&conn, &race)?;

    println!("✅ Race '{}' updated!", name);
    show_created_race(&race)?;

    Ok(())
}

fn show_created_race(race: &Race) -> Result<()> {
    println!("✅ Race '{}' created!", race.name);
    println!("   Display name: {}", race.display_name);
    
    if let Some(desc) = &race.description {
        println!("   Description: {desc}");
    }
    
    // Show metadata
    if !race.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &race.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}
