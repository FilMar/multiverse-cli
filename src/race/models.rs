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