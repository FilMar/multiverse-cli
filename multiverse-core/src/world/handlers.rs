use super::cli::WorldCommands;
use super::models::{WorldMeta, VisualIdentity};
use super::database::{init_world_database, world_database_exists};
use super::git::{WorkspaceGitManager, WorldGitRepo};
use crate::config::Config;
use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use std::fs;

pub fn handle_world_command(command: WorldCommands) -> Result<()> {
    match command {
        WorldCommands::Create { name, description, aesthetic, from_git } => {
            handle_create(name, description, aesthetic, from_git)
        }
        WorldCommands::List => handle_list(),
        WorldCommands::Info { name } => handle_info(name),
        WorldCommands::Pull { name } => handle_pull(name),
        WorldCommands::Push { name } => handle_push(name),
        WorldCommands::Status { name } => handle_status(name),
        WorldCommands::Delete { name, force } => handle_delete(name, force),
    }
}

fn handle_create(name: String, description: Option<String>, aesthetic: Option<String>, from_git: Option<String>) -> Result<()> {
    let (config, _) = Config::load_or_default();
    let world_path = config.workspace.path.join(&name);
    
    // Check if world already exists
    if world_path.exists() {
        bail!("World '{}' already exists at {}", name, world_path.display());
    }
    
    if let Some(repo_url) = from_git {
        // Clone from Git repository
        println!("🌍 Cloning world '{}' from {}...", name, repo_url);
        
        WorldGitRepo::clone_from(&repo_url, &world_path)?;
        
        println!("✅ World '{}' cloned from Git!", name);
        println!("   Location: {}", world_path.display());
        
        // Verify database exists or initialize it
        let db_path = world_path.join(".world.db");
        if !world_database_exists(&db_path) {
            println!("   Initializing database...");
            init_world_database(&db_path)
                .context("Failed to initialize world database")?;
        }
        
    } else {
        // Create local world
        println!("🌍 Creating world '{}'...", name);
        
        // Create world directory
        fs::create_dir_all(&world_path)
            .with_context(|| format!("Failed to create world directory {}", world_path.display()))?;
        
        // Create subdirectories
        let subdirs = ["series"];
        for subdir in &subdirs {
            let subdir_path = world_path.join(subdir);
            fs::create_dir_all(&subdir_path)
                .with_context(|| format!("Failed to create directory {}", subdir_path.display()))?;
        }
        
        // Create world metadata
        let mut world_meta = WorldMeta::new(name.clone(), description.clone());
        
        // Apply aesthetic if provided
        if let Some(aesthetic) = aesthetic {
            if let Some(ref mut visual_identity) = world_meta.visual_identity {
                visual_identity.estetica = aesthetic;
                visual_identity.descrizione = format!("Mondo con estetica {}", visual_identity.estetica);
            }
        }
        
        // Save world metadata to .world.json
        world_meta.save(&world_path)
            .context("Failed to save world metadata")?;
        
        // Create fundamental files
        create_fundamental_files(&world_path, &name, description.as_deref())
            .context("Failed to create fundamental files")?;
        
        // Initialize world database
        let db_path = world_path.join(".world.db");
        init_world_database(&db_path)
            .context("Failed to initialize world database")?;
        
        // Initialize Git repository
        let world_repo = WorldGitRepo::new(&world_path)?;
        world_repo.init()?;
        
        println!("✅ World '{}' created!", name);
        println!("   Location: {}", world_path.display());
        println!("   Core files: 00_ESSENTIAL.md, 01_HISTORY.md, README.md");
        println!("   Metadata: {}/.world.json", world_path.display());
        println!("   Database: {}/.world.db", world_path.display());
        println!("   Git: Repository initialized");
        
        if let Some(desc) = &description {
            println!("   Description: {}", desc);
        }
    }
    
    Ok(())
}

fn handle_list() -> Result<()> {
    println!("🌍 Worlds in workspace:");
    
    // Get workspace path
    let (config, _) = Config::load_or_default();
    let workspace_path = &config.workspace.path;
    
    if !workspace_path.exists() {
        println!("   Workspace directory does not exist: {}", workspace_path.display());
        return Ok(());
    }
    
    // Scan for world directories
    let mut worlds = Vec::new();
    for entry in fs::read_dir(workspace_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let world_meta_path = path.join(".world.json");
            if world_meta_path.exists() {
                match WorldMeta::load(&path) {
                    Ok(meta) => worlds.push((path, meta)),
                    Err(_) => {
                        println!("   ⚠️  {}: invalid .world.json", path.file_name().unwrap().to_string_lossy());
                    }
                }
            }
        }
    }
    
    if worlds.is_empty() {
        println!("   (no worlds found - use 'multiverse world create <name>' to create one)");
    } else {
        for (world_path, meta) in worlds {
            let world_name = world_path.file_name().unwrap().to_string_lossy();
            let db_path = world_path.join(".world.db");
            let db_status = if world_database_exists(&db_path) { "✅" } else { "❌" };
            
            println!("   {} {} {}", db_status, world_name, 
                meta.description.as_deref().unwrap_or("(no description)"));
            
            if let Some(visual) = &meta.visual_identity {
                println!("      Aesthetic: {} - {}", visual.estetica, visual.descrizione);
            }
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    println!("🌍 World: {}", name);
    
    // Get workspace path and world path
    let (config, _) = Config::load_or_default();
    let world_path = config.workspace.path.join(&name);
    
    if !world_path.exists() {
        bail!("World '{}' does not exist", name);
    }
    
    // Load world metadata
    let world_meta = WorldMeta::load(&world_path)
        .context("Failed to load world metadata")?;
    
    println!("   Location: {}", world_path.display());
    println!("   Name: {}", world_meta.name);
    
    if let Some(description) = &world_meta.description {
        println!("   Description: {}", description);
    }
    
    if let Some(visual) = &world_meta.visual_identity {
        println!("   Aesthetic: {} - {}", visual.estetica, visual.descrizione);
    }
    
    if let Some(config) = &world_meta.global_config {
        println!("   Numbering format: {}", config.formato_numerazione);
        println!("   Default template: {}", config.template_default);
    }
    
    // Check database status
    let db_path = world_path.join(".world.db");
    if world_database_exists(&db_path) {
        println!("   Database: ✅ Valid");
        
        // TODO: Query database for stats
        println!("   Series: (to be implemented)");
        println!("   Episodes: (to be implemented)");
    } else {
        println!("   Database: ❌ Missing or invalid");
    }
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    // Get workspace path and world path
    let (config, _) = Config::load_or_default();
    let world_path = config.workspace.path.join(&name);
    
    if !world_path.exists() {
        bail!("World '{}' does not exist", name);
    }
    
    if !force {
        println!("⚠️  Are you sure you want to delete world '{}'?", name);
        println!("   This will permanently delete: {}", world_path.display());
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting world '{}'...", name);
    
    // Delete world directory and all contents
    fs::remove_dir_all(&world_path)
        .with_context(|| format!("Failed to delete world directory {}", world_path.display()))?;
    
    println!("✅ World '{}' deleted!", name);
    
    Ok(())
}

fn handle_pull(name: Option<String>) -> Result<()> {
    let workspace_manager = WorkspaceGitManager::new()?;
    
    match name {
        Some(world_name) => workspace_manager.pull_world(&world_name),
        None => workspace_manager.pull_all(),
    }
}

fn handle_push(name: String) -> Result<()> {
    let workspace_manager = WorkspaceGitManager::new()?;
    workspace_manager.push_world(&name)
}

fn handle_status(name: Option<String>) -> Result<()> {
    let workspace_manager = WorkspaceGitManager::new()?;
    
    match name {
        Some(world_name) => workspace_manager.status_world(&world_name),
        None => workspace_manager.status_all(),
    }
}

// Git status printing is now handled by GitStatusPrinter in git_utility.rs

fn create_fundamental_files(world_path: &Path, name: &str, description: Option<&str>) -> Result<()> {
    // Create 00_ESSENTIAL.md
    let essential_content = format!(r#"# {name} - Essentials

## Overview
{description}

## Core Concepts
<!-- The 80% of the world that matters most -->

### Geography
<!-- Key locations and their significance -->

### Cultures & Societies
<!-- Major civilizations, their values, and interactions -->

### Power Structures
<!-- Governments, organizations, hierarchies -->

### Magic/Technology System
<!-- How the extraordinary works in this world -->

### Conflicts & Tensions
<!-- Core conflicts that drive stories -->

## Key Characters
<!-- The most important figures everyone should know -->

## Important Rules
<!-- What makes this world unique and consistent -->

## Themes
<!-- The big ideas this world explores -->
"#, 
        name = name,
        description = description.unwrap_or("A rich narrative universe waiting to be explored.")
    );
    
    let essential_path = world_path.join("00_ESSENTIAL.md");
    fs::write(&essential_path, essential_content)
        .with_context(|| format!("Failed to create {}", essential_path.display()))?;
    
    // Create 01_HISTORY.md
    let history_content = format!(r#"# {name} - Historical Timeline

## Chronological Events

### Ancient Era
<!-- Foundational events, creation myths, early civilizations -->

### Classical Period
<!-- Major civilizations at their peak, defining conflicts -->

### Recent History
<!-- Events that directly impact current stories -->

### Current Era
<!-- The "present day" of your narratives -->

## Important Dates
<!-- Key dates that characters would know -->

## Historical Figures
<!-- People who shaped this world's history -->

## Consequences
<!-- How past events influence the present -->

---
*This timeline should include ALL significant events across all stories in chronological order*
"#, name = name);
    
    let history_path = world_path.join("01_HISTORY.md");
    fs::write(&history_path, history_content)
        .with_context(|| format!("Failed to create {}", history_path.display()))?;
    
    // Create README.md
    let readme_content = format!(r#"# {name}

{description}

## Getting Started

This world consists of several key files and directories:

### Core Documentation
- **00_ESSENTIAL.md** - The 80% of world information you need to know
- **01_HISTORY.md** - Complete chronological timeline of all events
- **series/** - All narrative series (diaries and extras)

### Lore Files
Individual lore files are named with recognizable patterns:
- `luogo_<region>_<location>_<details>.md` - Locations
- `personaggio_<name>_<details>.md` - Characters  
- `organizzazione_<name>_<details>.md` - Organizations
- `evento_<name>_<date>.md` - Specific events
- `cultura_<name>_<details>.md` - Cultural information

### Database
- `.world.db` - Operational data (SQLite database)
- `.world.json` - World metadata

## Contributing

When adding content to this world:

1. **Check 00_ESSENTIAL.md first** - Does your addition fit the established world?
2. **Update 01_HISTORY.md** - Add any historical events in chronological order
3. **Use consistent naming** - Follow the established patterns for lore files
4. **Cross-reference** - Link related concepts between files

## Series in this World

<!-- List of narrative series set in this world -->

---

*Managed with [Multiverse CLI](https://github.com/your-repo/multiverse-cli)*
"#, 
        name = name,
        description = description.unwrap_or("A rich narrative universe with complex histories and interconnected stories.")
    );
    
    let readme_path = world_path.join("README.md");
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to create {}", readme_path.display()))?;
    
    Ok(())
}
