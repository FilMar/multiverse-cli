use serde::{Deserialize, Serialize};

/// Event metadata and configuration with flexible metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub display_name: String,
    pub event_type: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: EventStatus,
    pub date: String,  // Timeline date string (e.g., "3A/2 Lum 124 DF" or "2024-03-15")
    pub sort_key: Option<u64>,  // Cached sort key for performance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Historical,
    Ongoing,
    Planned,
    Cancelled,
    Archived,
}

impl Default for EventStatus {
    fn default() -> Self {
        Self::Historical
    }
}

// Core interface
impl Event {
    /// Factory method with full validation
    pub fn create_new(name: String, display_name: String, event_type: String, date: Option<String>, set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
        let mut metadata = std::collections::HashMap::new();
        
        // Process set_args into metadata
        for (key, value) in set_args {
            metadata.insert(key, serde_json::Value::String(value));
        }
        
        let date = date.unwrap_or_else(|| "now".to_string());
        Ok(Self::new(name, display_name, event_type, date, metadata))
    }
    
    /// Direct constructor
    pub fn new(name: String, display_name: String, event_type: String, date: String, metadata: std::collections::HashMap<String, serde_json::Value>) -> Self {
        let mut event = Self {
            name,
            display_name,
            event_type,
            metadata,
            description: None,
            created_at: chrono::Utc::now(),
            status: EventStatus::Historical,
            date,
            sort_key: None,
        };
        
        // Try to update sort key immediately
        let _ = event.update_sort_key();
        event
    }
    
    /// Create event in database
    pub fn create(&self) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        
        if Self::check_event_exists(&self.name)? {
            anyhow::bail!("Event '{}' already exists", self.name);
        }
        
        self.save_to_database()?;
        
        Ok(())
    }
    
    /// List all events
    pub fn list() -> anyhow::Result<Vec<Self>> {
        use anyhow::Context;

        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::list_events(&conn)
            .context("Failed to list events")
    }
    
    /// Get event by name
    pub fn get(name: &str) -> anyhow::Result<Option<Self>> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::get_event(&conn, name)
    }
    
    /// Delete event
    pub fn delete(name: &str) -> anyhow::Result<()> {
        let _world_root = Self::ensure_world_context()?;
        let conn = Self::get_database_connection()?;
        
        super::database::delete_event(&conn, name)
    }
    
    /// Update the sort key based on current date
    pub fn update_sort_key(&mut self) -> anyhow::Result<()> {
        use anyhow::Context;
        
        // Handle special "now" case
        let date_str = if self.date == "now" {
            let timeline_date = crate::TimelineDate::now();
            self.date = timeline_date.to_string();
            &self.date
        } else {
            &self.date
        };
        
        let timeline_date = crate::TimelineDate::parse(date_str)
            .context("Failed to parse event date")?;
        
        self.sort_key = Some(timeline_date.sort_key());
        Ok(())
    }
    
    /// Get parsed timeline date
    pub fn get_timeline_date(&self) -> anyhow::Result<crate::TimelineDate> {
        crate::TimelineDate::parse(&self.date)
    }
    
    /// List events sorted chronologically
    pub fn list_chronological() -> anyhow::Result<Vec<Self>> {
        let mut events = Self::list()?;
        
        // Ensure all events have sort keys
        for event in &mut events {
            if event.sort_key.is_none() {
                let _ = event.update_sort_key();
            }
        }
        
        // Sort by sort_key
        events.sort_by_key(|e| e.sort_key.unwrap_or(0));
        Ok(events)
    }

    pub fn update(&mut self, display_name: Option<String>, event_type: Option<String>, date: Option<String>, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
        if let Some(display_name) = display_name {
            self.display_name = display_name;
        }

        if let Some(event_type) = event_type {
            self.event_type = event_type;
        }

        if let Some(date) = date {
            self.date = date;
            self.update_sort_key()?;
        }

        for (key, value) in set_args {
            self.metadata.insert(key, serde_json::Value::String(value));
        }

        self.update_in_database()?;

        Ok(())
    }
}

// Database integration methods
impl Event {
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
        super::database::create_event(&conn, self)
            .context("Failed to create event")
    }

    fn update_in_database(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let conn = Self::get_database_connection()?;
        super::database::update_event(&conn, self)
            .context("Failed to update event in database")
    }
    
    fn check_event_exists(name: &str) -> anyhow::Result<bool> {
        let conn = Self::get_database_connection()?;
        
        match super::database::get_event(&conn, name)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}