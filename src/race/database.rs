use super::models::Race;
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize race tables in the database
pub fn init_races_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS races (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            description TEXT,
            metadata TEXT,
            created_at TEXT NOT NULL
        )",
        [],
    ).context("Failed to create races table")?;
    Ok(())
}

/// Create a new race
pub fn create_race(conn: &Connection, race: &Race) -> Result<()> {
    let metadata_json = serde_json::to_string(&race.metadata)
        .context("Failed to serialize race metadata")?;
    
    conn.execute(
        "INSERT INTO races (name, display_name, description, metadata, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            race.name,
            race.display_name,
            race.description,
            metadata_json,
            race.created_at.to_rfc3339(),
        ],
    ).context("Failed to insert race")?;
    
    Ok(())
}

/// List all races
pub fn list_races(conn: &Connection) -> Result<Vec<Race>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, description, metadata, created_at 
         FROM races ORDER BY created_at DESC"
    )?;
    
    let race_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(3)?;
        let created_at_str: String = row.get(4)?;
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Race {
            name: row.get(0)?,
            display_name: row.get(1)?,
            description: row.get(2)?,
            metadata,
            created_at,
        })
    })?;
    
    let mut races = Vec::new();
    for race in race_iter {
        races.push(race?);
    }
    
    Ok(races)
}

/// Get a specific race by name
pub fn get_race(conn: &Connection, name: &str) -> Result<Option<Race>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, description, metadata, created_at 
         FROM races WHERE name = ?1"
    )?;
    
    let mut rows = stmt.query_map([name], |row| {
        let metadata_str: String = row.get(3)?;
        let created_at_str: String = row.get(4)?;
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Race {
            name: row.get(0)?,
            display_name: row.get(1)?,
            description: row.get(2)?,
            metadata,
            created_at,
        })
    })?;
    
    match rows.next() {
        Some(race) => Ok(Some(race?)),
        None => Ok(None),
    }
}

/// Delete a race
pub fn delete_race(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM races WHERE name = ?1", [name])
        .context("Failed to delete race")?;
    
    Ok(())
}

/// Update an existing race
pub fn update_race(conn: &Connection, race: &Race) -> Result<()> {
    let metadata_json = serde_json::to_string(&race.metadata)
        .context("Failed to serialize race metadata")?;

    conn.execute(
        "UPDATE races SET display_name = ?1, description = ?2, metadata = ?3 WHERE name = ?4",
        params![
            race.display_name,
            race.description,
            metadata_json,
            race.name
        ],
    ).context("Failed to update race")?;

    Ok(())
}
