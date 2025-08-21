use super::cli::WorldCommands;
use super::config::{WorldConfig, WorldMeta, VisualIdentity};
use super::database::init_world_database;
use super::git::WorldGitRepo;
use anyhow::{Result, Context, bail};
use std::path::Path;
use std::fs;

pub fn handle_world_command(command: WorldCommands) -> Result<()> {
    match command {
        WorldCommands::Init { name, description, aesthetic, from_git } => {
            handle_init(name, description, aesthetic, from_git)
        }
        WorldCommands::Info => handle_info(),
        WorldCommands::Pull => handle_pull(),
        WorldCommands::Push => handle_push(),
        WorldCommands::Status => handle_status(),
        WorldCommands::Config { set, value } => handle_config(set, value),
    }
}

fn handle_init(name: String, description: Option<String>, aesthetic: Option<String>, from_git: Option<String>) -> Result<()> {
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    
    // Check if already in a multiverse project
    if current_dir.join(".multiverse").exists() {
        bail!("Already in a multiverse project. Use 'multiverse info' to see details.");
    }
    
    if let Some(repo_url) = from_git {
        // Clone from Git repository
        println!("🌍 Cloning multiverse project from {}...", repo_url);
        
        WorldGitRepo::clone_from(&repo_url, &current_dir)?;
        
        println!("✅ Project cloned from Git!");
        println!("   Location: {}", current_dir.display());
        
        // Verify database exists or initialize it
        if let Ok(db_path) = WorldConfig::get_database_path() {
            if !db_path.exists() {
                println!("   Initializing database...");
                init_world_database(&db_path)
                    .context("Failed to initialize world database")?;
            }
        }
        
    } else {
        // Create local project
        println!("🌍 Initializing multiverse project '{}'...", name);
        
        // Create .multiverse directory
        let multiverse_dir = current_dir.join(".multiverse");
        fs::create_dir_all(&multiverse_dir)
            .context("Failed to create .multiverse directory")?;
        
        // Create stories directory
        let stories_dir = current_dir.join("stories");
        fs::create_dir_all(&stories_dir)
            .context("Failed to create stories directory")?;
        
        // Create configuration
        let mut config = WorldConfig::new(name.clone(), description.clone());
        
        // Apply aesthetic if provided
        if let Some(aesthetic) = aesthetic {
            config.world.visual_identity.estetica = aesthetic;
            config.world.visual_identity.descrizione = format!("Mondo con estetica {}", config.world.visual_identity.estetica);
        }
        
        // Save configuration to .multiverse/config.toml
        config.save(&current_dir)
            .context("Failed to save configuration")?;
        
        // Create fundamental files
        create_fundamental_files(&current_dir, &name, description.as_deref())
            .context("Failed to create fundamental files")?;
        
        // Initialize database
        let db_path = multiverse_dir.join("world.db");
        init_world_database(&db_path)
            .context("Failed to initialize world database")?;
        
        // Initialize Git repository
        let world_repo = WorldGitRepo::new(&current_dir)?;
        world_repo.init()?;
        
        println!("✅ Multiverse project '{}' initialized!", name);
        println!("   Location: {}", current_dir.display());
        println!("   Core files: 00_ESSENTIAL.md, 01_HISTORY.md, README.md");
        println!("   Config: .multiverse/config.toml");
        println!("   Database: .multiverse/world.db");
        println!("   Git: Repository initialized");
        
        if let Some(desc) = &description {
            println!("   Description: {}", desc);
        }
    }
    
    Ok(())
}

fn handle_info() -> Result<()> {
    let config = WorldConfig::load()
        .context("Not in a multiverse project directory. Run 'multiverse init <name>' to create one.")?;
    
    let world_root = WorldConfig::get_world_root()?;
    
    println!("🌍 Multiverse Project: {}", config.world.name);
    println!("   Location: {}", world_root.display());
    
    if let Some(description) = &config.world.description {
        println!("   Description: {}", description);
    }
    
    println!("   Aesthetic: {} - {}", 
        config.world.visual_identity.estetica, 
        config.world.visual_identity.descrizione);
    
    println!("   Numbering format: {}", config.world.global_config.formato_numerazione);
    println!("   Default template: {}", config.world.global_config.template_default);
    
    // Check database status
    if let Ok(db_path) = WorldConfig::get_database_path() {
        if db_path.exists() {
            println!("   Database: ✅ Valid");
            // TODO: Query database for stats
            println!("   Series: (to be implemented)");
            println!("   Episodes: (to be implemented)");
        } else {
            println!("   Database: ❌ Missing");
        }
    }
    
    Ok(())
}

fn handle_pull() -> Result<()> {
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    println!("📥 Pulling updates...");
    
    let world_repo = WorldGitRepo::new(&world_root)?;
    world_repo.pull()?;
    
    println!("✅ Project updated!");
    
    Ok(())
}

fn handle_push() -> Result<()> {
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    println!("📤 Pushing changes...");
    
    let world_repo = WorldGitRepo::new(&world_root)?;
    world_repo.push()?;
    
    println!("✅ Changes pushed!");
    
    Ok(())
}

fn handle_status() -> Result<()> {
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    let config = WorldConfig::load()?;
    
    println!("📊 Git status for project '{}':", config.world.name);
    
    let world_repo = WorldGitRepo::new(&world_root)?;
    match world_repo.status() {
        Ok(status) => {
            use super::git::GitStatusPrinter;
            GitStatusPrinter::print_detailed(&status);
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    Ok(())
}

fn handle_config(set: Option<String>, value: Option<String>) -> Result<()> {
    match (set, value) {
        (Some(key), Some(val)) => {
            let mut config = WorldConfig::load()
                .context("Not in a multiverse project directory")?;
            let world_root = WorldConfig::get_world_root()?;
            println!("trying to configure: {} = {}", key, val);
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
                println!("   world.description = \"{}\"", desc);
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

fn create_fundamental_files(world_path: &Path, name: &str, description: Option<&str>) -> Result<()> {
    // Create empty fundamental files
    let files = ["00_ESSENTIAL.md", "01_HISTORY.md", "README.md"];
    
    for file_name in &files {
        let file_path = world_path.join(file_name);
        fs::write(&file_path, "")
            .with_context(|| format!("Failed to create {}", file_path.display()))?;
    }
    
    Ok(())
}
