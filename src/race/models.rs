//! Race entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

define_complete_entity!(
    Race,
    RaceStatus,
    RaceDb,
    table: "races",
    key_fields: { name: String },
    fields: { 
        display_name: String
    },
    status_variants: [ Active, Inactive, Extinct, Legendary, Mythical ],
    create_sql: "CREATE TABLE IF NOT EXISTS races (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT UNIQUE NOT NULL,
        display_name TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Active',
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
    )"
);

// Custom implementations for Race
impl Race {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }
    
    /// Resolve race name to database ID
    pub fn resolve_id(name: &str) -> anyhow::Result<String> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT id FROM races WHERE name = ?")?;
        let id: i32 = stmt.query_row([name], |row| {
            row.get(0)
        }).map_err(|_| anyhow::anyhow!("Race not found: '{}'", name))?;
        Ok(id.to_string())
    }
}