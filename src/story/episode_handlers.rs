use super::episode_cli::EpisodeCommands;
use super::episode_models::{Episode, EpisodeStatus};
use anyhow::Result;

pub fn handle_episode_command(command: EpisodeCommands) -> Result<()> {
    match command {
        EpisodeCommands::Create { story, title } => {
            handle_create(story, title)
        }
        EpisodeCommands::List { story } => handle_list(story),
        EpisodeCommands::Info { story, number } => handle_info(story, number),
        EpisodeCommands::Delete { story, number, force } => handle_delete(story, number, force),
        EpisodeCommands::Update { story, number, title, status, word_count } => handle_update(story, number, title, status, word_count),
    }
}

fn handle_update(story_name: String, episode_number: i32, title: Option<String>, status: Option<String>, word_count: Option<i32>) -> Result<()> {
    println!("🔄 Updating episode {} in story '{}'", episode_number, story_name);

    let mut episode = Episode::get(&story_name, episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;

    episode.update(title, status, word_count)?;

    println!("✅ Episode {} updated!", episode.episode_number);
    handle_info(story_name, episode_number)?;

    Ok(())
}

fn handle_create(story_name: String, title: Option<String>) -> Result<()> {
    use crate::world::WorldConfig;
    use anyhow::Context;
    
    println!("📄 Creating episode in story '{}'...", story_name);
    
    let episode = Episode::new(story_name.clone(), title.clone())?;
    episode.create()?;
    
    let world_root = WorldConfig::get_world_root()
        .context("Not in a multiverse project directory")?;
    let story = super::story_models::Story::get(&story_name)?
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", story_name))?;
    let story_path = story.get_story_path(&world_root);
    let episode_filename = format!("{:03}.md", episode.episode_number);
    let episode_path = story_path.join(&episode_filename);
    
    println!("✅ Episode {} created!", episode.episode_number);
    println!("   Story: {}", story_name);
    println!("   File: {}", episode_path.display());
    
    if let Some(title) = &title {
        println!("   Title: {}", title);
    }
    
    Ok(())
}

fn handle_list(story_name: String) -> Result<()> {
    let episodes = Episode::list(&story_name)?;
    
    if episodes.is_empty() {
        println!("📄 No episodes found in story '{}'", story_name);
        println!("   Use 'multiverse episode create --story {} --title <title>' to create one", story_name);
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
        
        let title_str = episode.title
            .as_deref()
            .unwrap_or("(no title)");
        
        let word_count_str = episode.word_count
            .map(|wc| format!(" ({} words)", wc))
            .unwrap_or_default();
        
        println!("   {} {:03}. {}{}", 
            status_emoji, 
            episode.episode_number, 
            title_str,
            word_count_str
        );
    }
    
    Ok(())
}

fn handle_info(story_name: String, episode_number: i32) -> Result<()> {
    let episode = Episode::get(&story_name, episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;
    
    println!("📄 Episode {}: {}", episode.episode_number, story_name);
    
    if let Some(title) = &episode.title {
        println!("   Title: {}", title);
    }
    
    println!("   Status: {:?}", episode.status);
    println!("   Created: {}", episode.created_at.format("%Y-%m-%d %H:%M"));
    println!("   Updated: {}", episode.updated_at.format("%Y-%m-%d %H:%M"));
    
    if let Some(word_count) = episode.word_count {
        println!("   Word count: {}", word_count);
    }
    
    Ok(())
}

fn handle_delete(story_name: String, episode_number: i32, force: bool) -> Result<()> {
    let episode = Episode::get(&story_name, episode_number)?
        .ok_or_else(|| anyhow::anyhow!("Episode {} not found in story '{}'", episode_number, story_name))?;
    
    if !force {
        println!("⚠️  Are you sure you want to delete episode {} from story '{}'?", episode_number, story_name);
        println!("   This will permanently delete the episode file and database entry");
        println!("   Use --force to skip this confirmation");
        return Ok(());
    }
    
    println!("🗑️  Deleting episode {} from story '{}'...", episode_number, story_name);
    
    episode.delete(force)?;
    
    println!("✅ Episode {} deleted!", episode_number);
    
    Ok(())
}
