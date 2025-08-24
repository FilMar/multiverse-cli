use super::models::{System, SystemStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize system tables in the database
pub fn init_system_tables(conn: &Connection) -> Result<()> {
    // Create systems table with flexible metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS systems (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            system_type TEXT NOT NULL,
            metadata TEXT,
            description TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Active'
        )",
        [],
    ).context("Failed to create systems table")?;
    
    Ok(())
}

/// Create a new system
pub fn create_system(conn: &Connection, system: &System) -> Result<()> {
    let status_str = match system.status {
        SystemStatus::Active => "Active",
        SystemStatus::Inactive => "Inactive", 
        SystemStatus::Deprecated => "Deprecated",
        SystemStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&system.metadata)
        .context("Failed to serialize system metadata")?;
    
    conn.execute(
        "INSERT INTO systems (name, display_name, system_type, metadata, description, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            system.name,
            system.display_name,
            system.system_type,
            metadata_json,
            system.description,
            system.created_at.to_rfc3339(),
            status_str
        ],
    ).context("Failed to insert system")?;
    
    Ok(())
}

/// Get a system by name
pub fn get_system(conn: &Connection, name: &str) -> Result<Option<System>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, system_type, metadata, description, created_at, status 
         FROM systems WHERE name = ?1"
    ).context("Failed to prepare get system query")?;
    
    let system_result = stmt.query_row(params![name], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Active" => SystemStatus::Active,
            "Inactive" => SystemStatus::Inactive,
            "Deprecated" => SystemStatus::Deprecated,
            "Archived" => SystemStatus::Archived,
            _ => SystemStatus::Active,
        };
        
        Ok(System {
            name: row.get(0)?,
            display_name: row.get(1)?,
            system_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    });
    
    match system_result {
        Ok(system) => Ok(Some(system)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(anyhow::anyhow!(e).context("Failed to get system")),
    }
}

/// List all systems
pub fn list_systems(conn: &Connection) -> Result<Vec<System>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, system_type, metadata, description, created_at, status 
         FROM systems ORDER BY created_at DESC"
    ).context("Failed to prepare list systems query")?;
    
    let system_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Active" => SystemStatus::Active,
            "Inactive" => SystemStatus::Inactive,
            "Deprecated" => SystemStatus::Deprecated,
            "Archived" => SystemStatus::Archived,
            _ => SystemStatus::Active,
        };
        
        Ok(System {
            name: row.get(0)?,
            display_name: row.get(1)?,
            system_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    }).context("Failed to execute list systems query")?;
    
    system_iter.collect::<Result<Vec<System>, rusqlite::Error>>()
        .context("Failed to collect systems")
}

/// Delete a system
pub fn delete_system(conn: &Connection, name: &str) -> Result<()> {
    let rows_affected = conn.execute(
        "DELETE FROM systems WHERE name = ?1",
        params![name],
    ).context("Failed to delete system")?;
    
    if rows_affected == 0 {
        return Err(anyhow::anyhow!("System '{}' not found", name));
    }
    
    Ok(())
}

/// Update an existing system
pub fn update_system(conn: &Connection, system: &System) -> Result<()> {
    let status_str = match system.status {
        SystemStatus::Active => "Active",
        SystemStatus::Inactive => "Inactive",
        SystemStatus::Deprecated => "Deprecated",
        SystemStatus::Archived => "Archived",
    };

    let metadata_json = serde_json::to_string(&system.metadata)
        .context("Failed to serialize system metadata")?;

    conn.execute(
        "UPDATE systems SET display_name = ?1, system_type = ?2, metadata = ?3, description = ?4, status = ?5 WHERE name = ?6",
        params![
            system.display_name,
            system.system_type,
            metadata_json,
            system.description,
            status_str,
            system.name
        ],
    ).context("Failed to update system")?;

    Ok(())
}

/// Count systems by status
pub fn count_systems(conn: &Connection) -> Result<i32> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM systems",
        [],
        |row| row.get(0)
    ).context("Failed to count systems")?;
    
    Ok(count)
}

/// Count systems by status
pub fn count_systems_by_status(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT status, COUNT(*) FROM systems GROUP BY status"
    ).context("Failed to prepare systems count by status query")?;
    
    let status_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    }).context("Failed to execute systems count by status query")?;
    
    status_iter.collect::<Result<Vec<(String, i32)>, rusqlite::Error>>()
        .context("Failed to collect systems status counts")
}