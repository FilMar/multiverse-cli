use super::story_cli::StoryCommands;
use super::story_models::Story;
use anyhow::Result;

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
    use crate::world::WorldConfig;
    use anyhow::Context;

    println!("📖 Creating story '{name}'...");
    
    let story = Story::new(name.clone(), narrator, story_type);
    story.create()?;
    
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    let story_path = story.get_story_path(&world_root);
    
    println!("✅ Story '{name}' created!");
    println!("   Location: {}", story_path.display());
    println!("   Narrator: {}", story.narrator);
    println!("   Type: {}", story.story_type);
    
    Ok(())
}

fn handle_list() -> Result<()> {
    let stories = Story::list()?;
    
    if stories.is_empty() {
        println!("📖 No stories found in this world");
        println!("   Use 'multiverse story create <name> --narrator <narrator>' to create one");
        return Ok(());
    }
    
    println!("📖 Stories in current world:");
    
    for story in stories {
        let status_emoji = match story.status {
            crate::story::story_models::StoryStatus::Active => "🟢",
            crate::story::story_models::StoryStatus::Paused => "🟡", 
            crate::story::story_models::StoryStatus::Completed => "✅",
            crate::story::story_models::StoryStatus::Archived => "📦",
        };
        
        println!("   {} {} ({}) - by {}", 
            status_emoji, 
            story.name, 
            story.story_type,
            story.narrator
        );
        
        if let Some(desc) = &story.description {
            println!("      {desc}");
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let story = Story::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", name))?;
    
    println!("📖 Story: {}", story.name);
    println!("   Narrator: {}", story.narrator);
    println!("   Type: {}", story.story_type);
    println!("   Status: {:?}", story.status);
    println!("   Created: {}", story.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = &story.description {
        println!("   Description: {desc}");
    }
    
    // TODO: Show episode count and stats
    println!("   Episodes: (to be implemented)");
    
    Ok(())
}

fn handle_delete(name: String, force: bool) -> Result<()> {
    let story = Story::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete story '{name}'?");
        println!("   This will permanently delete the story directory and all episodes");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting story '{name}'...");
    
    story.delete(force)?;
    
    println!("✅ Story '{name}' deleted!");
    
    Ok(())
}