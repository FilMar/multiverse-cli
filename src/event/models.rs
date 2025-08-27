//! Event entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

// Generate complete Event entity
define_complete_entity!(
    Event,
    EventStatus,
    EventDb,
    table: "events",
    key_fields: { 
        name: String 
    },
    fields: { 
        display_name: String,
        date_text: String,
        sort_key: i64
    },
    status_variants: [ Active, Inactive, Completed, Cancelled, Pending ],
    create_sql: "CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        display_name TEXT NOT NULL,
        date_text TEXT NOT NULL DEFAULT '',
        sort_key INTEGER NOT NULL DEFAULT 0,
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Active'
    )"
);

// Custom implementations for Event
impl Event {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }

    /// Count total events
    pub fn count_total() -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM events")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    /// Count events by status
    pub fn count_by_status() -> anyhow::Result<Vec<(String, i32)>> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) FROM events GROUP BY status ORDER BY status"
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

    /// Update date and calculate sort key
    pub fn update_date(&mut self, date_text: String) -> anyhow::Result<()> {
        use crate::timeline::TimelineDate;
        
        let timeline_date = TimelineDate::parse(&date_text)?;
        self.date_text = date_text;
        self.sort_key = timeline_date.sort_key() as i64;
        
        Ok(())
    }

    /// List events in chronological order (by sort_key)
    pub fn list_chronological() -> anyhow::Result<Vec<Event>> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM events ORDER BY sort_key ASC, created_at ASC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(Event {
                id: row.get("id")?,
                name: row.get("name")?,
                display_name: row.get("display_name")?,
                date_text: row.get("date_text")?,
                sort_key: row.get("sort_key")?,
                metadata: serde_json::from_str(&row.get::<_, String>("metadata")?).unwrap_or_default(),
                created_at: {
                    let created_at_str = row.get::<_, String>("created_at")?;
                    chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_e| rusqlite::Error::InvalidColumnType(
                            row.as_ref().column_index("created_at").unwrap(),
                            "created_at".to_string(),
                            rusqlite::types::Type::Text,
                        ))?
                        .with_timezone(&chrono::Utc)
                },
                status: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>("status")?)).unwrap_or(EventStatus::Active),
            })
        })?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        
        Ok(events)
    }
}