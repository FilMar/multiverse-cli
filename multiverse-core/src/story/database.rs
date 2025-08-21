use super::models::{Story, Episode, StoryStatus, EpisodeStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};
use std::path::Path;

/// Initialize story-related tables in the database
pub fn init_story_tables(conn: &Connection) -> Result<()> {
    // Create stories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stories (
            name TEXT PRIMARY KEY,
            narrator TEXT NOT NULL,
            story_type TEXT NOT NULL,
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
    
    conn.execute(
        "INSERT INTO stories (name, narrator, story_type, description, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            story.name,
            story.narrator,
            story.story_type,
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
        "SELECT name, narrator, story_type, description, created_at, status 
         FROM stories ORDER BY created_at DESC"
    )?;
    
    let story_iter = stmt.query_map([], |row| {
        let story_type_str: String = row.get(2)?;
        let status_str: String = row.get(5)?;
        let created_at_str: String = row.get(4)?;
        
        let story_type = story_type_str;
        
        let status = match status_str.as_str() {
            "Active" => StoryStatus::Active,
            "Paused" => StoryStatus::Paused,
            "Completed" => StoryStatus::Completed,
            "Archived" => StoryStatus::Archived,
            _ => StoryStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(2, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        Ok(Story {
            name: row.get(0)?,
            narrator: row.get(1)?,
            story_type,
            description: row.get(3)?,
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
        "SELECT name, narrator, story_type, description, created_at, status 
         FROM stories WHERE name = ?1"
    )?;
    
    let mut rows = stmt.query_map([name], |row| {
        let story_type_str: String = row.get(2)?;
        let status_str: String = row.get(5)?;
        let created_at_str: String = row.get(4)?;
        
        let story_type = story_type_str;
        
        let status = match status_str.as_str() {
            "Active" => StoryStatus::Active,
            "Paused" => StoryStatus::Paused,
            "Completed" => StoryStatus::Completed,
            "Archived" => StoryStatus::Archived,
            _ => StoryStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(2, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        Ok(Story {
            name: row.get(0)?,
            narrator: row.get(1)?,
            story_type,
            description: row.get(3)?,
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