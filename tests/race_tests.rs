mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_race_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceTest")?;
    
    // Create a race
    test.run_command_assert_success(&[
        "race", "create", "elves",
        "--set", "display_name=High Elves",
        "--set", "type=humanoid",
        "--set", "description=Ancient magical beings"
    ])?;
    
    // Verify race was created using query
    assert!(test.entity_exists("races", "elves")?);
    
    let metadata = test.get_metadata("races", "elves")?;
    assert!(metadata.contains("humanoid"));
    assert!(metadata.contains("Ancient magical beings"));
    
    Ok(())
}

#[test]
fn test_race_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceRelationTest")?;
    
    // Create dependencies for race relations
    // Based on handlers.rs: Races can relate to: system (forward), character (reverse)
    test.run_command_assert_success(&[
        "system", "create", "homeworld_system",
        "--set", "display_name=Elven Homeworld System"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "elf_lord",
        "--set", "display_name=Elven Lord"
    ])?;
    
    // Create race with system relation
    test.run_command_assert_success(&[
        "race", "create", "forest_elves",
        "--set", "display_name=Forest Elves",
        "--set", "system=homeworld_system*native_to"
    ])?;
    
    // Add reverse relation via character
    test.run_command_assert_success(&[
        "character", "update", "elf_lord",
        "--set", "race=forest_elves*member"
    ])?;
    
    // Verify race exists
    assert!(test.entity_exists("races", "forest_elves")?);
    
    // Verify relations were created
    assert!(test.query_count("race_system_relations")? > 0);
    assert!(test.query_count("character_race_relations")? > 0);
    
    Ok(())
}

#[test]
fn test_race_bidirectional_with_character() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceCharacterTest")?;
    
    // Create entities
    test.run_command_assert_success(&[
        "race", "create", "dwarves",
        "--set", "display_name=Mountain Dwarves"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "dwarf_warrior",
        "--set", "display_name=Dwarf Warrior"
    ])?;
    
    // Test forward relation: character -> race
    test.run_command_assert_success(&[
        "character", "update", "dwarf_warrior",
        "--set", "race=dwarves*born_as"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("character_race_relations")? > 0);
    
    // Test reverse relation: race -> character  
    test.run_command_assert_success(&[
        "race", "update", "dwarves",
        "--set", "character=dwarf_warrior*champion"
    ])?;
    
    // Verify we now have more relations (bidirectional)
    assert!(test.query_count("character_race_relations")? >= 2);
    
    Ok(())
}

#[test]
fn test_race_system_relationship() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceSystemTest")?;
    
    // Create system
    test.run_command_assert_success(&[
        "system", "create", "dragon_realm",
        "--set", "display_name=Dragon Realm System"
    ])?;
    
    // Create race with system relation
    test.run_command_assert_success(&[
        "race", "create", "dragons",
        "--set", "display_name=Ancient Dragons",
        "--set", "system=dragon_realm*rules"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("race_system_relations")? > 0);
    
    // Check the relation has correct data
    let relation_data = test.query("SELECT relationship_type FROM race_system_relations")?;
    assert!(relation_data.contains("rules"));
    
    Ok(())
}

#[test]
fn test_race_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceListTest")?;
    
    // Create a race first
    test.run_command_assert_success(&[
        "race", "create", "test_race",
        "--set", "display_name=Test Race"
    ])?;
    
    // Test race list
    test.run_command_assert_success(&["race", "list"])?;
    
    // Test race info
    test.run_command_assert_success(&["race", "info", "test_race"])?;
    
    Ok(())
}

#[test]
fn test_race_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceUpdateTest")?;
    
    // Create race
    test.run_command_assert_success(&[
        "race", "create", "updatable_race",
        "--set", "display_name=Original Race",
        "--set", "lifespan=short"
    ])?;
    
    // Update race
    test.run_command_assert_success(&[
        "race", "update", "updatable_race",
        "--set", "lifespan=immortal",
        "--set", "abilities=magic"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("races", "updatable_race")?;
    assert!(metadata.contains("immortal"));
    assert!(metadata.contains("magic"));
    
    Ok(())
}

#[test]
fn test_race_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("RaceDeleteTest")?;
    
    // Create race
    test.run_command_assert_success(&[
        "race", "create", "temp_race",
        "--set", "display_name=Temporary Race"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("races", "temp_race")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "race", "delete", "temp_race", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("races", "temp_race")?);
    
    Ok(())
}