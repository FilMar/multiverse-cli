use super::cli::EpisodeCommands;
use super::models::{Episode, EpisodeStatus};
use crate::relations::{process_relations, EntityType, separate_relation_fields};
use anyhow::Result;

pub fn handle_episode_command(command: EpisodeCommands) -> Result<()> {
    match command {
        EpisodeCommands::Create { story, set } => {
            handle_create(story, set)
        }
        EpisodeCommands::List { story } => handle_list(story),
        EpisodeCommands::Info { story, number } => handle_info(story, number),
        EpisodeCommands::Delete { story, number, force } => handle_delete(story, number, force),
        EpisodeCommands::Update { story, number, set } => handle_update(story, number, set),
    }
}

fn handle_update(story_name: String, episode_number: i32, set_args: Vec<(String, String)>) -> Result<()> {
    println!("🔄 Updating episode {} in story '{}'", episode_number, story_name);

    let mut episode = Episode::get(&story_name, &episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;

    let episode_id = format!("{}:{}", story_name, episode_number);
    let regular_fields = process_relations(EntityType::Episode(episode_id), set_args)?;
    episode.update(regular_fields)?;

    println!("✅ Episode {} updated!", episode.number);
    handle_info(story_name, episode_number)?;

    Ok(())
}

fn handle_create(story_name: String, set: Vec<(String, String)>) -> Result<()> {
    use crate::world::WorldConfig;
    use anyhow::Context;
    
    println!("📄 Creating episode in story '{}'...", story_name);
    
    // Separate relation fields from regular fields  
    let relation_keys = ["character"];
    let (relation_fields, regular_fields) = separate_relation_fields(set.clone(), &relation_keys);
    
    let mut episode = Episode::new_with_next_number(story_name.clone())?;
    
    // Apply regular set arguments to episode before creating
    if !regular_fields.is_empty() {
        episode.process_set_args(regular_fields.clone())?;
    }
    
    episode.create_with_file()?;
    
    // THEN process relations after episode exists in database
    if !relation_fields.is_empty() {
        let episode_id = format!("{}:{}", episode.story, episode.number);
        process_relations(EntityType::Episode(episode_id), relation_fields)?;
    }
    
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    let story = crate::story::Story::get(&story_name.to_string())?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", story_name))?;
    let story_path = story.get_story_path(&world_root);
    let episode_filename = format!("{:03}.md", episode.number);
    let episode_path = story_path.join(&episode_filename);
    
    println!("✅ Episode {} created!", episode.number);
    println!("   Story: {}", story_name);
    println!("   File: {}", episode_path.display());
    
    // Extract title from regular fields if provided
    if let Some((_, title)) = regular_fields.iter().find(|(key, _)| key == "title") {
        println!("   Title: {}", title);
    }
    
    Ok(())
}

fn handle_list(story_name: String) -> Result<()> {
    let episodes = Episode::list_for_story(&story_name)?;
    
    if episodes.is_empty() {
        println!("📄 No episodes found in story '{}'", story_name);
        println!("   Use 'multiverse episode create --story {} --set title=<title>' to create one", story_name);
        return Ok(());
    }
    
    println!("📄 Episodes in story '{}':", story_name);
    
    for episode in episodes {
        let status_emoji = match episode.status {
            EpisodeStatus::Draft => "📝",
            EpisodeStatus::InProgress => "⏳",
            EpisodeStatus::Review => "👀",
            EpisodeStatus::Published => "✅",
        };
        
        let title_str = if !episode.title.is_empty() {
            episode.title.as_str()
        } else {
            "(no title)"
        };
        
        let word_count_str = if episode.word_count > 0 {
            format!(" ({} words)", episode.word_count)
        } else {
            String::new()
        };
        
        println!("   {} {:03}. {}{}", 
            status_emoji, 
            episode.number, 
            title_str,
            word_count_str
        );
    }
    
    Ok(())
}

fn handle_info(story_name: String, episode_number: i32) -> Result<()> {
    let episode = Episode::get(&story_name, &episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;
    
    println!("📄 Episode {}: {}", episode.number, story_name);
    
    if !episode.title.is_empty() {
        println!("   Title: {}", episode.title);
    }
    
    println!("   Status: {:?}", episode.status);
    println!("   Word Count: {}", episode.word_count);
    println!("   Created: {}", episode.created_at.format("%Y-%m-%d %H:%M"));
    
    // Show metadata
    if !episode.metadata.is_empty() {
        println!("   Metadata:");
        for (key, value) in &episode.metadata {
            println!("     {}: {}", key, value);
        }
    }
    
    Ok(())
}

fn handle_delete(story_name: String, episode_number: i32, force: bool) -> Result<()> {
    let episode = Episode::get(&story_name, &episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete episode {} from story '{}'?", episode_number, story_name);
        println!("   This will permanently delete the episode file and database entry");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting episode {} from story '{}'...", episode_number, story_name);
    
    episode.delete_with_file(force)?;
    
    println!("✅ Episode {} deleted!", episode_number);
    
    Ok(())
}
