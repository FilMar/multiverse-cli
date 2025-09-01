mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_basic_query() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryTest")?;
    
    // Create some test data
    test.run_command_assert_success(&[
        "character", "create", "alice",
        "--set", "display_name=Alice Smith"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "bob", 
        "--set", "display_name=Bob Jones"
    ])?;
    
    // Test basic count query
    assert_eq!(test.query_count("characters")?, 2);
    
    // Test more complex query
    let result = test.query("SELECT * FROM characters ORDER BY name")?;
    assert!(result.contains("alice"));
    assert!(result.contains("Alice Smith"));
    assert!(result.contains("bob"));
    assert!(result.contains("Bob Jones"));
    
    Ok(())
}

#[test]
fn test_query_with_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryRelationTest")?;
    
    // Create entities with relations
    test.run_command_assert_success(&[
        "location", "create", "castle",
        "--set", "display_name=The Castle"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "knight",
        "--set", "display_name=Sir Knight",
        "--set", "location=castle*resident"
    ])?;
    
    // Query the relation data
    let relation_result = test.query(
        "SELECT * FROM character_location_relations cr 
         JOIN characters c ON cr.from_id = c.id 
         JOIN locations l ON cr.to_id = l.id"
    )?;
    
    
    assert!(relation_result.contains("knight"));
    assert!(relation_result.contains("castle"));
    assert!(relation_result.contains("resident"));
    
    Ok(())
}

#[test]
fn test_query_metadata_json() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryMetadataTest")?;
    
    // Create entity with rich metadata
    test.run_command_assert_success(&[
        "character", "create", "wizard",
        "--set", "display_name=Grand Wizard",
        "--set", "age=150",
        "--set", "school=evocation",
        "--set", "power_level=9000"
    ])?;
    
    // Query specific metadata fields using JSON functions
    let age_result = test.query("SELECT json_extract(metadata, '$.age') FROM characters WHERE name = 'wizard'")?;
    assert!(age_result.contains("150"));
    
    // Query multiple metadata fields
    let metadata_result = test.query("SELECT * FROM characters WHERE name = 'wizard'")?;
    assert!(metadata_result.contains("evocation"));
    assert!(metadata_result.contains("9000"));
    
    Ok(())
}

#[test] 
fn test_query_complex_joins() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryComplexTest")?;
    
    // Create a complex scenario
    test.run_command_assert_success(&[
        "story", "create", "epic_tale",
        "--set", "type=diary",
        "--set", "narrator=Hero"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "protagonist",
        "--set", "display_name=Main Character"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "hometown",
        "--set", "display_name=Starting Town"
    ])?;
    
    test.run_command_assert_success(&[
        "faction", "create", "allies",
        "--set", "display_name=Allied Forces"
    ])?;
    
    test.run_command_assert_success(&[
        "event", "create", "departure",
        "--set", "display_name=The Departure",
        "--set", "character=protagonist*main",
        "--set", "location=hometown*from",
        "--set", "faction=allies*with"
    ])?;
    
    // Complex query joining multiple tables through relations
    let complex_result = test.query(
        "SELECT * FROM events e
         LEFT JOIN event_character_relations ecr ON e.id = ecr.from_id
         LEFT JOIN characters c ON ecr.to_id = c.id
         LEFT JOIN event_location_relations elr ON e.id = elr.from_id  
         LEFT JOIN locations l ON elr.to_id = l.id
         LEFT JOIN event_faction_relations efr ON e.id = efr.from_id
         LEFT JOIN factions f ON efr.to_id = f.id
         WHERE e.name = 'departure'"
    )?;
    
    assert!(complex_result.contains("The Departure"));
    assert!(complex_result.contains("Main Character"));
    assert!(complex_result.contains("Starting Town"));
    assert!(complex_result.contains("Allied Forces"));
    
    Ok(())
}

#[test]
fn test_query_aggregations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryAggregationTest")?;
    
    // Create multiple entities of different types
    for i in 1..=3 {
        test.run_command_assert_success(&[
            "character", "create", &format!("char{}", i),
            "--set", &format!("display_name=Character {}", i)
        ])?;
    }
    
    for i in 1..=2 {
        test.run_command_assert_success(&[
            "location", "create", &format!("loc{}", i),
            "--set", &format!("display_name=Location {}", i)
        ])?;
    }
    
    // Test aggregation queries
    let char_count = test.query("SELECT COUNT(*) as character_count FROM characters")?;
    assert!(char_count.contains("3"));
    
    let loc_count = test.query("SELECT COUNT(*) as location_count FROM locations")?;
    assert!(loc_count.contains("2"));
    
    // Test union query
    let total_entities = test.query(
        "SELECT 'characters' as type, COUNT(*) as count FROM characters
         UNION ALL
         SELECT 'locations' as type, COUNT(*) as count FROM locations
         ORDER BY count DESC"
    )?;
    
    assert!(total_entities.contains("characters"));
    assert!(total_entities.contains("locations"));
    
    Ok(())
}

#[test]
fn test_query_security() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QuerySecurityTest")?;
    
    // Create test data
    test.run_command_assert_success(&[
        "character", "create", "test_char",
        "--set", "display_name=Test Character"
    ])?;
    
    // Test that SELECT queries work
    let select_result = test.query("SELECT name FROM characters")?;
    assert!(select_result.contains("test_char"));
    
    // Note: The actual SQL injection protection should be tested at the CLI level
    // The query system should reject non-SELECT statements
    // This would be implementation dependent based on how the query system works
    
    Ok(())
}

#[test]
fn test_query_table_structure() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("QueryStructureTest")?;
    
    // Test querying database schema
    let tables = test.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
    
    // Verify all expected tables exist
    assert!(tables.contains("characters"));
    assert!(tables.contains("stories"));
    assert!(tables.contains("episodes"));
    assert!(tables.contains("locations"));
    assert!(tables.contains("events"));
    assert!(tables.contains("factions"));
    assert!(tables.contains("systems"));
    assert!(tables.contains("races"));
    
    // Verify relation tables exist
    assert!(tables.contains("character_location_relations"));
    assert!(tables.contains("character_faction_relations"));
    assert!(tables.contains("event_character_relations"));
    
    // Test that we can query the characters table (verifies structure implicitly)
    test.run_command_assert_success(&[
        "character", "create", "test_char",
        "--set", "display_name=Test Character"
    ])?;
    
    // Verify we can query the created character (tests table structure implicitly)
    let char_data = test.query("SELECT * FROM characters WHERE name = 'test_char'")?;
    assert!(char_data.contains("test_char"));
    assert!(char_data.contains("Test Character"));
    assert!(char_data.contains("Active"));
    
    Ok(())
}
