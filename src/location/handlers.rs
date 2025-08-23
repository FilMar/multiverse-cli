use super::cli::LocationCommands;
use super::models::{Location, LocationStatus};
use anyhow::Result;

pub fn handle_location_command(command: LocationCommands) -> Result<()> {
    match command {
        LocationCommands::Create { name, display_name, location_type, set } => {
            handle_create(name, display_name, location_type, set)
        }
        LocationCommands::List => handle_list(),
        LocationCommands::Info { name } => handle_info(name),
        LocationCommands::Delete { name, force } => handle_delete(name, force),
    }
}

fn handle_create(name: String, display_name: String, location_type: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🏛️  Creating location '{name}' ({display_name})");
    
    let location = Location::create_new(name.clone(), display_name, location_type, set_args)?;
    location.create()?;
    
    show_created_location(&location)?;
    
    Ok(())
}

fn show_created_location(location: &Location) -> Result<()> {
    println!("✅ Location '{}' created!", location.name);
    println!("   Display name: {}", location.display_name);
    println!("   Type: {}", location.location_type);
    println!("   Status: {:?}", location.status);
    
    if let Some(desc) = &location.description {
        println!("   Description: {desc}");
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
        
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            location.name, 
            location.display_name,
            location.location_type
        );
        
        // Show key metadata fields
        if let Some(population) = location.metadata.get("population") {
            println!("      Population: {}", population.as_str().unwrap_or("Unknown"));
        }
        if let Some(climate) = location.metadata.get("climate") {
            println!("      Climate: {}", climate.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(desc) = &location.description {
            println!("      {desc}");
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let location = Location::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Location '{}' not found", name))?;
    
    println!("🏛️  Location: {} - \"{}\"", location.name, location.display_name);
    println!("   Type: {}", location.location_type);
    println!("   Status: {:?}", location.status);
    println!("   Created: {}", location.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = &location.description {
        println!("   Description: {desc}");
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