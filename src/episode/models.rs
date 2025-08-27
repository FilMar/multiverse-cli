//! Episode entity using the new modular macro system
//! Episodes are uniquely identified by story + number combination

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

// Generate complete Episode entity with composite key
define_complete_entity!(
    Episode,
    EpisodeStatus,
    EpisodeDb,
    table: "episodes",
    key_fields: { 
        story: String,
        number: i32 
    },
    fields: { 
        title: String,
        word_count: i32
    },
    status_variants: [ Draft, InProgress, Review, Published ],
    create_sql: "CREATE TABLE IF NOT EXISTS episodes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        story TEXT NOT NULL,
        number INTEGER NOT NULL,
        title TEXT NOT NULL DEFAULT '',
        word_count INTEGER NOT NULL DEFAULT 0,
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Draft',
        UNIQUE(story, number)
    )"
);

// Custom implementations for Episode
impl Episode {
    /// Get the display title (fallback to episode number)
    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            format!("{:03}. {}", self.number, self.title)
        } else {
            format!("{:03}. Episode {}", self.number, self.number)
        }
    }

    /// Get episode file path within story directory
    pub fn get_episode_path(&self, story_path: &std::path::Path) -> std::path::PathBuf {
        let filename = format!("{:03}.md", self.number);
        story_path.join(filename)
    }

    /// Create a new episode with the next sequential number for the story
    pub fn new_with_next_number(story_name: String) -> anyhow::Result<Self> {
        let next_number = Self::get_next_episode_number(&story_name)?;
        let episode = Episode {
            id: 0, // Will be set by database
            story: story_name,
            number: next_number,
            title: String::new(),
            word_count: 0,
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            status: EpisodeStatus::Draft,
        };
        Ok(episode)
    }

    /// Get the next episode number for a story
    pub fn get_next_episode_number(story_name: &str) -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT COALESCE(MAX(number), 0) + 1 FROM episodes WHERE story = ?1"
        )?;
        let next_number: i32 = stmt.query_row([story_name], |row| row.get(0))?;
        Ok(next_number)
    }

    /// Create episode with file in the story directory
    pub fn create_with_file(&mut self) -> anyhow::Result<()> {
        use crate::world::WorldConfig;
        use anyhow::Context;
        
        // Verify the story exists
        let story = crate::story::Story::get(&self.story)?
            .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", self.story))?;

        // Create the episode in database first
        self.create()?;
        
        // Create episode file
        let world_root = WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")?;
        let story_path = story.get_story_path(&world_root);
        let episode_path = self.get_episode_path(&story_path);
        
        // Create episode content
        let content = self.generate_episode_content(&story)?;
        std::fs::write(&episode_path, content)
            .with_context(|| format!("Failed to write episode file: {}", episode_path.display()))?;
        
        println!("📄 Created episode file: {}", episode_path.display());
        Ok(())
    }

    /// Generate initial episode content
    fn generate_episode_content(&self, story: &crate::story::Story) -> anyhow::Result<String> {
        let title = if !self.title.is_empty() {
            &self.title
        } else {
            "Untitled Episode"
        };

        let content = format!(
            "# {}\n\n**Story:** {}\n**Episode:** {}\n**Status:** {:?}\n**Created:** {}\n\n---\n\n[Episode content goes here]\n\n---\n\n**Word Count:** {}\n",
            title,
            story.display_name(),
            self.number,
            self.status,
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.word_count
        );
        
        Ok(content)
    }

    /// Delete episode with file from filesystem
    pub fn delete_with_file(&self, force: bool) -> anyhow::Result<()> {
        use crate::world::WorldConfig;
        use anyhow::Context;
        
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }
        
        // Get the story and episode path
        let story = crate::story::Story::get(&self.story)?
            .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", self.story))?;
        
        let world_root = WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")?;
        let story_path = story.get_story_path(&world_root);
        let episode_path = self.get_episode_path(&story_path);
        
        // Delete from database first
        self.delete(force)?;
        
        // Delete episode file if it exists
        if episode_path.exists() {
            std::fs::remove_file(&episode_path)
                .with_context(|| format!("Failed to delete episode file: {}", episode_path.display()))?;
            println!("🗑️ Deleted episode file: {}", episode_path.display());
        }
        
        Ok(())
    }

    /// Get episodes for a specific story
    pub fn list_for_story(story_name: &str) -> anyhow::Result<Vec<Episode>> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, story, number, title, word_count, metadata, created_at, status 
             FROM episodes WHERE story = ?1 ORDER BY number ASC"
        )?;
        
        let episode_iter = stmt.query_map([story_name], |row| {
            let metadata_json: String = row.get("metadata")?;
            let metadata: std::collections::HashMap<String, serde_json::Value> = 
                serde_json::from_str(&metadata_json).unwrap_or_default();
            
            let created_at_str: String = row.get("created_at")?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(
                    row.as_ref().column_index("created_at").unwrap(),
                    "created_at".to_string(),
                    rusqlite::types::Type::Text,
                ))?
                .with_timezone(&chrono::Utc);
            
            let status_str: String = row.get("status")?;
            let status: EpisodeStatus = serde_json::from_str(&format!("\"{}\"", status_str))
                .map_err(|_| rusqlite::Error::InvalidColumnType(
                    row.as_ref().column_index("status").unwrap(),
                    "status".to_string(),
                    rusqlite::types::Type::Text,
                ))?;
            
            Ok(Episode {
                id: row.get("id")?,
                story: row.get("story")?,
                number: row.get("number")?,
                title: row.get("title")?,
                word_count: row.get("word_count")?,
                metadata,
                created_at,
                status,
            })
        })?;
        
        let mut episodes = Vec::new();
        for episode in episode_iter {
            episodes.push(episode?);
        }
        
        Ok(episodes)
    }

    /// Count episodes for a story
    pub fn count_for_story(story_name: &str) -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM episodes WHERE story = ?1")?;
        let count: i32 = stmt.query_row([story_name], |row| row.get(0))?;
        Ok(count)
    }

    /// Get episodes by status for a story
    pub fn list_by_status(story_name: &str, status: EpisodeStatus) -> anyhow::Result<Vec<Episode>> {
        let status_str = format!("{:?}", status);
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, story, number, title, word_count, metadata, created_at, status 
             FROM episodes WHERE story = ?1 AND status = ?2 ORDER BY number ASC"
        )?;
        
        let episode_iter = stmt.query_map([story_name, &status_str], |row| {
            let metadata_json: String = row.get("metadata")?;
            let metadata: std::collections::HashMap<String, serde_json::Value> = 
                serde_json::from_str(&metadata_json).unwrap_or_default();
            
            let created_at_str: String = row.get("created_at")?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(
                    row.as_ref().column_index("created_at").unwrap(),
                    "created_at".to_string(),
                    rusqlite::types::Type::Text,
                ))?
                .with_timezone(&chrono::Utc);
            
            let episode_status: EpisodeStatus = serde_json::from_str(&format!("\"{}\"", status_str))
                .map_err(|_| rusqlite::Error::InvalidColumnType(
                    row.as_ref().column_index("status").unwrap(),
                    "status".to_string(),
                    rusqlite::types::Type::Text,
                ))?;
            
            Ok(Episode {
                id: row.get("id")?,
                story: row.get("story")?,
                number: row.get("number")?,
                title: row.get("title")?,
                word_count: row.get("word_count")?,
                metadata,
                created_at,
                status: episode_status,
            })
        })?;
        
        let mut episodes = Vec::new();
        for episode in episode_iter {
            episodes.push(episode?);
        }
        
        Ok(episodes)
    }

    /// Calculate total word count for a story
    pub fn total_word_count_for_story(story_name: &str) -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT COALESCE(SUM(word_count), 0) FROM episodes WHERE story = ?1")?;
        let total: i32 = stmt.query_row([story_name], |row| row.get(0))?;
        Ok(total)
    }
}