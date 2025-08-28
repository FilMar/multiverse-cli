use super::cli::RaceCommands;
use super::models::{Race, RaceStatus};
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

    let mut race = Race::create_new(name.clone(), set_args)?;
    race.create().context("Failed to create race in database")?;

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

    race.update(set_args)?;

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
