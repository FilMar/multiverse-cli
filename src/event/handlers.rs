use super::cli::EventCommands;
use super::models::{Event, EventStatus};
use anyhow::Result;

pub fn handle_event_command(command: EventCommands) -> Result<()> {
    match command {
        EventCommands::Create { name, display_name, event_type, date, mut set } => {
            set.push(("display_name".to_string(), display_name));
            set.push(("event_type".to_string(), event_type));
            if let Some(date) = date {
                set.push(("date".to_string(), date));
            }
            handle_create(name, set)
        }
        EventCommands::List => handle_list(),
        EventCommands::Timeline => handle_timeline(),
        EventCommands::Info { name } => handle_info(name),
        EventCommands::Delete { name, force } => handle_delete(name, force),
        EventCommands::Update { name, display_name, event_type, date, mut set } => {
            if let Some(display_name) = display_name {
                set.push(("display_name".to_string(), display_name));
            }
            if let Some(event_type) = event_type {
                set.push(("event_type".to_string(), event_type));
            }
            if let Some(date) = date {
                set.push(("date".to_string(), date));
            }
            handle_update(name, set)
        }
    }
}

fn handle_update(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating event '{}'", name);

    let mut event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;

    event.update(set_args)?;

    println!("✅ Event '{}' updated!", name);
    show_created_event(&event)?;

    Ok(())
}

fn handle_create(name: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("📅 Creating event '{}'", name);
    
    let mut event = Event::create_new(name.clone(), set_args)?;
    event.update_sort_key()?;
    event.create()?;
    
    show_created_event(&event)?;
    
    Ok(())
}

fn show_created_event(event: &Event) -> Result<()> {
    println!("✅ Event '{}' created!", event.name);
    println!("   Display name: {}", event.display_name);
    println!("   Type: {}", event.event_type);
    println!("   Status: {:?}", event.status);
    println!("   Date: {}", event.date);
    
    if let Some(sort_key) = event.sort_key {
        println!("   Sort key: {}", sort_key);
    }
    
    if let Some(desc) = event.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or_default());
    }
    
    // Show metadata
    if !event.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &event.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let events = Event::list()?;
    
    if events.is_empty() {
        println!("📅 No events found in this world");
        println!("   Use 'multiverse event create <name> --display-name <name> --type <type>' to create one");
        return Ok(())
    }
    
    println!("📅 Events in current world:");
    
    for event in events {
        let status_emoji = match event.status {
            EventStatus::Historical => "📜",
            EventStatus::Ongoing => "🔥",
            EventStatus::Planned => "⏳",
            EventStatus::Cancelled => "❌",
            EventStatus::Archived => "📦",
        };
        
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            event.name, 
            event.display_name,
            event.event_type
        );
        
        // Show key metadata fields
        if let Some(year) = event.metadata.get("year") {
            println!("      Year: {}", year.as_str().unwrap_or("Unknown"));
        }
        if let Some(importance) = event.metadata.get("importance") {
            println!("      Importance: {}", importance.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(desc) = event.metadata.get("description") {
            println!("      {desc}");
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;
    
    println!("📅 Event: {} - \"{}\"", event.name, event.display_name);
    println!("   Type: {}", event.event_type);
    println!("   Status: {:?}", event.status);
    println!("   Created: {}", event.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = event.metadata.get("description") {
        println!("   Description: {desc}");
    }
    
    // Show metadata
    if !event.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &event.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    // TODO: Show characters, locations, factions involved
    println!("   Participants: (to be implemented)");
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete event '{}'?", name);
        println!("   This will permanently delete the event and remove it from all references");
        println!("   Use --force to skip this confirmation");
        return Ok(())
    }
    
    println!("🗑️  Deleting event '{}'...", name);
    
    event.delete(force)?;
    
    println!("✅ Event '{}' deleted!", name);
    
    Ok(())
}

fn handle_timeline() -> Result<()> {
    let events = Event::list_chronological()?;
    
    if events.is_empty() {
        println!("📅 No events found in this world");
        println!("   Use 'multiverse event create <name> --display-name <name> --type <type>' to create one");
        return Ok(())
    }
    
    println!("⏰ Events Timeline (chronological order):");
    
    for event in events {
        let status_emoji = match event.status {
            EventStatus::Historical => "📜",
            EventStatus::Ongoing => "🔥",
            EventStatus::Planned => "⏳",
            EventStatus::Cancelled => "❌",
            EventStatus::Archived => "📦",
        };
        
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            event.date,
            event.display_name,
            event.event_type
        );
        
        if let Some(sort_key) = event.sort_key {
            println!("      [Sort: {}] {}", sort_key, event.name);
        }
    }
    
    Ok(())
}
