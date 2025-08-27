//! Faction entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

// Generate complete Faction entity
define_complete_entity!(
    Faction,
    FactionStatus,
    FactionDb,
    table: "factions",
    key_fields: { 
        name: String 
    },
    fields: { 
        display_name: String
    },
    status_variants: [ Active, Inactive, Disbanded, Allied, Hostile ],
    create_sql: "CREATE TABLE IF NOT EXISTS factions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        display_name TEXT NOT NULL,
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Active'
    )"
);

// Custom implementations for Faction
impl Faction {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }

    /// Count total factions
    pub fn count_total() -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM factions")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    /// Count factions by status
    pub fn count_by_status() -> anyhow::Result<Vec<(String, i32)>> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) FROM factions GROUP BY status ORDER BY status"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }
}