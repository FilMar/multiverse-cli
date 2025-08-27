use super::cli::FactionCommands;
use super::models::Faction;
use anyhow::Result;

pub fn handle_faction_command(command: FactionCommands) -> Result<()> {
    match command {
        FactionCommands::Create { name, set } => {
            handle_create(name, set)
        }
        FactionCommands::List => handle_list(),
        FactionCommands::Info { name } => handle_info(name),
        FactionCommands::Delete { name, force } => handle_delete(name, force),
        FactionCommands::Update { name, set } => handle_update(name, set),
    }
}

fn handle_update(name: String, mut set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating faction '{name}'");

    let mut faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;

    // Normalize field names: title -> display_name
    for (key, _) in &mut set_args {
        if key == "title" {
            *key = "display_name".to_string();
        }
    }

    faction.update(set_args)?;

    println!("✅ Faction '{}' updated!", name);
    show_created_faction(&faction)?;

    Ok(())
}

fn handle_create(name: String, mut set_args: Vec<(String, String)>) -> Result<()> {
    let title = set_args.iter()
        .find(|(k, _)| k == "title" || k == "display_name")
        .map(|(_, v)| v.as_str())
        .unwrap_or(&name);
    
    println!("⚔️ Creating faction '{name}' ({})", title);

    // Normalize field names: title -> display_name
    for (key, _) in &mut set_args {
        if key == "title" {
            *key = "display_name".to_string();
        }
    }

    // Use Faction factory method with built-in validation
    let mut faction = Faction::create_new(name.clone(), set_args)?;
    faction.create()?;
    
    // Display success information
    show_created_faction(&faction)?;
    
    Ok(())
}

fn show_created_faction(faction: &Faction) -> Result<()> {
    println!("✅ Faction '{}' created!", faction.name);
    println!("   Title: {}", faction.display_name);
    println!("   Status: {:?}", faction.status);
    
    // Show metadata
    if !faction.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &faction.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let factions = Faction::list()?;
    
    if factions.is_empty() {
        println!("⚔️ No factions found in this world");
        println!("   Use 'multiverse faction create <name> --set title=\"<title>\"' to create one");
        return Ok(());
    }
    
    println!("⚔️ Factions in current world:");
    
    for faction in factions {
        let status_emoji = match faction.status {
            crate::faction::models::FactionStatus::Active => "🟢",
            crate::faction::models::FactionStatus::Inactive => "⚫",
            crate::faction::models::FactionStatus::Disbanded => "💥", 
            crate::faction::models::FactionStatus::Allied => "🤝",
            crate::faction::models::FactionStatus::Hostile => "⚔️",
        };
        
        println!("   {} {} - \"{}\"", 
            status_emoji, 
            faction.name, 
            faction.display_name
        );
        
        // Show key metadata fields
        if let Some(faction_type) = faction.metadata.get("type") {
            println!("      Type: {}", faction_type.as_str().unwrap_or("Unknown"));
        }
        if let Some(description) = faction.metadata.get("description") {
            println!("      {}", description.as_str().unwrap_or(""));
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;
    
    println!("⚔️ Faction: {} - \"{}\"", faction.name, faction.display_name);
    println!("   Status: {:?}", faction.status);
    println!("   Created: {}", faction.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = faction.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
    }
    
    // Show metadata
    if !faction.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &faction.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete faction '{name}'?");
        println!("   This will permanently delete the faction from database");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting faction '{name}'...");
    
    faction.delete(force)?;
    
    println!("✅ Faction '{name}' deleted!");
    
    Ok(())
}