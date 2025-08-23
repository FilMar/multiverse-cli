use super::story_models::{Story, StoryStatus};
use super::episode_models::{Episode, EpisodeStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize story-related tables in the database
pub fn init_story_tables(conn: &Connection) -> Result<()> {
    // Create stories table with flexible metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stories (
            name TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            story_type TEXT NOT NULL,
            metadata TEXT,
            description TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Active'
        )",
        [],
    ).context("Failed to create stories table")?;
    
    // Create episodes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS episodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            story_name TEXT NOT NULL,
            episode_number INTEGER NOT NULL,
            title TEXT,
            status TEXT NOT NULL DEFAULT 'Draft',
            word_count INTEGER,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (story_name) REFERENCES stories (name) ON DELETE CASCADE,
            UNIQUE(story_name, episode_number)
        )",
        [],
    ).context("Failed to create episodes table")?;
    
    Ok(())
}

/// Create a new story
pub fn create_story(conn: &Connection, story: &Story) -> Result<()> {
    let status_str = match story.status {
        StoryStatus::Active => "Active",
        StoryStatus::Paused => "Paused",
        StoryStatus::Completed => "Completed",
        StoryStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&story.metadata)
        .context("Failed to serialize story metadata")?;
    
    conn.execute(
        "INSERT INTO stories (name, title, story_type, metadata, description, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            story.name,
            story.title,
            story.story_type,
            metadata_json,
            story.description,
            story.created_at.to_rfc3339(),
            status_str
        ],
    ).context("Failed to insert story")?;
    
    Ok(())
}

/// List all stories
pub fn list_stories(conn: &Connection) -> Result<Vec<Story>> {
    let mut stmt = conn.prepare(
        "SELECT name, title, story_type, metadata, description, created_at, status 
         FROM stories ORDER BY created_at DESC"
    )?;
    
    let story_iter = stmt.query_map([], |row| {
        let story_type_str: String = row.get(2)?;
        let metadata_str: String = row.get(3)?;
        let status_str: String = row.get(6)?;
        let created_at_str: String = row.get(5)?;
        
        let status = match status_str.as_str() {
            "Active" => StoryStatus::Active,
            "Paused" => StoryStatus::Paused,
            "Completed" => StoryStatus::Completed,
            "Archived" => StoryStatus::Archived,
            _ => StoryStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(5, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Story {
            name: row.get(0)?,
            title: row.get(1)?,
            story_type: story_type_str,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    })?;
    
    let mut stories = Vec::new();
    for story in story_iter {
        stories.push(story?);
    }
    
    Ok(stories)
}

/// Get a specific story by name
pub fn get_story(conn: &Connection, name: &str) -> Result<Option<Story>> {
    let mut stmt = conn.prepare(
        "SELECT name, title, story_type, metadata, description, created_at, status 
         FROM stories WHERE name = ?1"
    )?;
    
    let mut rows = stmt.query_map([name], |row| {
        let story_type_str: String = row.get(2)?;
        let metadata_str: String = row.get(3)?;
        let status_str: String = row.get(6)?;
        let created_at_str: String = row.get(5)?;
        
        let status = match status_str.as_str() {
            "Active" => StoryStatus::Active,
            "Paused" => StoryStatus::Paused,
            "Completed" => StoryStatus::Completed,
            "Archived" => StoryStatus::Archived,
            _ => StoryStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(5, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Story {
            name: row.get(0)?,
            title: row.get(1)?,
            story_type: story_type_str,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    })?;
    
    match rows.next() {
        Some(story) => Ok(Some(story?)),
        None => Ok(None),
    }
}

/// Delete a story and all its episodes
pub fn delete_story(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM stories WHERE name = ?1", [name])
        .context("Failed to delete story")?;
    
    Ok(())
}

/// Create a new episode
pub fn create_episode(conn: &Connection, episode: &Episode) -> Result<()> {
    let status_str = match episode.status {
        EpisodeStatus::Draft => "Draft",
        EpisodeStatus::InProgress => "InProgress",
        EpisodeStatus::Review => "Review",
        EpisodeStatus::Published => "Published",
    };
    
    conn.execute(
        "INSERT INTO episodes (story_name, episode_number, title, status, word_count, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            episode.story_name,
            episode.episode_number,
            episode.title,
            status_str,
            episode.word_count,
            episode.created_at.to_rfc3339(),
            episode.updated_at.to_rfc3339()
        ],
    ).context("Failed to insert episode")?;
    
    Ok(())
}

/// List episodes for a story
pub fn list_episodes(conn: &Connection, story_name: &str) -> Result<Vec<Episode>> {
    let mut stmt = conn.prepare(
        "SELECT id, story_name, episode_number, title, status, word_count, created_at, updated_at
         FROM episodes WHERE story_name = ?1 ORDER BY episode_number ASC"
    )?;
    
    let episode_iter = stmt.query_map([story_name], |row| {
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(6)?;
        let updated_at_str: String = row.get(7)?;
        
        let status = match status_str.as_str() {
            "Draft" => EpisodeStatus::Draft,
            "InProgress" => EpisodeStatus::InProgress,
            "Review" => EpisodeStatus::Review,
            "Published" => EpisodeStatus::Published,
            _ => EpisodeStatus::Draft,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(7, "updated_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        Ok(Episode {
            id: row.get(0)?,
            story_name: row.get(1)?,
            episode_number: row.get(2)?,
            title: row.get(3)?,
            status,
            word_count: row.get(5)?,
            created_at,
            updated_at,
        })
    })?;
    
    let mut episodes = Vec::new();
    for episode in episode_iter {
        episodes.push(episode?);
    }
    
    Ok(episodes)
}

/// Get a specific episode
pub fn get_episode(conn: &Connection, story_name: &str, episode_number: i32) -> Result<Option<Episode>> {
    let mut stmt = conn.prepare(
        "SELECT id, story_name, episode_number, title, status, word_count, created_at, updated_at
         FROM episodes WHERE story_name = ?1 AND episode_number = ?2"
    )?;
    
    let mut rows = stmt.query_map([story_name, &episode_number.to_string()], |row| {
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(6)?;
        let updated_at_str: String = row.get(7)?;
        
        let status = match status_str.as_str() {
            "Draft" => EpisodeStatus::Draft,
            "InProgress" => EpisodeStatus::InProgress,
            "Review" => EpisodeStatus::Review,
            "Published" => EpisodeStatus::Published,
            _ => EpisodeStatus::Draft,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(7, "updated_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        Ok(Episode {
            id: row.get(0)?,
            story_name: row.get(1)?,
            episode_number: row.get(2)?,
            title: row.get(3)?,
            status,
            word_count: row.get(5)?,
            created_at,
            updated_at,
        })
    })?;
    
    match rows.next() {
        Some(episode) => Ok(Some(episode?)),
        None => Ok(None),
    }
}

/// Delete an episode
pub fn delete_episode(conn: &Connection, story_name: &str, episode_number: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM episodes WHERE story_name = ?1 AND episode_number = ?2",
        params![story_name, episode_number]
    ).context("Failed to delete episode")?;
    
    Ok(())
}

/// Get the next episode number for a story
pub fn get_next_episode_number(conn: &Connection, story_name: &str) -> Result<i32> {
    let mut stmt = conn.prepare(
        "SELECT COALESCE(MAX(episode_number), 0) + 1 FROM episodes WHERE story_name = ?1"
    )?;
    
    let next_number: i32 = stmt.query_row([story_name], |row| row.get(0))?;
    Ok(next_number)
}

/// Get total count of stories
pub fn count_stories(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM stories")?;
    let count: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

/// Get total count of episodes across all stories
pub fn count_episodes(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM episodes")?;
    let count: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

/// Get episodes count by status
pub fn count_episodes_by_status(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT status, COUNT(*) FROM episodes GROUP BY status ORDER BY status"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}