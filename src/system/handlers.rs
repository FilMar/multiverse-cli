use super::cli::SystemCommands;
use super::models::{System, SystemStatus};
use anyhow::Result;

pub fn handle_system_command(command: SystemCommands) -> Result<()> {
    match command {
        SystemCommands::Create {
            name,
            display_name,
            system_type,
            set,
        } => handle_create(name, display_name, system_type, set),
        SystemCommands::List => handle_list(),
        SystemCommands::Info { name } => handle_info(name),
        SystemCommands::Delete { name, force } => handle_delete(name, force),
        SystemCommands::Update { name, display_name, system_type, set } => handle_update(name, display_name, system_type, set),
    }
}

fn handle_update(name: String, display_name: Option<String>, system_type: Option<String>, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating system '{name}'");

    let mut system = System::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("System '{}' not found", name))?;

    system.update(display_name, system_type, set_args)?;

    println!("✅ System '{}' updated!", name);
    show_created_system(&system)?;

    Ok(())
}

fn handle_create(
    name: String,
    display_name: String,
    system_type: String,
    set_args: Vec<(String, String)>,
) -> Result<()> {
    println!("⚙️  Creating system '{name}' ({display_name})");
    let system = System::create_new(name.clone(), display_name, system_type, set_args)?;
    system.create()?;
    show_created_system(&system)?;
    Ok(())
}

fn show_created_system(system: &System) -> Result<()> {
    println!("✅ System '{}' created!", system.name);
    println!("   Display name: {}", system.display_name);
    println!("   Type: {}", system.system_type);
    println!("   Status: {:?}", system.status);

    if let Some(desc) = &system.description {
        println!("   Description: {desc}");
    }

    // Show metadata
    if !system.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &system.metadata {
            println!("     {}: {}", key, value);
        }
    }

    Ok(())
}

fn handle_list() -> Result<()> {
    let systems = System::list()?;

    if systems.is_empty() {
        println!("⚙️  No systems found in this world");
        println!("   Use 'multiverse system create <name> --display-name <name> --type <type>' to create one");
        return Ok(());
    }

    println!("⚙️  Systems in current world:");

    for system in systems {
        let status_emoji = match system.status {
            SystemStatus::Active => "🟢",
            SystemStatus::Inactive => "🟡",
            SystemStatus::Deprecated => "🔴",
            SystemStatus::Archived => "📦",
        };

        println!(
            "   {} {} - \"{}\" ({})",
            status_emoji, system.name, system.display_name, system.system_type
        );

        // Show key metadata fields
        if let Some(complexity) = system.metadata.get("complexity") {
            println!(
                "      Complexity: {}",
                complexity.as_str().unwrap_or("Unknown")
            );
        }
        if let Some(origin) = system.metadata.get("origin") {
            println!("      Origin: {}", origin.as_str().unwrap_or("Unknown"));
        }

        if let Some(desc) = &system.description {
            println!("      {desc}");
        }
    }

    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let system =
        System::get(&name)?.ok_or_else(|| anyhow::anyhow!("System '{}' not found", name))?;

    println!("⚙️  System: {} - \"{}\"", system.name, system.display_name);
    println!("   Type: {}", system.system_type);
    println!("   Status: {:?}", system.status);
    println!("   Created: {}", system.created_at.format("%Y-%m-%d %H:%M"));

    if let Some(desc) = &system.description {
        println!("   Description: {desc}");
    }

    // Show metadata
    if !system.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &system.metadata {
            println!("     {}: {}", key, value);
        }
    }

    // TODO: Show episodes/characters where system is used
    println!("   Usage: (to be implemented)");

    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let system =
        System::get(&name)?.ok_or_else(|| anyhow::anyhow!("System '{}' not found", name))?;

    if !force {
        println!("⚠️  Are you sure you want to delete system '{name}'?");
        println!(
            "   This will permanently delete the system and remove it from all usage references"
        );
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }

    println!("🗑️  Deleting system '{name}'...");

    System::delete(&name)?;

    println!("✅ System '{name}' deleted!");

    Ok(())
}

