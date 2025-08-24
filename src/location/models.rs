use serde::{Deserialize, Serialize};

/// Location metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub display_name: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: LocationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationStatus {
    Active,
    Destroyed,
    Abandoned,
    Archived,
}

impl Default for LocationStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Core interface
impl Location {
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
            created_at: chrono::Utc::now(),
            status: LocationStatus::Active,
        }
    }

    /// Create location in database
    pub fn create(&self) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        
        if Self::check_location_exists(&self.name)? {
            anyhow::bail!("Location '{}' already exists", self.name);
        }
        
        self.save_to_database()?;
        
        Ok(())
    }

    /// List all locations
    pub fn list() -> anyhow::Result<Vec<Location>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_locations(&conn)
            .context("Failed to list locations")
    }

    /// Get a location by name
    pub fn get(name: &str) -> anyhow::Result<Option<Location>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_location(&conn, name)
    }

    /// Delete location from database
    pub fn delete(&self, force: bool) -> anyhow::Result<()> {
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }

        let _world_root = Self::ensure_world_context()?;
        
        self.delete_from_database()?;
        
        Ok(())
    }

    pub fn update(&mut self, display_name: Option<String>, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
        if let Some(display_name) = display_name {
            self.display_name = display_name;
        }

        for (key, value) in set_args {
            self.metadata.insert(key, serde_json::Value::String(value));
        }

        self.update_in_database()?;

        Ok(())
    }
}

// Private utility functions
impl Location {
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

    fn check_location_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        Ok(super::database::get_location(&conn, name)?.is_some())
    }

    fn save_to_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::create_location(&conn, self)
            .context("Failed to save location to database")
    }

    fn update_in_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::update_location(&conn, self)
            .context("Failed to update location in database")
    }

    fn delete_from_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::delete_location(&conn, &self.name)
            .context("Failed to delete location from database")
    }
}