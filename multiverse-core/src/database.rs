use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use anyhow::{Result, Context};

/// Get a database connection for a specific database file
pub fn get_connection(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path.display()))?;
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])
        .context("Failed to enable foreign keys")?;
    
    Ok(conn)
}

/// Initialize a new database file with basic setup
pub fn init_database(db_path: &Path) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    
    let conn = get_connection(db_path)?;
    
    // Run initial migrations
    run_initial_migrations(&conn)
        .context("Failed to run initial database migrations")?;
    
    Ok(())
}

/// Run initial database migrations (creates core tables)
fn run_initial_migrations(conn: &Connection) -> SqliteResult<()> {
    // Create migrations tracking table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    Ok(())
}

/// Check if database file exists and is valid
pub fn database_exists(db_path: &Path) -> bool {
    if !db_path.exists() {
        return false;
    }
    
    // Try to open and verify it's a valid SQLite database
    match get_connection(db_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Execute a SQL statement safely
pub fn execute_sql(conn: &Connection, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
    conn.execute(sql, params)
        .with_context(|| format!("Failed to execute SQL: {}", sql))
}

/// Prepare a SQL statement safely  
pub fn prepare_statement<'a>(conn: &'a Connection, sql: &str) -> Result<rusqlite::Statement<'a>> {
    conn.prepare(sql)
        .with_context(|| format!("Failed to prepare SQL statement: {}", sql))
}
