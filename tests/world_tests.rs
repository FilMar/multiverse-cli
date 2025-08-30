mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_world_initialization() -> Result<()> {
    let test = MultiverseTest::new()?;
    
    // Initialize world
    test.init_world("TestWorld")?;
    
    // Verify world info works
    let output = test.run_command(&["info"])?;
    assert!(output.status.success());
    let info_output = String::from_utf8_lossy(&output.stdout);
    assert!(info_output.contains("TestWorld"));
    
    // Verify database tables exist using query
    let tables_result = test.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
    assert!(tables_result.contains("stories"));
    assert!(tables_result.contains("characters"));
    assert!(tables_result.contains("locations"));
    assert!(tables_result.contains("systems"));
    assert!(tables_result.contains("factions"));
    assert!(tables_result.contains("races"));
    assert!(tables_result.contains("events"));
    assert!(tables_result.contains("episodes"));
    
    // Check all relation tables exist
    assert!(tables_result.contains("character_location_relations"));
    assert!(tables_result.contains("character_faction_relations"));
    assert!(tables_result.contains("character_race_relations"));
    assert!(tables_result.contains("character_system_relations"));
    assert!(tables_result.contains("location_faction_relations"));
    assert!(tables_result.contains("location_system_relations"));
    assert!(tables_result.contains("event_character_relations"));
    assert!(tables_result.contains("event_location_relations"));
    assert!(tables_result.contains("event_faction_relations"));
    
    Ok(())
}

#[test]
fn test_world_status() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StatusTest")?;
    
    let output = test.run_command_assert_success(&["world", "status"])?;
    let status_output = String::from_utf8_lossy(&output.stdout);
    assert!(status_output.contains("StatusTest"));
    
    Ok(())
}