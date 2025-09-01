mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_system_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemTest")?;
    
    // Create a system
    test.run_command_assert_success(&[
        "system", "create", "solar_system",
        "--set", "display_name=Our Solar System",
        "--set", "type=planetary",
        "--set", "description=A star system with planets"
    ])?;
    
    // Verify system was created using query
    assert!(test.entity_exists("systems", "solar_system")?);
    
    let metadata = test.get_metadata("systems", "solar_system")?;
    assert!(metadata.contains("planetary"));
    assert!(metadata.contains("star system"));
    
    Ok(())
}

#[test]
fn test_system_reverse_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemRelationTest")?;
    
    // Create entities that can reference systems
    // Based on handlers.rs: Systems can have reverse relations with: character, location, race
    test.run_command_assert_success(&[
        "character", "create", "spacer",
        "--set", "display_name=Space Explorer"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "planet",
        "--set", "display_name=Home Planet"
    ])?;
    
    test.run_command_assert_success(&[
        "race", "create", "aliens",
        "--set", "display_name=Alien Species"
    ])?;
    
    // Create system and use update to add reverse relations
    test.run_command_assert_success(&[
        "system", "create", "galaxy",
        "--set", "display_name=The Galaxy"
    ])?;
    
    // Update system with reverse relations
    test.run_command_assert_success(&[
        "system", "update", "galaxy",
        "--set", "character=spacer*inhabitant",
        "--set", "location=planet*contains",
        "--set", "race=aliens*native_to"
    ])?;
    
    // Verify system exists
    assert!(test.entity_exists("systems", "galaxy")?);
    
    // Verify reverse relations were created
    assert!(test.query_count("character_system_relations")? > 0);
    assert!(test.query_count("location_system_relations")? > 0);
    assert!(test.query_count("race_system_relations")? > 0);
    
    Ok(())
}

#[test]
fn test_system_bidirectional_with_location() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemLocationTest")?;
    
    // Create entities
    test.run_command_assert_success(&[
        "system", "create", "star_system",
        "--set", "display_name=Binary Star System"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "asteroid_belt",
        "--set", "display_name=The Asteroid Belt"
    ])?;
    
    // Test forward relation: location -> system
    test.run_command_assert_success(&[
        "location", "update", "asteroid_belt",
        "--set", "system=star_system*orbits"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("location_system_relations")? > 0);
    
    // Check the relation has correct data
    let relation_data = test.query("SELECT infrastructure_type FROM location_system_relations")?;
    assert!(relation_data.contains("orbits"));
    
    Ok(())
}

#[test]
fn test_system_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemListTest")?;
    
    // Create a system first
    test.run_command_assert_success(&[
        "system", "create", "test_system",
        "--set", "display_name=Test System"
    ])?;
    
    // Test system list
    test.run_command_assert_success(&["system", "list"])?;
    
    // Test system info
    test.run_command_assert_success(&["system", "info", "test_system"])?;
    
    Ok(())
}

#[test]
fn test_system_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemUpdateTest")?;
    
    // Create system
    test.run_command_assert_success(&[
        "system", "create", "updatable_system",
        "--set", "display_name=Original System",
        "--set", "type=simple"
    ])?;
    
    // Update system
    test.run_command_assert_success(&[
        "system", "update", "updatable_system",
        "--set", "type=complex",
        "--set", "danger_level=high"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("systems", "updatable_system")?;
    assert!(metadata.contains("complex"));
    assert!(metadata.contains("high"));
    
    Ok(())
}

#[test]
fn test_system_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("SystemDeleteTest")?;
    
    // Create system
    test.run_command_assert_success(&[
        "system", "create", "temp_system",
        "--set", "display_name=Temporary System"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("systems", "temp_system")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "system", "delete", "temp_system", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("systems", "temp_system")?);
    
    Ok(())
}