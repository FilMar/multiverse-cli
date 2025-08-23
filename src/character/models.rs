use serde::{Deserialize, Serialize};

/// Character metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub display_name: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: CharacterStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterStatus {
    Active,
    Inactive,
    Deceased,
    Archived,
}

impl Default for CharacterStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Core interface
impl Character {
    /// Factory method with full validation
    pub fn create_new(name: String, display_name: String, set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
        let mut metadata = std::collections::HashMap::new();
        
        // Process set_args into metadata
        for (key, value) in set_args {
            metadata.insert(key, serde_json::Value::String(value));
        }
        
        Ok(Self::new(name, display_name, metadata))
    }
    
    /// Direct constructor
    pub fn new(name: String, display_name: String, metadata: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            name,
            display_name,
            metadata,
            description: None,
            created_at: chrono::Utc::now(),
            status: CharacterStatus::Active,
        }
    }

    /// Create character in database
    pub fn create(&self) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        
        if Self::check_character_exists(&self.name)? {
            anyhow::bail!("Character '{}' already exists", self.name);
        }
        
        self.save_to_database()?;
        
        Ok(())
    }

    /// List all characters
    pub fn list() -> anyhow::Result<Vec<Character>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_characters(&conn)
            .context("Failed to list characters")
    }

    /// Get a character by name
    pub fn get(name: &str) -> anyhow::Result<Option<Character>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_character(&conn, name)
    }

    /// Delete character from database
    pub fn delete(&self, force: bool) -> anyhow::Result<()> {
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }

        let _world_root = Self::ensure_world_context()?;
        
        self.delete_from_database()?;
        
        Ok(())
    }
}

// Private utility functions
impl Character {
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

    fn check_character_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        Ok(super::database::get_character(&conn, name)?.is_some())
    }

    fn save_to_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::create_character(&conn, self)
            .context("Failed to save character to database")
    }

    fn delete_from_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::delete_character(&conn, &self.name)
            .context("Failed to delete character from database")
    }
}