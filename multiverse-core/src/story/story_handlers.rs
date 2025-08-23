use super::story_cli::StoryCommands;
use super::story_models::Story;
use anyhow::Result;

pub fn handle_story_command(command: StoryCommands) -> Result<()> {
    match command {
        StoryCommands::Create { name, title, story_type, set } => {
            handle_create(name, title, story_type, set)
        }
        StoryCommands::Types => handle_types(),
        StoryCommands::List => handle_list(),
        StoryCommands::Info { name } => handle_info(name),
        StoryCommands::Delete { name, force } => handle_delete(name, force),
    }
}

fn handle_create(name: String, title: String, story_type: String, set_args: Vec<(String, String)>) -> Result<()> {
    println!("📖 Creating story '{name}' ({})", title);
    
    // Use Story factory method with built-in validation
    let story = Story::create_new(name.clone(), title, story_type, set_args)?;
    story.create()?;
    
    // Display success information
    show_created_story(&story)?;
    
    Ok(())
}

fn show_created_story(story: &Story) -> Result<()> {
    use crate::world::WorldConfig;
    use anyhow::Context;
    
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    let story_path = story.get_story_path(&world_root);
    
    // Get story type config for display name
    let config = WorldConfig::load()?;
    let type_config = config.get_story_type(&story.story_type)?;
    
    println!("✅ Story '{}' created!", story.name);
    println!("   Location: {}", story_path.display());
    println!("   Title: {}", story.title);
    println!("   Type: {} ({})", story.story_type, type_config.display_name);
    
    // Show metadata
    if !story.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &story.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_types() -> Result<()> {
    use crate::world::WorldConfig;
    use anyhow::Context;

    let config = WorldConfig::load()
        .context("Failed to load world configuration")?;

    let story_types = config.list_story_types();
    if story_types.is_empty() {
        println!("📚 No story types configured");
        println!("   Add story types to .multiverse/config.toml");
        return Ok(());
    }

    println!("📚 Available story types:");

    for (type_name, type_config) in story_types {
        println!("\n   {} ({})", type_name, type_config.display_name);
        
        if !type_config.required_fields.is_empty() {
            println!("      Required: {}", type_config.required_fields.join(", "));
        }
        
        if !type_config.optional_fields.is_empty() {
            println!("      Optional: {}", type_config.optional_fields.join(", "));
        }
        
        if !type_config.defaults.is_empty() {
            println!("      Defaults:");
            for (key, value) in &type_config.defaults {
                println!("        {}: {}", key, value);
            }
        }
        
        println!("      Numbering: {}", type_config.numbering_format);
    }
    
    println!("\n📖 Example usage:");
    let first_type = config.world.global_config.story_types.keys().next().unwrap();
    println!("   multiverse story create my_story --title \"My Story\" --type {} --set <field>=<value>", first_type);

    Ok(())
}

fn handle_list() -> Result<()> {
    let stories = Story::list()?;
    
    if stories.is_empty() {
        println!("📖 No stories found in this world");
        println!("   Use 'multiverse story types' to see available story types");
        println!("   Use 'multiverse story create <name> --title <title> --type <type>' to create one");
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
        
        println!("   {} {} - \"{}\" ({})", 
            status_emoji, 
            story.name, 
            story.title,
            story.story_type
        );
        
        // Show key metadata fields
        if let Some(narrator) = story.metadata.get("narrator") {
            println!("      by {}", narrator.as_str().unwrap_or("Unknown"));
        }
        if let Some(author) = story.metadata.get("author") {
            println!("      by {}", author.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(desc) = &story.description {
            println!("      {desc}");
        }
    }
    
    Ok(())
}

fn handle_info(name: String) -> Result<()> {
    let story = Story::get(&name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", name))?;
    
    println!("📖 Story: {} - \"{}\"", story.name, story.title);
    println!("   Type: {}", story.story_type);
    println!("   Status: {:?}", story.status);
    println!("   Created: {}", story.created_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(desc) = &story.description {
        println!("   Description: {desc}");
    }
    
    // Show metadata
    if !story.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &story.metadata {
            println!("     {}: {}", key, value);
        }
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