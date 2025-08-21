use crate::database::{init_database, get_connection};
use crate::story::database::init_story_tables;
use anyhow::Result;
use std::path::Path;

/// Initialize world database with core tables
pub fn init_world_database(db_path: &Path) -> Result<()> {
    // Initialize basic database
    init_database(db_path)?;
    
    let conn = get_connection(db_path)?;
    
    // Initialize story tables
    init_story_tables(&conn)?;
    
    Ok(())
}

/// Check if world database exists and is valid
pub fn world_database_exists(db_path: &Path) -> bool {
    crate::database::database_exists(db_path)
}
