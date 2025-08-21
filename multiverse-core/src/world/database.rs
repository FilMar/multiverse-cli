use crate::database::{execute_sql, init_database};
use rusqlite::Connection;
use anyhow::Result;
use std::path::Path;

/// Initialize world database with core tables
pub fn init_world_database(db_path: &Path) -> Result<()> {
    // Initialize basic database
    init_database(db_path)?;
    
    let conn = crate::database::get_connection(db_path)?;
    
    // For now, just ensure the database is properly initialized
    // Other modules (series, episodes, characters, etc.) will add their own tables
    
    Ok(())
}

/// Check if world database exists and is valid
pub fn world_database_exists(db_path: &Path) -> bool {
    crate::database::database_exists(db_path)
}