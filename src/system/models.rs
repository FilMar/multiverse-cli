//! System entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

define_complete_entity!(
    System,
    SystemStatus,
    SystemDb,
    table: "systems",
    key_fields: { name: String },
    fields: { 
        display_name: String,
        system_type: String
    },
    status_variants: [ Active, Inactive, Deprecated, Archived ],
    create_sql: "CREATE TABLE IF NOT EXISTS systems (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT UNIQUE NOT NULL,
        display_name TEXT NOT NULL,
        system_type TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Active',
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
    )"
);

// Custom implementations for System
impl System {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }
    
    /// Resolve system name to database ID
    pub fn resolve_id(name: &str) -> anyhow::Result<String> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT id FROM systems WHERE name = ?")?;
        let id: i32 = stmt.query_row([name], |row| {
            row.get(0)
        }).map_err(|_| anyhow::anyhow!("System not found: '{}'", name))?;
        Ok(id.to_string())
    }
}