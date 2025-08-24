use super::models::{Location, LocationStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize location tables in the database
pub fn init_location_tables(conn: &Connection) -> Result<()> {
    // Create locations table with flexible metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS locations (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            metadata TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Active'
        )",
        [],
    ).context("Failed to create locations table")?;
    
    Ok(())
}

/// Create a new location
pub fn create_location(conn: &Connection, location: &Location) -> Result<()> {
    let status_str = match location.status {
        LocationStatus::Active => "Active",
        LocationStatus::Destroyed => "Destroyed",
        LocationStatus::Abandoned => "Abandoned",
        LocationStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&location.metadata)
        .context("Failed to serialize location metadata")?;
    
    conn.execute(
        "INSERT INTO locations (name, display_name, metadata, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            location.name,
            location.display_name,
            metadata_json,
            location.created_at.to_rfc3339(),
            status_str
        ],
    ).context("Failed to insert location")?;
    
    Ok(())
}

/// List all locations
pub fn list_locations(conn: &Connection) -> Result<Vec<Location>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, metadata, created_at, status 
         FROM locations ORDER BY created_at DESC"
    )?;
    
    let location_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(2)?;
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(3)?;
        
        let status = match status_str.as_str() {
            "Active" => LocationStatus::Active,
            "Destroyed" => LocationStatus::Destroyed,
            "Abandoned" => LocationStatus::Abandoned,
            "Archived" => LocationStatus::Archived,
            _ => LocationStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(2, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Location {
            name: row.get(0)?,
            display_name: row.get(1)?,
            metadata,
            created_at,
            status,
        })
    })?;
    
    let mut locations = Vec::new();
    for location in location_iter {
        locations.push(location?);
    }
    
    Ok(locations)
}

/// Get a specific location by name
pub fn get_location(conn: &Connection, name: &str) -> Result<Option<Location>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, metadata, created_at, status 
         FROM locations WHERE name = ?1"
    )?;
    
    let mut rows = stmt.query_map([name], |row| {
        let metadata_str: String = row.get(2)?;
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(3)?;
        
        let status = match status_str.as_str() {
            "Active" => LocationStatus::Active,
            "Destroyed" => LocationStatus::Destroyed,
            "Abandoned" => LocationStatus::Abandoned,
            "Archived" => LocationStatus::Archived,
            _ => LocationStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(2, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Location {
            name: row.get(0)?,
            display_name: row.get(1)?,
            metadata,
            created_at,
            status,
        })
    })?;
    
    match rows.next() {
        Some(location) => Ok(Some(location?)),
        None => Ok(None),
    }
}

/// Delete a location
pub fn delete_location(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM locations WHERE name = ?1", [name])
        .context("Failed to delete location")?;
    
    Ok(())
}

/// Update an existing location
pub fn update_location(conn: &Connection, location: &Location) -> Result<()> {
    let status_str = match location.status {
        LocationStatus::Active => "Active",
        LocationStatus::Destroyed => "Destroyed",
        LocationStatus::Abandoned => "Abandoned",
        LocationStatus::Archived => "Archived",
    };

    let metadata_json = serde_json::to_string(&location.metadata)
        .context("Failed to serialize location metadata")?;

    conn.execute(
        "UPDATE locations SET display_name = ?1, metadata = ?2, status = ?3 WHERE name = ?4",
        params![
            location.display_name,
            metadata_json,
            status_str,
            location.name
        ],
    ).context("Failed to update location")?;

    Ok(())
}