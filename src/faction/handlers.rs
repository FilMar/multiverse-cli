use super::cli::FactionCommands;
use super::models::{Faction, FactionStatus};
use anyhow::Result;

pub fn handle_faction_command(command: FactionCommands) -> Result<()> {
    match command {
        FactionCommands::Create { name, display_name, faction_type, mut set } => {
            set.push(("display_name".to_string(), display_name));
            set.push(("faction_type".to_string(), faction_type));
            handle_create(name, set)
        }
        FactionCommands::List => handle_list(),
        FactionCommands::Info { name } => handle_info(name),
        FactionCommands::Delete { name, force } => handle_delete(name, force),
        FactionCommands::Update { name, display_name, faction_type, set } => handle_update(name, display_name, faction_type, set),
    }
}

fn handle_update(name: String, display_name: Option<String>, faction_type: Option<String>, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating faction '{}'", name);

    let mut faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;

    if let Some(display_name) = display_name {
        faction.display_name = display_name;
    }
    if let Some(faction_type) = faction_type {
        faction.faction_type = faction_type;
    }

    faction.update(set_args)?;

    println!("✅ Faction '{}' updated!", name);
    show_created_faction(&faction)?;

    Ok(())
}

fn handle_create(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("⚔️  Creating faction '{}'", name);
    
    let mut faction = Faction::create_new(name.clone(), set_args)?;
    faction.create()?;
    
    show_created_faction(&faction)?;
    
    Ok(())
}

fn show_created_faction(faction: &Faction) -> Result<()> {
    println!("✅ Faction '{}' created!", faction.name);
    println!("   Display name: {}", faction.display_name);
    println!("   Type: {}", faction.faction_type);
    println!("   Status: {:?}", faction.status);
    
    if let Some(desc) = &faction.description {
        println!("   Description: {desc}");
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

fn handle_list() -> Result<()> {
    let factions = Faction::list()?;
    
    if factions.is_empty() {
        println!("⚔️  No factions found in this world");
        println!("   Use 'multiverse faction create <name> --display-name <name> --type <type>' to create one");
        return Ok(())
    }
    
    println!("⚔️  Factions in current world:");
    
    for faction in factions {
        let status_emoji = match faction.status {
            FactionStatus::Active => "🟢",
            FactionStatus::Disbanded => "💥",
            FactionStatus::Dormant => "🟡",
            FactionStatus::Archived => "📦",
        };
        
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            faction.name,
            faction.display_name,
            faction.faction_type
        );
        
        // Show key metadata fields
        if let Some(size) = faction.metadata.get("size") {
            println!("      Size: {}", size.as_str().unwrap_or("Unknown"));
        }
        if let Some(alignment) = faction.metadata.get("alignment") {
            println!("      Alignment: {}", alignment.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(desc) = &faction.description {
            println!("      {desc}");
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;
    
    println!("⚔️  Faction: {} - \"{}\"", faction.name, faction.display_name);
    println!("   Type: {}", faction.faction_type);
    println!("   Status: {:?}", faction.status);
    println!("   Created: {}", faction.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = &faction.description {
        println!("   Description: {desc}");
    }
    
    // Show metadata
    if !faction.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &faction.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    // TODO: Show members, enemies, territories
    println!("   Members: (to be implemented)");
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let faction = Faction::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Faction '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete faction '{}'?", name);
        println!("   This will permanently delete the faction and remove it from all references");
        println!("   Use --force to skip this confirmation");
        return Ok(())
    }
    
    println!("🗑️  Deleting faction '{}'...", name);
    
    faction.delete(force)?;
    
    println!("✅ Faction '{}' deleted!", name);
    
    Ok(())
}
