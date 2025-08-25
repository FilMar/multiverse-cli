use anyhow::Result;
use crate::database::get_connection;
use crate::world::config::WorldConfig;
use super::cli::{RelationCommands, CharacterRelationAction, CharacterEpisodeAction, LocationRelationAction, LocationCharacterAction};
use super::character_episode::{
    init_episode_character_tables, add_character_to_episode, remove_character_from_episode,
    get_episode_characters, get_character_episodes, update_character_role_in_episode
};

pub fn handle_relation_command(command: RelationCommands) -> Result<()> {
    let db_path = WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;
    
    // Initialize tables
    init_episode_character_tables(&conn)?;
    
    match command {
        RelationCommands::Character { action } => handle_character_relation(action, &conn),
        RelationCommands::Location { action } => handle_location_relation(action, &conn),
    }
}

fn handle_character_relation(action: CharacterRelationAction, conn: &rusqlite::Connection) -> Result<()> {
    match action {
        CharacterRelationAction::Episode { action } => {
            match action {
                CharacterEpisodeAction::Add { character_name, episode_id, story, role, importance } => {
                    add_character_to_episode(conn, episode_id, &character_name, &role, &importance)?;
                    println!("✅ Added {} to episode {} (story: {}) with role '{}' ({})", 
                            character_name, episode_id, story, role, importance);
                }
                CharacterEpisodeAction::Remove { character_name, episode_id, story } => {
                    remove_character_from_episode(conn, episode_id, &character_name)?;
                    println!("✅ Removed {} from episode {} (story: {})", character_name, episode_id, story);
                }
                CharacterEpisodeAction::List { character_name, episode, story } => {
                    if let Some(character) = &character_name {
                        // List episodes for character
                        let episodes = get_character_episodes(conn, &character)?;
                        if episodes.is_empty() {
                            println!("🔍 No episodes found for character '{}' in story '{}'", character, story);
                        } else {
                            println!("📺 Episodes for character '{}' in story '{}':", character, story);
                            for (episode_id, role, importance) in episodes {
                                println!("   Episode {}: {} ({})", episode_id, role, importance);
                            }
                        }
                    } else if let Some(ep_id) = episode {
                        // List characters for episode
                        let characters = get_episode_characters(conn, ep_id)?;
                        if characters.is_empty() {
                            println!("🔍 No characters found for episode {} in story '{}'", ep_id, story);
                        } else {
                            println!("👥 Characters in episode {} (story '{}'):", ep_id, story);
                            for (character_name, role, importance) in characters {
                                println!("   {}: {} ({})", character_name, role, importance);
                            }
                        }
                    } else {
                        println!("❌ Please specify either a character name or --episode <id>");
                    }
                }
                CharacterEpisodeAction::Update { character_name, episode_id, story, role, importance } => {
                    // Get current relation to fill in missing values
                    let current_chars = get_episode_characters(conn, episode_id)?;
                    let current = current_chars.iter()
                        .find(|(name, _, _)| name == &character_name);
                    
                    if let Some((_, current_role, current_importance)) = current {
                        let new_role = role.as_deref().unwrap_or(current_role);
                        let new_importance = importance.as_deref().unwrap_or(current_importance);
                        
                        update_character_role_in_episode(conn, episode_id, &character_name, new_role, new_importance)?;
                        println!("✅ Updated {} in episode {} (story: {}): role='{}', importance='{}'", 
                                character_name, episode_id, story, new_role, new_importance);
                    } else {
                        println!("❌ Character '{}' not found in episode {} (story: '{}')", character_name, episode_id, story);
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_location_relation(action: LocationRelationAction, _conn: &rusqlite::Connection) -> Result<()> {
    match action {
        LocationRelationAction::Character { action: _ } => {
            println!("⚠️  Location relations not yet implemented");
        }
    }
    Ok(())
}


