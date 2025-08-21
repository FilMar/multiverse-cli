use serde::{Deserialize, Serialize};
use std::path::Path;

/// Story metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub narrator: String,
    pub story_type: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: StoryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

impl Default for StoryStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Core interface
impl Story {
    pub fn new(name: String, narrator: String, story_type: Option<String>) -> Self {
        let story_type = story_type.unwrap_or_else(|| "diary".to_string());
        
        Self {
            name,
            narrator,
            story_type,
            description: None,
            created_at: chrono::Utc::now(),
            status: StoryStatus::Active,
        }
    }
    
    /// Get the story directory path within stories/ with pattern nome_tipo/
    pub fn get_story_path(&self, world_root: &Path) -> std::path::PathBuf {
        let dir_name = format!("{}_{}", self.name, self.story_type);
        world_root.join("stories").join(&dir_name)
    }

    /// Create story on filesystem and database
    pub fn create(&self) -> anyhow::Result<()> {
        let world_root = Self::ensure_world_context()?;
        
        if Self::check_story_exists(&self.name)? {
            anyhow::bail!("Story '{}' already exists", self.name);
        }
        
        self.create_story_directory(&world_root)?;
        self.save_to_database()?;
        
        Ok(())
    }

    /// List all stories
    pub fn list() -> anyhow::Result<Vec<Story>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_stories(&conn)
            .context("Failed to list stories")
    }

    /// Get a story by name
    pub fn get(name: &str) -> anyhow::Result<Option<Story>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_story(&conn, name)
    }

    /// Delete story from database and filesystem
    pub fn delete(&self, force: bool) -> anyhow::Result<()> {
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }

        let world_root = Self::ensure_world_context()?;
        
        self.delete_from_database()?;
        self.delete_story_directory(&world_root)?;
        
        Ok(())
    }
}

// Private utility functions
impl Story {
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
            .context("Not in a multiverse project directory. Run 'multiverse world init <name>' to create one.")
    }

    fn check_story_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        Ok(super::database::get_story(&conn, name)?.is_some())
    }

    fn create_story_directory(&self, world_root: &Path) -> anyhow::Result<()> {
        use std::fs;
        use anyhow::Context;

        let story_path = self.get_story_path(world_root);
        fs::create_dir_all(&story_path)
            .with_context(|| format!("Failed to create story directory {}", story_path.display()))
    }

    fn save_to_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::create_story(&conn, self)
            .context("Failed to save story to database")
    }

    fn delete_from_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::delete_story(&conn, &self.name)
            .context("Failed to delete story from database")
    }

    fn delete_story_directory(&self, world_root: &Path) -> anyhow::Result<()> {
        use std::fs;
        use anyhow::Context;

        let story_path = self.get_story_path(world_root);
        if story_path.exists() {
            fs::remove_dir_all(&story_path)
                .with_context(|| format!("Failed to delete story directory {}", story_path.display()))?;
        }
        Ok(())
    }
}