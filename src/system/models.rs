use serde::{Deserialize, Serialize};

/// System metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    pub display_name: String,
    pub system_type: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: SystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Active,
    Inactive,
    Deprecated,
    Archived,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Core interface
impl System {
    /// Factory method with full validation
    pub fn create_new(name: String, display_name: String, system_type: String, set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
        let mut metadata = std::collections::HashMap::new();
        
        // Process set_args into metadata
        for (key, value) in set_args {
            metadata.insert(key, serde_json::Value::String(value));
        }
        
        Ok(Self::new(name, display_name, system_type, metadata))
    }
    
    /// Direct constructor
    pub fn new(name: String, display_name: String, system_type: String, metadata: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            name,
            display_name,
            system_type,
            metadata,
            description: None,
            created_at: chrono::Utc::now(),
            status: SystemStatus::Active,
        }
    }
    
    /// Create system in database
    pub fn create(&self) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        
        if Self::check_system_exists(&self.name)? {
            anyhow::bail!("System '{}' already exists", self.name);
        }
        
        self.save_to_database()?;
        
        Ok(())
    }
    
    /// List all systems
    pub fn list() -> anyhow::Result<Vec<Self>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_systems(&conn)
            .context("Failed to list systems")
    }
    
    /// Get system by name
    pub fn get(name: &str) -> anyhow::Result<Option<Self>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_system(&conn, name)
    }
    
    /// Delete system
    pub fn delete(name: &str) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::delete_system(&conn, name)
    }

    pub fn update(&mut self, display_name: Option<String>, system_type: Option<String>, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
        if let Some(display_name) = display_name {
            self.display_name = display_name;
        }

        if let Some(system_type) = system_type {
            self.system_type = system_type;
        }

        for (key, value) in set_args {
            self.metadata.insert(key, serde_json::Value::String(value));
        }

        self.update_in_database()?;

        Ok(())
    }
}

// Database integration methods
impl System {
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
        super::database::create_system(&conn, self)
            .context("Failed to create system")
    }

    fn update_in_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::update_system(&conn, self)
            .context("Failed to update system in database")
    }
    
    fn check_system_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        
        match super::database::get_system(&conn, name)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}