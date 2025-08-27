use super::cli::RaceCommands;
use super::models::Race;
use anyhow::{Result, Context};

pub fn handle_race_command(command: RaceCommands) -> Result<()> {
    match command {
        RaceCommands::Create { name, display_name, description, mut set } => {
            set.push(("display_name".to_string(), display_name));
            if let Some(description) = description {
                set.push(("description".to_string(), description));
            }
            handle_create(name, set)
        }
        RaceCommands::List => handle_list(),
        RaceCommands::Info { name } => handle_info(name),
        RaceCommands::Delete { name, force } => handle_delete(name, force),
        RaceCommands::Update { name, display_name, description, mut set } => {
            if let Some(display_name) = display_name {
                set.push(("display_name".to_string(), display_name));
            }
            if let Some(description) = description {
                set.push(("description".to_string(), description));
            }
            handle_update(name, set)
        }
    }
}

fn handle_create(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("✨ Creating race: {}", name);

    let race = Race::create_new(name.clone(), set_args)?;
    race.create().context("Failed to create race in database")?;

    println!("✅ Race '{}' created successfully.", name);
    Ok(())
}

fn handle_list() -> Result<()> {
    let races = Race::list().context("Failed to list races")?;
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
    match Race::get(&name)? {
        Some(race) => {
            println!("Race Information:");
            println!("  Name: {}", race.name);
            println!("  Display Name: {}", race.display_name);
            if let Some(description) = race.metadata.get("description") {
                println!("  Description: {}", description.as_str().unwrap_or_default());
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
    let race = Race::get(&name)?.ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;
    race.delete(force).context("Failed to delete race")?;
    println!("🗑️ Race '{}' has been deleted.", name);
    Ok(())
}

fn handle_update(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating race '{}'", name);

    let mut race = Race::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Race '{}' not found", name))?;

    race.update(set_args)?;

    println!("✅ Race '{}' updated!", name);
    show_created_race(&race)?;

    Ok(())
}

fn show_created_race(race: &Race) -> Result<()> {
    println!("✅ Race '{}' created!", race.name);
    println!("   Display name: {}", race.display_name);
    
    if let Some(description) = race.metadata.get("description") {
        println!("   Description: {}", description.as_str().unwrap_or_default());
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
