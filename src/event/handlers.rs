use super::cli::EventCommands;
use super::models::Event;
use anyhow::Result;

pub fn handle_event_command(command: EventCommands) -> Result<()> {
    match command {
        EventCommands::Create { name, set } => {
            handle_create(name, set)
        }
        EventCommands::List => handle_list(),
        EventCommands::Timeline => handle_timeline(),
        EventCommands::Info { name } => handle_info(name),
        EventCommands::Delete { name, force } => handle_delete(name, force),
        EventCommands::Update { name, set } => handle_update(name, set),
    }
}

fn handle_update(name: String, mut set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating event '{name}'");

    let mut event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;

    // Normalize field names: title -> display_name
    for (key, _) in &mut set_args {
        if key == "title" {
            *key = "display_name".to_string();
        }
    }

    // Check if date is being updated
    let date_update = set_args.iter().find(|(k, _)| k == "date").cloned();
    
    event.update(set_args)?;
    
    // If date was updated, recalculate sort_key
    if let Some((_, date_str)) = date_update {
        event.update_date(date_str)?;
        // The update_date method already updates the event, we just need to save it
        event.update(vec![])?; // This saves the current state including date_text and sort_key
    }

    println!("✅ Event '{}' updated!", name);
    show_created_event(&event)?;

    Ok(())
}

fn handle_create(name: String, mut set_args: Vec<(String, String)>) -> Result<()> {
    let title = set_args.iter()
        .find(|(k, _)| k == "title" || k == "display_name")
        .map(|(_, v)| v.as_str())
        .unwrap_or(&name);
    
    println!("📅 Creating event '{name}' ({})", title);

    // Normalize field names: title -> display_name
    for (key, _) in &mut set_args {
        if key == "title" {
            *key = "display_name".to_string();
        }
    }

    // Use Event factory method with built-in validation
    let mut event = Event::create_new(name.clone(), set_args)?;
    
    // If date was provided, parse it and update sort_key BEFORE creating
    if let Some(date_value) = event.metadata.get("date") {
        if let Some(date_str) = date_value.as_str() {
            event.update_date(date_str.to_string())?;
        }
    }
    
    event.create()?;
    
    // Display success information
    show_created_event(&event)?;
    
    Ok(())
}

fn show_created_event(event: &Event) -> Result<()> {
    println!("✅ Event '{}' created!", event.name);
    println!("   Title: {}", event.display_name);
    println!("   Status: {:?}", event.status);
    
    if !event.date_text.is_empty() {
        println!("   Date: {}", event.date_text);
        println!("   Sort key: {}", event.sort_key);
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
        println!("   Use 'multiverse event create <name> --set title=\"<title>\"' to create one");
        return Ok(());
    }
    
    println!("📅 Events in current world:");
    
    for event in events {
        let status_emoji = match event.status {
            crate::event::models::EventStatus::Active => "🟢",
            crate::event::models::EventStatus::Inactive => "⚫",
            crate::event::models::EventStatus::Completed => "✅", 
            crate::event::models::EventStatus::Cancelled => "❌",
            crate::event::models::EventStatus::Pending => "⏳",
        };
        
        println!("   {} {} - \"{}\"", 
            status_emoji, 
            event.name, 
            event.display_name
        );
        
        if !event.date_text.is_empty() {
            println!("      Date: {}", event.date_text);
        }
        
        // Show key metadata fields
        if let Some(event_type) = event.metadata.get("type") {
            println!("      Type: {}", event_type.as_str().unwrap_or("Unknown"));
        }
        if let Some(description) = event.metadata.get("description") {
            println!("      {}", description.as_str().unwrap_or(""));
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;
    
    println!("📅 Event: {} - \"{}\"", event.name, event.display_name);
    println!("   Status: {:?}", event.status);
    println!("   Created: {}", event.created_at.format("%Y-%m-%d %H:%M"));
    
    if !event.date_text.is_empty() {
        println!("   Date: {}", event.date_text);
        println!("   Sort key: {}", event.sort_key);
    }
    
    if let Some(desc) = event.metadata.get("description") {
        println!("   Description: {}", desc.as_str().unwrap_or(""));
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

fn handle_delete(name: String, force: bool) -> Result<()> {
    let event = Event::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Event '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete event '{name}'?");
        println!("   This will permanently delete the event from database");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting event '{name}'...");
    
    event.delete(force)?;
    
    println!("✅ Event '{name}' deleted!");
    
    Ok(())
}

fn handle_timeline() -> Result<()> {
    let events = Event::list_chronological()?;
    
    if events.is_empty() {
        println!("📅 No events found in this world");
        println!("   Use 'multiverse event create <name> --set title=\"<title>\"' to create one");
        return Ok(());
    }
    
    println!("⏰ Events Timeline (chronological order):");
    
    for event in events {
        let status_emoji = match event.status {
            crate::event::models::EventStatus::Active => "🟢",
            crate::event::models::EventStatus::Inactive => "⚫",
            crate::event::models::EventStatus::Completed => "✅", 
            crate::event::models::EventStatus::Cancelled => "❌",
            crate::event::models::EventStatus::Pending => "⏳",
        };
        
        let date_display = if !event.date_text.is_empty() {
            &event.date_text
        } else {
            "Unknown date"
        };
        
        println!("   {} {} - \"{}\"", 
            status_emoji, 
            date_display,
            event.display_name
        );
        
        println!("      [Sort: {}] {}", event.sort_key, event.name);
        
        if let Some(event_type) = event.metadata.get("type") {
            println!("      Type: {}", event_type.as_str().unwrap_or("Unknown"));
        }
    }
    
    Ok(())
}
