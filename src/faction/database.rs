use super::models::{Faction, FactionStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize faction tables in the database
pub fn init_faction_tables(conn: &Connection) -> Result<()> {
    // Create factions table with flexible metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS factions (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            faction_type TEXT NOT NULL,
            metadata TEXT,
            description TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Active'
        )",
        [],
    ).context("Failed to create factions table")?;
    
    Ok(())
}

/// Create a new faction
pub fn create_faction(conn: &Connection, faction: &Faction) -> Result<()> {
    let status_str = match faction.status {
        FactionStatus::Active => "Active",
        FactionStatus::Disbanded => "Disbanded", 
        FactionStatus::Dormant => "Dormant",
        FactionStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&faction.metadata)
        .context("Failed to serialize faction metadata")?;
    
    conn.execute(
        "INSERT INTO factions (name, display_name, faction_type, metadata, description, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            faction.name,
            faction.display_name,
            faction.faction_type,
            metadata_json,
            faction.description,
            faction.created_at.to_rfc3339(),
            status_str
        ],
    ).context("Failed to insert faction")?;
    
    Ok(())
}

/// Get a faction by name
pub fn get_faction(conn: &Connection, name: &str) -> Result<Option<Faction>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, faction_type, metadata, description, created_at, status 
         FROM factions WHERE name = ?1"
    ).context("Failed to prepare get faction query")?;
    
    let faction_result = stmt.query_row(params![name], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Active" => FactionStatus::Active,
            "Disbanded" => FactionStatus::Disbanded,
            "Dormant" => FactionStatus::Dormant,
            "Archived" => FactionStatus::Archived,
            _ => FactionStatus::Active,
        };
        
        Ok(Faction {
            name: row.get(0)?,
            display_name: row.get(1)?,
            faction_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    });
    
    match faction_result {
        Ok(faction) => Ok(Some(faction)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(anyhow::anyhow!(e).context("Failed to get faction")),
    }
}

/// List all factions
pub fn list_factions(conn: &Connection) -> Result<Vec<Faction>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, faction_type, metadata, description, created_at, status 
         FROM factions ORDER BY created_at DESC"
    ).context("Failed to prepare list factions query")?;
    
    let faction_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Active" => FactionStatus::Active,
            "Disbanded" => FactionStatus::Disbanded,
            "Dormant" => FactionStatus::Dormant,
            "Archived" => FactionStatus::Archived,
            _ => FactionStatus::Active,
        };
        
        Ok(Faction {
            name: row.get(0)?,
            display_name: row.get(1)?,
            faction_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
        })
    }).context("Failed to execute list factions query")?;
    
    faction_iter.collect::<Result<Vec<Faction>, rusqlite::Error>>()
        .context("Failed to collect factions")
}

/// Delete a faction
pub fn delete_faction(conn: &Connection, name: &str) -> Result<()> {
    let rows_affected = conn.execute(
        "DELETE FROM factions WHERE name = ?1",
        params![name],
    ).context("Failed to delete faction")?;
    
    if rows_affected == 0 {
        return Err(anyhow::anyhow!("Faction '{}' not found", name));
    }
    
    Ok(())
}

/// Update an existing faction
pub fn update_faction(conn: &Connection, faction: &Faction) -> Result<()> {
    let status_str = match faction.status {
        FactionStatus::Active => "Active",
        FactionStatus::Disbanded => "Disbanded",
        FactionStatus::Dormant => "Dormant",
        FactionStatus::Archived => "Archived",
    };

    let metadata_json = serde_json::to_string(&faction.metadata)
        .context("Failed to serialize faction metadata")?;

    conn.execute(
        "UPDATE factions SET display_name = ?1, faction_type = ?2, metadata = ?3, description = ?4, status = ?5 WHERE name = ?6",
        params![
            faction.display_name,
            faction.faction_type,
            metadata_json,
            faction.description,
            status_str,
            faction.name
        ],
    ).context("Failed to update faction")?;

    Ok(())
}

/// Count factions
pub fn count_factions(conn: &Connection) -> Result<i32> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM factions",
        [],
        |row| row.get(0)
    ).context("Failed to count factions")?;
    
    Ok(count)
}

/// Count factions by status
pub fn count_factions_by_status(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT status, COUNT(*) FROM factions GROUP BY status"
    ).context("Failed to prepare factions count by status query")?;
    
    let status_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    }).context("Failed to execute factions count by status query")?;
    
    status_iter.collect::<Result<Vec<(String, i32)>, rusqlite::Error>>()
        .context("Failed to collect factions status counts")
}