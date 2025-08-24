use super::models::{Character, CharacterStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize character tables in the database
pub fn init_character_tables(conn: &Connection) -> Result<()> {
    // Create characters table with flexible metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            metadata TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Active'
        )",
        [],
    ).context("Failed to create characters table")?;
    
    Ok(())
}

/// Create a new character
pub fn create_character(conn: &Connection, character: &Character) -> Result<()> {
    let status_str = match character.status {
        CharacterStatus::Active => "Active",
        CharacterStatus::Inactive => "Inactive",
        CharacterStatus::Deceased => "Deceased",
        CharacterStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&character.metadata)
        .context("Failed to serialize character metadata")?;
    
    conn.execute(
        "INSERT INTO characters (name, display_name, metadata, created_at, status) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            character.name,
            character.display_name,
            metadata_json,
            character.created_at.to_rfc3339(),
            status_str
        ],
    ).context("Failed to insert character")?;
    
    Ok(())
}

/// List all characters
pub fn list_characters(conn: &Connection) -> Result<Vec<Character>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, metadata, created_at, status 
         FROM characters ORDER BY created_at DESC"
    )?;
    
    let character_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(2)?;
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(3)?;
        
        let status = match status_str.as_str() {
            "Active" => CharacterStatus::Active,
            "Inactive" => CharacterStatus::Inactive,
            "Deceased" => CharacterStatus::Deceased,
            "Archived" => CharacterStatus::Archived,
            _ => CharacterStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(2, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Character {
            name: row.get(0)?,
            display_name: row.get(1)?,
            metadata,
            created_at,
            status,
        })
    })?;
    
    let mut characters = Vec::new();
    for character in character_iter {
        characters.push(character?);
    }
    
    Ok(characters)
}

/// Get a specific character by name
pub fn get_character(conn: &Connection, name: &str) -> Result<Option<Character>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, metadata, created_at, status 
         FROM characters WHERE name = ?1"
    )?;
    
    let mut rows = stmt.query_map([name], |row| {
        let metadata_str: String = row.get(2)?;
        let status_str: String = row.get(4)?;
        let created_at_str: String = row.get(3)?;
        
        let status = match status_str.as_str() {
            "Active" => CharacterStatus::Active,
            "Inactive" => CharacterStatus::Inactive,
            "Deceased" => CharacterStatus::Deceased,
            "Archived" => CharacterStatus::Archived,
            _ => CharacterStatus::Active,
        };
        
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
            
        let metadata: std::collections::HashMap<String, serde_json::Value> = 
            serde_json::from_str(&metadata_str)
                .map_err(|_e| rusqlite::Error::InvalidColumnType(2, "metadata".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Character {
            name: row.get(0)?,
            display_name: row.get(1)?,
            metadata,
            created_at,
            status,
        })
    })?;
    
    match rows.next() {
        Some(character) => Ok(Some(character?)),
        None => Ok(None),
    }
}

/// Delete a character
pub fn delete_character(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM characters WHERE name = ?1", [name])
        .context("Failed to delete character")?;
    
    Ok(())
}

/// Update an existing character
pub fn update_character(conn: &Connection, character: &Character) -> Result<()> {
    let status_str = match character.status {
        CharacterStatus::Active => "Active",
        CharacterStatus::Inactive => "Inactive",
        CharacterStatus::Deceased => "Deceased",
        CharacterStatus::Archived => "Archived",
    };

    let metadata_json = serde_json::to_string(&character.metadata)
        .context("Failed to serialize character metadata")?;

    conn.execute(
        "UPDATE characters SET display_name = ?1, metadata = ?2, status = ?3 WHERE name = ?4",
        params![
            character.display_name,
            metadata_json,
            status_str,
            character.name
        ],
    ).context("Failed to update character")?;

    Ok(())
}

/// Get total count of characters
pub fn count_characters(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM characters")?;
    let count: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

/// Get characters count by status
pub fn count_characters_by_status(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT status, COUNT(*) FROM characters GROUP BY status ORDER BY status"
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