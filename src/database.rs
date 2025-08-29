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

/// Execute a SELECT query and return results as formatted table
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
        println!("No columns returned");
        return Ok(());
    }
    
    // Execute query and collect all rows
    let rows = stmt.query_map([], |row| {
        let mut values = Vec::new();
        for i in 0..column_names.len() {
            // Try to get as string, fallback for different types
            let value = match row.get::<_, String>(i) {
                Ok(s) => s,
                Err(_) => {
                    // Try other types
                    if let Ok(i) = row.get::<_, i64>(i) {
                        i.to_string()
                    } else if let Ok(f) = row.get::<_, f64>(i) {
                        f.to_string()
                    } else {
                        "NULL".to_string()
                    }
                }
            };
            values.push(value);
        }
        Ok(values)
    })?;
    
    let mut all_rows = Vec::new();
    for row in rows {
        all_rows.push(row?);
    }
    
    // Print results as table
    print_table(&column_names, &all_rows);
    
    Ok(())
}

/// Print results in a nice table format
fn print_table(headers: &[String], rows: &[Vec<String>]) {
    if rows.is_empty() {
        println!("📊 No results found");
        return;
    }
    
    // Calculate column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, value) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(value.len());
            }
        }
    }
    
    // Print header
    print!("┌");
    for (i, width) in widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < widths.len() - 1 {
            print!("┬");
        }
    }
    println!("┐");
    
    print!("│");
    for (i, (header, width)) in headers.iter().zip(&widths).enumerate() {
        print!(" {:width$} ", header, width = width);
        if i < headers.len() - 1 {
            print!("│");
        }
    }
    println!("│");
    
    // Print separator
    print!("├");
    for (i, width) in widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < widths.len() - 1 {
            print!("┼");
        }
    }
    println!("┤");
    
    // Print rows
    for row in rows {
        print!("│");
        for (i, (value, width)) in row.iter().zip(&widths).enumerate() {
            print!(" {:width$} ", value, width = width);
            if i < row.len() - 1 {
                print!("│");
            }
        }
        println!("│");
    }
    
    // Print bottom
    print!("└");
    for (i, width) in widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < widths.len() - 1 {
            print!("┴");
        }
    }
    println!("┘");
    
    println!("📊 {} row(s) returned", rows.len());
}

