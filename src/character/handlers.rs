use super::cli::CharacterCommands;
use super::models::{Character, CharacterStatus};
use anyhow::Result;

pub fn handle_character_command(command: CharacterCommands) -> Result<()> {
    match command {
        CharacterCommands::Create { name, set } => {
            handle_create(name, set)
        }
        CharacterCommands::List => handle_list(),
        CharacterCommands::Info { name } => handle_info(name),
        CharacterCommands::Delete { name, force } => handle_delete(name, force),
        CharacterCommands::Update { name, set } => handle_update(name, set),
    }
}

fn handle_update(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating character '{name}'");

    let mut character = Character::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Character '{}' not found", name))?;

    character.update(set_args)?;

    println!("✅ Character '{}' updated!", name);
    show_created_character(&character)?;

    Ok(())
}

fn handle_create(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("👤 Creating character '{name}'");
    
    let mut character = Character::create_new(name.clone(), set_args)?;
    character.create()?;
    
    show_created_character(&character)?;
    
    Ok(())
}

fn show_created_character(character: &Character) -> Result<()> {
    println!("✅ Character '{}' created!", character.name);
    println!("   Display name: {}", character.display_name);
    println!("   Status: {:?}", character.status);
    // Show metadata
    if !character.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &character.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let characters = Character::list()?;
    
    if characters.is_empty() {
        println!("👤 No characters found in this world");
        println!("   Use 'multiverse character create <name> --set display_name=\"<name>\"' to create one");
        return Ok(());
    }
    
    println!("👤 Characters in current world:");
    
    for character in characters {
        let status_emoji = match character.status {
            CharacterStatus::Active => "🟢",
            CharacterStatus::Inactive => "🟡",
            CharacterStatus::Deceased => "💀",
            CharacterStatus::Archived => "📦",
        };
        println!("   {} {} - \"{}\"", 
            status_emoji, 
            character.name, 
            character.display_name
        );
        // Show key metadata fields
        if let Some(age) = character.metadata.get("age") {
            println!("      Age: {}", age.as_str().unwrap_or("Unknown"));
        }
        if let Some(faction) = character.metadata.get("faction") {
            println!("      Faction: {}", faction.as_str().unwrap_or("Unknown"));
        }
        if let Some(desc) = character.metadata.get("description") {
            println!("      {}", desc.as_str().unwrap_or(""));
        }
    }
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let character = Character::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Character '{}' not found", name))?;
    println!("👤 Character: {} - \"{}\"", character.name, character.display_name);
    println!("   Status: {:?}", character.status);
    println!("   Created: {}", character.created_at.format("%Y-%m-%d %H:%M"));
    if let Some(desc) = character.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
    }
    // Show metadata
    if !character.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &character.metadata {
            println!("     {}: {}", key, value);
        }
    }
    // TODO: Show episodes where character appears with roles
    println!("   Episodes: (to be implemented)");
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let character = Character::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Character '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete character '{name}'?");
        println!("   This will permanently delete the character and remove them from all episodes");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting character '{name}'...");
    
    character.delete(force)?;
    
    println!("✅ Character '{name}' deleted!");
    
    Ok(())
}
