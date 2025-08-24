use super::models::{Event, EventStatus};
use anyhow::{Result, Context};
use rusqlite::{Connection, params};

/// Initialize event tables in the database
pub fn init_event_tables(conn: &Connection) -> Result<()> {
    // Create events table with flexible metadata and timeline support
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            name TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            event_type TEXT NOT NULL,
            metadata TEXT,
            description TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Historical',
            date TEXT NOT NULL DEFAULT 'now',
            sort_key INTEGER
        )",
        [],
    ).context("Failed to create events table")?;
    
    Ok(())
}

/// Create a new event
pub fn create_event(conn: &Connection, event: &Event) -> Result<()> {
    let status_str = match event.status {
        EventStatus::Historical => "Historical",
        EventStatus::Ongoing => "Ongoing", 
        EventStatus::Planned => "Planned",
        EventStatus::Cancelled => "Cancelled",
        EventStatus::Archived => "Archived",
    };
    
    let metadata_json = serde_json::to_string(&event.metadata)
        .context("Failed to serialize event metadata")?;
    
    conn.execute(
        "INSERT INTO events (name, display_name, event_type, metadata, description, created_at, status, date, sort_key) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            event.name,
            event.display_name,
            event.event_type,
            metadata_json,
            event.description,
            event.created_at.to_rfc3339(),
            status_str,
            event.date,
            event.sort_key
        ],
    ).context("Failed to insert event")?;
    
    Ok(())
}

/// Get an event by name
pub fn get_event(conn: &Connection, name: &str) -> Result<Option<Event>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, event_type, metadata, description, created_at, status, date, sort_key 
         FROM events WHERE name = ?1"
    ).context("Failed to prepare get event query")?;
    
    let event_result = stmt.query_row(params![name], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Historical" => EventStatus::Historical,
            "Ongoing" => EventStatus::Ongoing,
            "Planned" => EventStatus::Planned,
            "Cancelled" => EventStatus::Cancelled,
            "Archived" => EventStatus::Archived,
            _ => EventStatus::Historical,
        };
        
        let date: String = row.get(7)?;
        let sort_key: Option<i64> = row.get(8).ok();
        let sort_key = sort_key.map(|k| k as u64);
        
        Ok(Event {
            name: row.get(0)?,
            display_name: row.get(1)?,
            event_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
            date,
            sort_key,
        })
    });
    
    match event_result {
        Ok(event) => Ok(Some(event)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(anyhow::anyhow!(e).context("Failed to get event")),
    }
}

/// List all events
pub fn list_events(conn: &Connection) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT name, display_name, event_type, metadata, description, created_at, status, date, sort_key 
         FROM events ORDER BY created_at DESC"
    ).context("Failed to prepare list events query")?;
    
    let event_iter = stmt.query_map([], |row| {
        let metadata_str: String = row.get(3)?;
        let metadata = serde_json::from_str(&metadata_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("JSON parse error: {}", e).into(), rusqlite::types::Type::Text))?;
        
        let created_at_str: String = row.get(5)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("DateTime parse error: {}", e).into(), rusqlite::types::Type::Text))?
            .with_timezone(&chrono::Utc);
        
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Historical" => EventStatus::Historical,
            "Ongoing" => EventStatus::Ongoing,
            "Planned" => EventStatus::Planned,
            "Cancelled" => EventStatus::Cancelled,
            "Archived" => EventStatus::Archived,
            _ => EventStatus::Historical,
        };
        
        let date: String = row.get(7)?;
        let sort_key: Option<i64> = row.get(8).ok();
        let sort_key = sort_key.map(|k| k as u64);
        
        Ok(Event {
            name: row.get(0)?,
            display_name: row.get(1)?,
            event_type: row.get(2)?,
            metadata,
            description: row.get(4)?,
            created_at,
            status,
            date,
            sort_key,
        })
    }).context("Failed to execute list events query")?;
    
    event_iter.collect::<Result<Vec<Event>, rusqlite::Error>>()
        .context("Failed to collect events")
}

/// Delete an event
pub fn delete_event(conn: &Connection, name: &str) -> Result<()> {
    let rows_affected = conn.execute(
        "DELETE FROM events WHERE name = ?1",
        params![name],
    ).context("Failed to delete event")?;
    
    if rows_affected == 0 {
        return Err(anyhow::anyhow!("Event '{}' not found", name));
    }
    
    Ok(())
}

/// Update an existing event
pub fn update_event(conn: &Connection, event: &Event) -> Result<()> {
    let status_str = match event.status {
        EventStatus::Historical => "Historical",
        EventStatus::Ongoing => "Ongoing",
        EventStatus::Planned => "Planned",
        EventStatus::Cancelled => "Cancelled",
        EventStatus::Archived => "Archived",
    };

    let metadata_json = serde_json::to_string(&event.metadata)
        .context("Failed to serialize event metadata")?;

    conn.execute(
        "UPDATE events SET display_name = ?1, event_type = ?2, metadata = ?3, description = ?4, status = ?5, date = ?6, sort_key = ?7 WHERE name = ?8",
        params![
            event.display_name,
            event.event_type,
            metadata_json,
            event.description,
            status_str,
            event.date,
            event.sort_key,
            event.name
        ],
    ).context("Failed to update event")?;

    Ok(())
}

/// Count events
pub fn count_events(conn: &Connection) -> Result<i32> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM events",
        [],
        |row| row.get(0)
    ).context("Failed to count events")?;
    
    Ok(count)
}

/// Count events by status
pub fn count_events_by_status(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT status, COUNT(*) FROM events GROUP BY status"
    ).context("Failed to prepare events count by status query")?;
    
    let status_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    }).context("Failed to execute events count by status query")?;
    
    status_iter.collect::<Result<Vec<(String, i32)>, rusqlite::Error>>()
        .context("Failed to collect events status counts")
}