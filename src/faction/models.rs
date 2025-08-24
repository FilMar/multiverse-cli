use serde::{Deserialize, Serialize};

/// Faction metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    pub display_name: String,
    pub faction_type: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: FactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionStatus {
    Active,
    Disbanded,
    Dormant,
    Archived,
}

impl Default for FactionStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Core interface
impl Faction {
    /// Factory method with full validation
    pub fn create_new(name: String, display_name: String, faction_type: String, set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
        let mut metadata = std::collections::HashMap::new();
        
        // Process set_args into metadata
        for (key, value) in set_args {
            metadata.insert(key, serde_json::Value::String(value));
        }
        
        Ok(Self::new(name, display_name, faction_type, metadata))
    }
    
    /// Direct constructor
    pub fn new(name: String, display_name: String, faction_type: String, metadata: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            name,
            display_name,
            faction_type,
            metadata,
            description: None,
            created_at: chrono::Utc::now(),
            status: FactionStatus::Active,
        }
    }
    
    /// Create faction in database
    pub fn create(&self) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        
        if Self::check_faction_exists(&self.name)? {
            anyhow::bail!("Faction '{}' already exists", self.name);
        }
        
        self.save_to_database()?;
        
        Ok(())
    }
    
    /// List all factions
    pub fn list() -> anyhow::Result<Vec<Self>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_factions(&conn)
            .context("Failed to list factions")
    }
    
    /// Get faction by name
    pub fn get(name: &str) -> anyhow::Result<Option<Self>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_faction(&conn, name)
    }
    
    /// Delete faction
    pub fn delete(name: &str) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::delete_faction(&conn, name)
    }

    pub fn update(&mut self, display_name: Option<String>, faction_type: Option<String>, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
        if let Some(display_name) = display_name {
            self.display_name = display_name;
        }

        if let Some(faction_type) = faction_type {
            self.faction_type = faction_type;
        }

        for (key, value) in set_args {
            self.metadata.insert(key, serde_json::Value::String(value));
        }

        self.update_in_database()?;

        Ok(())
    }
}

// Database integration methods
impl Faction {
    fn get_database_connection() -> anyhow::Result<rusqlite::Connection> {
        use anyhow::Context;
        
        let db_path = std::path::Path::new(".multiverse/world.db");
        crate::database::get_connection(db_path)
            .context("Failed to connect to world database")
    }
    
    fn ensure_world_context() -> anyhow::Result<std::path::PathBuf> {
        let current_dir = std::env::current_dir()?;
        let multiverse_dir = current_dir.join(".multiverse");
        
        if !multiverse_dir.exists() {
            anyhow::bail!("Not in a multiverse project directory. Run 'multiverse world init' first.");
        }
        
        Ok(current_dir)
    }
    
    fn save_to_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;
        
        let conn = Self::get_database_connection()?;
        super::database::create_faction(&conn, self)
            .context("Failed to create faction")
    }

    fn update_in_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::update_faction(&conn, self)
            .context("Failed to update faction in database")
    }
    
    fn check_faction_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        
        match super::database::get_faction(&conn, name)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}