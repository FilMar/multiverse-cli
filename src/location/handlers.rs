use super::cli::LocationCommands;
use super::models::{Location, LocationStatus};
use anyhow::Result;

pub fn handle_location_command(command: LocationCommands) -> Result<()> {
    match command {
        LocationCommands::Create { name, display_name, set } => {
            handle_create(name, display_name, set)
        }
        LocationCommands::List => handle_list(),
        LocationCommands::Info { name } => handle_info(name),
        LocationCommands::Delete { name, force } => handle_delete(name, force),
        LocationCommands::Update { name, display_name, set } => handle_update(name, display_name, set),
    }
}

fn handle_update(name: String, display_name: Option<String>, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating location '{name}'");

    let mut location = Location::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Location '{}' not found", name))?;

    location.update(display_name, set_args)?;

    println!("✅ Location '{}' updated!", name);
    show_created_location(&location)?;

    Ok(())
}

fn handle_create(name: String, display_name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🏛️  Creating location '{name}' ({display_name})");
    
    let location = Location::create_new(name.clone(), display_name, set_args)?;
    location.create()?;
    
    show_created_location(&location)?;
    
    Ok(())
}

fn show_created_location(location: &Location) -> Result<()> {
    println!("✅ Location '{}' created!", location.name);
    println!("   Display name: {}", location.display_name);
    println!("   Status: {:?}", location.status);
    
    if let Some(type_val) = location.metadata.get("type") {
        println!("   Type: {}", type_val.as_str().unwrap_or("Unknown"));
    }
    
    if let Some(desc) = location.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
    }
    
    // Show metadata
    if !location.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &location.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let locations = Location::list()?;
    
    if locations.is_empty() {
        println!("🏛️  No locations found in this world");
        println!("   Use 'multiverse location create <name> --display-name <name> --type <type>' to create one");
        return Ok(());
    }
    
    println!("🏛️  Locations in current world:");
    
    for location in locations {
        let status_emoji = match location.status {
            LocationStatus::Active => "🟢",
            LocationStatus::Destroyed => "💥",
            LocationStatus::Abandoned => "🏚️",
            LocationStatus::Archived => "📦",
        };
        
        let type_str = location.metadata.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
            
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            location.name, 
            location.display_name,
            type_str
        );
        
        // Show key metadata fields
        if let Some(population) = location.metadata.get("population") {
            println!("      Population: {}", population.as_str().unwrap_or("Unknown"));
        }
        if let Some(climate) = location.metadata.get("climate") {
            println!("      Climate: {}", climate.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(desc) = location.metadata.get("description") {
            println!("      {}", desc.as_str().unwrap_or(""));
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let location = Location::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Location '{}' not found", name))?;
    
    println!("🏛️  Location: {} - \"{}\"", location.name, location.display_name);
    
    if let Some(type_val) = location.metadata.get("type") {
        println!("   Type: {}", type_val.as_str().unwrap_or("Unknown"));
    }
    
    println!("   Status: {:?}", location.status);
    println!("   Created: {}", location.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = location.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
    }
    
    // Show metadata
    if !location.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &location.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    // TODO: Show episodes where location appears
    println!("   Episodes: (to be implemented)");
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let location = Location::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Location '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete location '{name}'?");
        println!("   This will permanently delete the location and remove it from all episodes");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting location '{name}'...");
    
    location.delete(force)?;
    
    println!("✅ Location '{name}' deleted!");
    
    Ok(())
}