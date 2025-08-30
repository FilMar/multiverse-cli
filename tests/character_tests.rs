mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_character_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("CharacterTest")?;
    
    // Create a character
    test.run_command_assert_success(&[
        "character", "create", "hero",
        "--set", "display_name=The Hero",
        "--set", "age=25",
        "--set", "description=A brave hero"
    ])?;
    
    // Verify character was created using query
    assert!(test.entity_exists("characters", "hero")?);
    
    let metadata = test.get_metadata("characters", "hero")?;
    assert!(metadata.contains("25"));
    assert!(metadata.contains("brave hero"));
    
    // Test character list
    test.run_command_assert_success(&["character", "list"])?;
    
    // Test character info
    test.run_command_assert_success(&["character", "info", "hero"])?;
    
    Ok(())
}

#[test]
fn test_character_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("CharacterRelationTest")?;
    
    // Create dependencies for character relations
    test.run_command_assert_success(&[
        "location", "create", "hometown",
        "--set", "display_name=Hero's Hometown"
    ])?;
    
    test.run_command_assert_success(&[
        "faction", "create", "guild",
        "--set", "display_name=Adventurers Guild"
    ])?;
    
    test.run_command_assert_success(&[
        "race", "create", "human",
        "--set", "display_name=Human Race"
    ])?;
    
    test.run_command_assert_success(&[
        "system", "create", "world_system",
        "--set", "display_name=World System"
    ])?;
    
    // Create character with relations
    // Based on handlers.rs: Characters can relate to: episode, location, faction, race, system
    test.run_command_assert_success(&[
        "character", "create", "adventurer",
        "--set", "display_name=The Adventurer",
        "--set", "location=hometown*resident",
        "--set", "faction=guild*member", 
        "--set", "race=human*born",
        "--set", "system=world_system*inhabitant"
    ])?;
    
    // Verify character exists
    assert!(test.entity_exists("characters", "adventurer")?);
    
    // Verify relations were created
    assert!(test.query_count("character_location_relations")? > 0);
    assert!(test.query_count("character_faction_relations")? > 0);
    assert!(test.query_count("character_race_relations")? > 0);
    assert!(test.query_count("character_system_relations")? > 0);
    
    Ok(())
}

#[test]
fn test_character_bidirectional_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("CharacterBidirectionalTest")?;
    
    // Create entities
    test.run_command_assert_success(&[
        "character", "create", "hero",
        "--set", "display_name=The Hero"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "castle",
        "--set", "display_name=The Castle"
    ])?;
    
    // Test bidirectional: location can reference character (reverse relation)
    test.run_command_assert_success(&[
        "location", "update", "castle",
        "--set", "character=hero*lord"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("character_location_relations")? > 0);
    
    // Check the relation has correct data
    let relation_data = test.query("SELECT relationship_type FROM character_location_relations")?;
    assert!(relation_data.contains("lord"));
    
    Ok(())
}

#[test]
fn test_character_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("CharacterUpdateTest")?;
    
    // Create character
    test.run_command_assert_success(&[
        "character", "create", "hero",
        "--set", "display_name=The Hero",
        "--set", "age=20"
    ])?;
    
    // Update character
    test.run_command_assert_success(&[
        "character", "update", "hero",
        "--set", "age=25",
        "--set", "experience=veteran"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("characters", "hero")?;
    assert!(metadata.contains("25"));
    assert!(metadata.contains("veteran"));
    
    Ok(())
}

#[test]
fn test_character_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("CharacterDeleteTest")?;
    
    // Create character
    test.run_command_assert_success(&[
        "character", "create", "temp_hero",
        "--set", "display_name=Temporary Hero"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("characters", "temp_hero")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "character", "delete", "temp_hero", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("characters", "temp_hero")?);
    
    Ok(())
}