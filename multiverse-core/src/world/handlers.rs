use super::cli::WorldCommands;
use super::models::World;
use super::git::GitStatusPrinter;
use super::config::WorldConfig;
use anyhow::{Result, Context, bail};

pub fn handle_world_command(command: WorldCommands) -> Result<()> {
    match command {
        WorldCommands::Init { name, description, aesthetic, from_git, merge } => {
            handle_init(name, description, aesthetic, from_git, merge)
        }
        WorldCommands::Info => handle_info(),
        WorldCommands::Pull => handle_pull(),
        WorldCommands::Push => handle_push(),
        WorldCommands::Status => handle_status(),
        WorldCommands::Config { set, value } => handle_config(set, value),
        WorldCommands::Import { file, all } => handle_import(file, all),
    }
}

fn handle_init(name: String, description: Option<String>, aesthetic: Option<String>, from_git: Option<String>, merge: bool) -> Result<()> {
    if from_git.is_some() {
        println!("🌍 Cloning multiverse project from {}...", from_git.as_ref().unwrap());
    } else {
        println!("🌍 Initializing multiverse project '{name}'...");
    }
    
    let world = World::init(name.clone(), description.clone(), aesthetic, from_git.clone(), merge)?;
    
    if from_git.is_some() {
        println!("✅ Project cloned from Git!");
        println!("   Location: {}", world.path.display());
    } else {
        println!("✅ Multiverse project '{name}' initialized!");
        println!("   Location: {}", world.path.display());
        println!("   Core files: 00_ESSENTIAL.md, 01_HISTORY.md, README.md");
        println!("   Config: .multiverse/config.toml");
        println!("   Database: .multiverse/world.db");
        println!("   Git: Repository initialized");
        
        if let Some(desc) = &description {
            println!("   Description: {desc}");
        }
    }
    
    Ok(())
}

fn handle_info() -> Result<()> {
    let world = World::info()?;
    
    println!("🌍 Multiverse Project: {}", world.meta.name);
    println!("   Location: {}", world.path.display());
    
    if let Some(description) = &world.meta.description {
        println!("   Description: {description}");
    }
    
    if let Some(visual_identity) = &world.meta.visual_identity {
        println!("   Aesthetic: {} - {}", 
            visual_identity.estetica, 
            visual_identity.descrizione);
    }
    
    if let Some(global_config) = &world.meta.global_config {
        println!("   Numbering format: {}", global_config.formato_numerazione);
        println!("   Default template: {}", global_config.template_default);
    }
    
    println!("   Database: ✅ Valid");
    // TODO: Query database for stats
    println!("   Series: (to be implemented)");
    println!("   Episodes: (to be implemented)");
    
    Ok(())
}

fn handle_pull() -> Result<()> {
    println!("📥 Pulling updates...");
    
    let world = World::info()?;
    world.pull()?;
    
    println!("✅ Project updated!");
    
    Ok(())
}

fn handle_push() -> Result<()> {
    println!("📤 Pushing changes...");
    
    let world = World::info()?;
    world.push()?;
    
    println!("✅ Changes pushed!");
    
    Ok(())
}

fn handle_status() -> Result<()> {
    let world = World::info()?;
    
    println!("📊 Git status for project '{}':", world.meta.name);
    
    match world.status() {
        Ok(status) => {
            GitStatusPrinter::print_detailed(&status);
        }
        Err(e) => println!("   ❌ Error: {e}"),
    }
    
    Ok(())
}

fn handle_config(set: Option<String>, value: Option<String>) -> Result<()> {
    match (set, value) {
        (Some(key), Some(val)) => {
            let mut config = WorldConfig::load()
                .context("Not in a multiverse project directory")?;
            let world_root = WorldConfig::get_world_root()?;
            println!("trying to configure: {key} = {val}");
            match key.as_str() {
                "world.name" => config.world.name = val,
                "world.description" => config.world.description = Some(val),
                "world.visual_identity.estetica" => config.world.visual_identity.estetica = val,
                "world.visual_identity.descrizione" => config.world.visual_identity.descrizione = val,
                "world.global_config.formato_numerazione" => config.world.global_config.formato_numerazione = val,
                "world.global_config.template_default" => config.world.global_config.template_default = val,
                _ => bail!("Unknown configuration key: {}", key),
            }
            config.save(&world_root)?;
            println!("✅ Configuration updated!");
        }
        (None, None) => {
            // Show current configuration
            let config = WorldConfig::load()
                .context("Not in a multiverse project directory")?;
            
            println!("📋 Current configuration:");
            println!("   world.name = \"{}\"", config.world.name);
            if let Some(desc) = &config.world.description {
                println!("   world.description = \"{desc}\"");
            }
            println!("   world.visual_identity.estetica = \"{}\"", config.world.visual_identity.estetica);
            println!("   world.visual_identity.descrizione = \"{}\"", config.world.visual_identity.descrizione);
            println!("   world.global_config.formato_numerazione = \"{}\"", config.world.global_config.formato_numerazione);
            println!("   world.global_config.template_default = \"{}\"", config.world.global_config.template_default);
        }
        _ => bail!("Both --set and --value must be provided together"),
    }
    
    Ok(())
}

fn handle_import(file: Option<String>, all: bool) -> Result<()> {
    if all {
        println!("📥 Importing all SQL files from sql/...");
    } else if file.is_some() {
        println!("📥 Importing SQL scripts...");
    }
    
    World::import_sql(file, all)?;
    
    Ok(())
}
