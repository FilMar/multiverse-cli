//! Character entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

// Generate complete Character entity
define_complete_entity!(
    Character,
    CharacterStatus,
    CharacterDb,
    table: "characters",
    key_fields: { 
        name: String 
    },
    fields: { 
        display_name: String 
    },
    status_variants: [ Active, Inactive, Deceased, Archived ],
    create_sql: "CREATE TABLE IF NOT EXISTS characters (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        display_name TEXT NOT NULL,
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Active'
    )"
);

// Custom implementations for Character
impl Character {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }
}
