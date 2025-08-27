//! Character-Faction relation implementation using macros
//! Handles character faction membership with --set factions=name:rank:loyalty

use crate::define_complete_relation;
use crate::define_relation_db_struct;
use crate::relations::models::Relation;

// ================================
// LOW-LEVEL DATABASE OPERATIONS
// ================================

// Generate the database handler struct for direct CRUD operations
define_relation_db_struct!(
    CharacterFactionDb,
    table: "character_factions",
    key_fields: {
        character_name: String,
        faction_name: String
    },
    fields: {
        rank: String,
        loyalty: String,
    },
    create_sql: "CREATE TABLE IF NOT EXISTS character_factions (
        character_name TEXT NOT NULL,
        faction_name TEXT NOT NULL,
        rank TEXT DEFAULT 'Member',
        loyalty TEXT DEFAULT 'Neutral',
        created_at TEXT NOT NULL,
        PRIMARY KEY (character_name, faction_name),
        FOREIGN KEY (character_name) REFERENCES characters (name) ON DELETE CASCADE,
        FOREIGN KEY (faction_name) REFERENCES factions (name) ON DELETE CASCADE
    )"
);

// ================================
// HIGH-LEVEL RELATION (meta-driven)
// ================================

// Generate the complete relation system for meta-driven operations
define_complete_relation!(
    CharacterFaction,
    table: "character_factions",
    key_fields: {
        character_name: String,
        faction_name: String
    },
    fields: {
        rank: String,
        loyalty: String,
    },
    sql: "INSERT INTO character_factions (character_name, faction_name, rank, loyalty, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    update_fields: [rank, loyalty],
    parser: {
        name: parse_character_faction_string,
        struct: CharacterFactionRelation,
        format: "faction_name:rank[:loyalty]"
    },
    processor: {
        name: process_character_faction_relations,
        init_fn: CharacterFactionDb::init_table
    },
    db_struct: CharacterFactionDb
);

// ================================
// BACKWARD COMPATIBILITY
// ================================

// For compatibility with existing code
pub fn init_character_faction_tables(conn: &rusqlite::Connection) -> anyhow::Result<()> {
    CharacterFactionDb::init_table(conn)
}

// Get all factions for a character
pub fn get_character_factions(
    conn: &rusqlite::Connection,
    character_name: &str
) -> anyhow::Result<Vec<(String, String, String)>> {
    let sql = "SELECT faction_name, rank, loyalty FROM character_factions WHERE character_name = ?1 ORDER BY rank DESC, faction_name";
    
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([character_name], |row| {
        Ok((
            row.get::<_, String>(0)?, // faction_name
            row.get::<_, String>(1)?, // rank
            row.get::<_, String>(2)?, // loyalty
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

// Get all members of a faction
pub fn get_faction_members(
    conn: &rusqlite::Connection,
    faction_name: &str
) -> anyhow::Result<Vec<(String, String, String)>> {
    let sql = "SELECT character_name, rank, loyalty FROM character_factions WHERE faction_name = ?1 ORDER BY rank DESC, character_name";
    
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([faction_name], |row| {
        Ok((
            row.get::<_, String>(0)?, // character_name
            row.get::<_, String>(1)?, // rank
            row.get::<_, String>(2)?, // loyalty
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

// Get faction membership statistics by rank
pub fn get_faction_rank_stats(
    conn: &rusqlite::Connection,
    faction_name: &str
) -> anyhow::Result<Vec<(String, i32)>> {
    let sql = "SELECT rank, COUNT(*) FROM character_factions WHERE faction_name = ?1 GROUP BY rank ORDER BY COUNT(*) DESC";
    
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([faction_name], |row| {
        Ok((
            row.get::<_, String>(0)?, // rank
            row.get::<_, i32>(1)?,    // count
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

// Get faction loyalty statistics
pub fn get_faction_loyalty_stats(
    conn: &rusqlite::Connection,
    faction_name: &str
) -> anyhow::Result<Vec<(String, i32)>> {
    let sql = "SELECT loyalty, COUNT(*) FROM character_factions WHERE faction_name = ?1 GROUP BY loyalty ORDER BY COUNT(*) DESC";
    
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([faction_name], |row| {
        Ok((
            row.get::<_, String>(0)?, // loyalty
            row.get::<_, i32>(1)?,    // count
        ))
    })?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    
    Ok(results)
}

// ================================
// GENERATED BY MACROS:
// ================================

// High-level (meta-driven):
// - CharacterFaction struct with Relation trait
// - parse_character_faction_string() function
// - process_character_faction_relations() function
// - CharacterFactionRelation struct for parsing

// Low-level (database):
// - CharacterFactionDb struct with CRUD methods
// - init_table(), insert(), update(), delete(), get_by_first_key()

// Usage examples:
// --set factions=rebels:soldier
// --set factions=thieves_guild:leader:loyal,merchants:member:neutral
// --set factions=empire:spy:conflicted