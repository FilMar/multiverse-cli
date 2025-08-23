use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize episode-character relationship tables
pub fn init_episode_character_tables(conn: &Connection) -> Result<()> {
    // Create episode_characters relationship table (many-to-many)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS episode_characters (
            episode_id INTEGER NOT NULL,
            character_name TEXT NOT NULL,
            role TEXT NOT NULL,
            importance TEXT DEFAULT 'Supporting',
            created_at TEXT NOT NULL,
            PRIMARY KEY (episode_id, character_name),
            FOREIGN KEY (episode_id) REFERENCES episodes (id) ON DELETE CASCADE,
            FOREIGN KEY (character_name) REFERENCES characters (name) ON DELETE CASCADE
        )",
        [],
    ).context("Failed to create episode_characters table")?;
    
    Ok(())
}

/// Add a character to an episode with specific role
pub fn add_character_to_episode(conn: &Connection, episode_id: i32, character_name: &str, role: &str, importance: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO episode_characters (episode_id, character_name, role, importance, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            episode_id,
            character_name,
            role,
            importance,
            chrono::Utc::now().to_rfc3339()
        ],
    ).context("Failed to add character to episode")?;
    
    Ok(())
}

/// Remove a character from an episode
pub fn remove_character_from_episode(conn: &Connection, episode_id: i32, character_name: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM episode_characters WHERE episode_id = ?1 AND character_name = ?2",
        params![episode_id, character_name]
    ).context("Failed to remove character from episode")?;
    
    Ok(())
}

/// Get characters for a specific episode with their roles
pub fn get_episode_characters(conn: &Connection, episode_id: i32) -> Result<Vec<(String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT character_name, role, importance FROM episode_characters WHERE episode_id = ?1 ORDER BY importance, character_name"
    )?;
    
    let rows = stmt.query_map([episode_id], |row| {
        Ok((
            row.get::<_, String>(0)?, // character_name
            row.get::<_, String>(1)?, // role
            row.get::<_, String>(2)?  // importance
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

/// Get episodes where a character appears with their roles
pub fn get_character_episodes(conn: &Connection, character_name: &str) -> Result<Vec<(i32, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT episode_id, role, importance FROM episode_characters WHERE character_name = ?1 ORDER BY episode_id"
    )?;
    
    let rows = stmt.query_map([character_name], |row| {
        Ok((
            row.get::<_, i32>(0)?,    // episode_id
            row.get::<_, String>(1)?, // role
            row.get::<_, String>(2)?  // importance
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

/// Update character role in an episode
pub fn update_character_role_in_episode(conn: &Connection, episode_id: i32, character_name: &str, role: &str, importance: &str) -> Result<()> {
    conn.execute(
        "UPDATE episode_characters SET role = ?3, importance = ?4 WHERE episode_id = ?1 AND character_name = ?2",
        params![episode_id, character_name, role, importance]
    ).context("Failed to update character role in episode")?;
    
    Ok(())
}