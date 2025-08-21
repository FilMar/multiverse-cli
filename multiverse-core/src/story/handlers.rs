use super::cli::StoryCommands;
use super::models::Story;
use super::database::{create_story, list_stories, get_story, delete_story};
use crate::world::WorldConfig;
use anyhow::{Result, Context, bail};
use rusqlite::Connection;
use std::fs;

pub fn handle_story_command(command: StoryCommands) -> Result<()> {
    match command {
        StoryCommands::Create { name, narrator, story_type } => {
            handle_create(name, narrator, story_type)
        }
        StoryCommands::List => handle_list(),
        StoryCommands::Info { name } => handle_info(name),
        StoryCommands::Delete { name, force } => handle_delete(name, force),
    }
}

fn handle_create(name: String, narrator: String, story_type: Option<String>) -> Result<()> {
    // Check if we're in a multiverse project
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory. Run 'multiverse world init <name>' to create one.")?;
    
    let db_path = WorldConfig::get_database_path()?;
    let conn = Connection::open(&db_path)
        .context("Failed to open database")?;
    
    // Check if story already exists
    if get_story(&conn, &name)?.is_some() {
        bail!("Story '{}' already exists", name);
    }
    
    println!("📖 Creating story '{}'...", name);
    
    // Create story object
    let story = Story::new(name.clone(), narrator, story_type);
    
    // Create story directory
    let story_path = story.get_story_path(&world_root);
    fs::create_dir_all(&story_path)
        .with_context(|| format!("Failed to create story directory {}", story_path.display()))?;
    
    // Save to database
    create_story(&conn, &story)
        .context("Failed to save story to database")?;
    
    println!("✅ Story '{}' created!", name);
    println!("   Location: {}", story_path.display());
    println!("   Narrator: {}", story.narrator);
    println!("   Type: {}", story.story_type);
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let _world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    let db_path = WorldConfig::get_database_path()?;
    let conn = Connection::open(&db_path)
        .context("Failed to open database")?;
    
    let stories = list_stories(&conn)
        .context("Failed to list stories")?;
    
    if stories.is_empty() {
        println!("📖 No stories found in this world");
        println!("   Use 'multiverse story create <name> --narrator <narrator>' to create one");
        return Ok(());
    }
    
    println!("📖 Stories in current world:");
    
    for story in stories {
        let status_emoji = match story.status {
            crate::story::models::StoryStatus::Active => "🟢",
            crate::story::models::StoryStatus::Paused => "🟡", 
            crate::story::models::StoryStatus::Completed => "✅",
            crate::story::models::StoryStatus::Archived => "📦",
        };
        
        let type_str = &story.story_type;
        
        println!("   {} {} ({}) - by {}", 
            status_emoji, 
            story.name, 
            type_str,
            story.narrator
        );
        
        if let Some(desc) = &story.description {
            println!("      {}", desc);
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let _world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    let db_path = WorldConfig::get_database_path()?;
    let conn = Connection::open(&db_path)
        .context("Failed to open database")?;
    
    let story = get_story(&conn, &name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", name))?;
    
    println!("📖 Story: {}", story.name);
    println!("   Narrator: {}", story.narrator);
    println!("   Type: {}", story.story_type);
    println!("   Status: {:?}", story.status);
    println!("   Created: {}", story.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = &story.description {
        println!("   Description: {}", desc);
    }
    
    // TODO: Show episode count and stats
    println!("   Episodes: (to be implemented)");
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    
    let db_path = WorldConfig::get_database_path()?;
    let conn = Connection::open(&db_path)
        .context("Failed to open database")?;
    
    // Check if story exists
    let story = get_story(&conn, &name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete story '{}'?", name);
        println!("   This will permanently delete the story directory and all episodes");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting story '{}'...", name);
    
    // Delete from database
    delete_story(&conn, &name)
        .context("Failed to delete story from database")?;
    
    // Delete story directory
    let story_path = story.get_story_path(&world_root);
    if story_path.exists() {
        fs::remove_dir_all(&story_path)
            .with_context(|| format!("Failed to delete story directory {}", story_path.display()))?;
    }
    
    println!("✅ Story '{}' deleted!", name);
    
    Ok(())
}