use serde::{Deserialize, Serialize};
use std::path::Path;

/// Story metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub title: String,
    pub story_type: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
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
    /// Factory method with full validation - recommended way to create stories
    pub fn create_new(name: String, title: String, story_type: String, set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
        use crate::world::WorldConfig;
        use anyhow::Context;
        
        // Load config and get story type
        let config = WorldConfig::load()
            .context("Failed to load world configuration")?;
        
        let type_config = config.get_story_type(&story_type)?;
        
        // Use StoryTypeConfig to build and validate metadata
        let metadata = type_config.build_metadata(set_args)
            .with_context(|| format!("Failed to build metadata for story type '{}'", story_type))?;
        
        Ok(Self::new(name, title, story_type, metadata))
    }
    
    /// Direct constructor (for internal use or when metadata is already validated)
    pub fn new(name: String, title: String, story_type: String, metadata: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            name,
            title,
            story_type,
            metadata,
            description: None,
            created_at: chrono::Utc::now(),
            status: StoryStatus::Active,
        }
    }
    
    /// Get the story directory path within stories/ with snake_case name
    pub fn get_story_path(&self, world_root: &Path) -> std::path::PathBuf {
        let snake_case_name = self.title_to_snake_case();
        world_root.join("stories").join(&snake_case_name)
    }
    
    /// Convert story title to snake_case for directory name
    fn title_to_snake_case(&self) -> String {
        self.title
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c.is_whitespace() || c == '-' || c == '_' {
                    '_'
                } else {
                    // Skip special characters
                    '\0'
                }
            })
            .filter(|&c| c != '\0')
            .collect::<String>()
            .split('_')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("_")
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