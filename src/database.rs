use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use anyhow::{Result, Context};
use serde_json::{json, Value as JsonValue};

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

/// Validate that a SQL query is a SELECT-only statement (security check)
fn validate_select_only(sql: &str) -> Result<()> {
    let sql_trimmed = sql.trim().to_lowercase();
    
    // Must start with SELECT
    if !sql_trimmed.starts_with("select") {
        return Err(anyhow::anyhow!("Only SELECT queries are allowed"));
    }
    
    // Check for dangerous keywords (basic security)
    let forbidden = ["insert", "update", "delete", "drop", "create", "alter", "pragma"];
    for keyword in forbidden {
        if sql_trimmed.contains(keyword) {
            return Err(anyhow::anyhow!("Query contains forbidden keyword: {}", keyword));
        }
    }
    
    Ok(())
}

/// Execute a SELECT query and return results as JSON
pub fn execute_query(sql: &str) -> Result<()> {
    // Validate query is SELECT-only
    validate_select_only(sql)?;
    
    // Get database connection
    let db_path = crate::world::WorldConfig::get_database_path()?;
    let conn = get_connection(&db_path)?;
    
    // Prepare and execute query
    let mut stmt = conn.prepare(sql)
        .with_context(|| format!("Failed to prepare query: {}", sql))?;
    
    // Get column names
    let column_names: Vec<String> = stmt.column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();
    
    if column_names.is_empty() {
        let result = json!({
            "query": sql,
            "columns": [],
            "rows": [],
            "count": 0
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }
    
    // Execute query - rusqlite non ha conversione automatica, tocca farlo a mano
    let mut all_rows = Vec::new();
    let rows = stmt.query_map([], |row| {
        let mut row_map = serde_json::Map::new();
        
        for (i, col_name) in column_names.iter().enumerate() {
            // Rusqlite value handling - prova i tipi più comuni
            let value = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => JsonValue::Null,
                rusqlite::types::ValueRef::Integer(i) => JsonValue::Number(serde_json::Number::from(i)),
                rusqlite::types::ValueRef::Real(f) => {
                    if let Some(n) = serde_json::Number::from_f64(f) {
                        JsonValue::Number(n)
                    } else {
                        JsonValue::String(f.to_string())
                    }
                },
                rusqlite::types::ValueRef::Text(s) => {
                    let text = String::from_utf8_lossy(s).to_string();
                    // Try to parse as JSON first, fallback to string
                    serde_json::from_str(&text).unwrap_or(JsonValue::String(text))
                },
                rusqlite::types::ValueRef::Blob(b) => {
                    JsonValue::String(format!("<blob:{} bytes>", b.len()))
                }
            };
            row_map.insert(col_name.clone(), value);
        }
        
        Ok(JsonValue::Object(row_map))
    })?;
    
    for row_result in rows {
        all_rows.push(row_result?);
    }
    
    // Create structured JSON response
    let result = json!({
        "query": sql,
        "columns": column_names,
        "rows": all_rows,
        "count": all_rows.len()
    });
    
    // Pretty-print JSON
    println!("{}", serde_json::to_string_pretty(&result)?);
    
    Ok(())
}


