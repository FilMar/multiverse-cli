use serde::{Deserialize, Serialize};

/// Individual episode within a story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub story_name: String,
    pub episode_number: i32,
    pub title: Option<String>,
    pub status: EpisodeStatus,
    pub word_count: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpisodeStatus {
    Draft,
    InProgress,
    Review,
    Published,
}

impl Default for EpisodeStatus {
    fn default() -> Self {
        Self::Draft
    }
}

// Core interface
impl Episode {
    pub fn new(story_name: String, title: Option<String>) -> anyhow::Result<Self> {
        let episode_number = Self::get_next_episode_number(&story_name)?;
        
        Ok(Episode {
            id: 0, // Will be set by database
            story_name,
            episode_number,
            title,
            status: EpisodeStatus::Draft,
            word_count: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    /// Create episode on filesystem and database
    pub fn create(&self) -> anyhow::Result<()> {
        let world_root = Self::ensure_world_context()?;
        let story = Self::get_story_or_fail(&self.story_name)?;
        
        self.save_to_database()?;
        self.create_episode_file(&story, &world_root)?;
        
        Ok(())
    }

    /// List episodes for a story
    pub fn list(story_name: &str) -> anyhow::Result<Vec<Episode>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        Self::verify_story_exists(story_name)?;
        
        let conn = Self::get_database_connection()?;
        super::database::list_episodes(&conn, story_name)
            .context("Failed to list episodes")
    }

    /// Get a specific episode
    pub fn get(story_name: &str, episode_number: i32) -> anyhow::Result<Option<Episode>> {
        let _world_root = Self::ensure_world_context()?;
        Self::verify_story_exists(story_name)?;
        
        let conn = Self::get_database_connection()?;
        super::database::get_episode(&conn, story_name, episode_number)
    }

    /// Delete episode from database and filesystem
    pub fn delete(&self, force: bool) -> anyhow::Result<()> {
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }

        let world_root = Self::ensure_world_context()?;
        let story = Self::get_story_or_fail(&self.story_name)?;
        
        self.delete_from_database()?;
        self.delete_episode_file(&story, &world_root)?;
        
        Ok(())
    }
}

// Private utility functions  
impl Episode {
    fn get_database_connection() -> anyhow::Result<rusqlite::Connection> {
        use crate::world::WorldConfig;
        use rusqlite::Connection;
        use anyhow::Context;

        let db_path = WorldConfig::get_database_path()?;
        Connection::open(&db_path).context("Failed to open database")
    }

    fn ensure_world_context() -> anyhow::Result<std::path::PathBuf> {
        use crate::world::WorldConfig;
        use anyhow::Context;

        WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")
    }

    fn verify_story_exists(story_name: &str) -> anyhow::Result<()> {
        let conn = Self::get_database_connection()?;
        super::database::get_story(&conn, story_name)?
            .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", story_name))?;
        Ok(())
    }

    fn get_story_or_fail(story_name: &str) -> anyhow::Result<super::story_models::Story> {
        let conn = Self::get_database_connection()?;
        super::database::get_story(&conn, story_name)?
            .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", story_name))
    }

    fn get_next_episode_number(story_name: &str) -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        super::database::get_next_episode_number(&conn, story_name)
    }

    fn save_to_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::create_episode(&conn, self)
            .context("Failed to save episode to database")
    }

    fn delete_from_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::delete_episode(&conn, &self.story_name, self.episode_number)
            .context("Failed to delete episode from database")
    }

    fn create_episode_file(&self, story: &super::story_models::Story, world_root: &std::path::Path) -> anyhow::Result<()> {
        use std::fs;
        use anyhow::Context;

        let story_path = story.get_story_path(world_root);
        let episode_filename = format!("{:03}.md", self.episode_number);
        let episode_path = story_path.join(&episode_filename);
        
        fs::write(&episode_path, "")
            .with_context(|| format!("Failed to create episode file {}", episode_path.display()))
    }

    fn delete_episode_file(&self, story: &super::story_models::Story, world_root: &std::path::Path) -> anyhow::Result<()> {
        use std::fs;
        use anyhow::Context;

        let story_path = story.get_story_path(world_root);
        let episode_filename = format!("{:03}.md", self.episode_number);
        let episode_path = story_path.join(&episode_filename);
        
        if episode_path.exists() {
            fs::remove_file(&episode_path)
                .with_context(|| format!("Failed to delete episode file {}", episode_path.display()))?;
        }
        Ok(())
    }
}