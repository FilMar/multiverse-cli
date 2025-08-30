mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_faction_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionTest")?;
    
    // Create a faction
    test.run_command_assert_success(&[
        "faction", "create", "knights",
        "--set", "display_name=Knights of Honor",
        "--set", "type=military",
        "--set", "description=Noble knights"
    ])?;
    
    // Verify faction was created using query
    assert!(test.entity_exists("factions", "knights")?);
    
    let metadata = test.get_metadata("factions", "knights")?;
    assert!(metadata.contains("military"));
    assert!(metadata.contains("Noble knights"));
    
    Ok(())
}

#[test]
fn test_faction_reverse_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionRelationTest")?;
    
    // Create entities that can reference factions
    // Based on handlers.rs: Factions can have reverse relations with: character, location, event
    test.run_command_assert_success(&[
        "character", "create", "knight",
        "--set", "display_name=Sir Knight"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "fortress",
        "--set", "display_name=The Fortress"
    ])?;
    
    test.run_command_assert_success(&[
        "event", "create", "war",
        "--set", "display_name=The Great War"
    ])?;
    
    // Create faction and use update to add reverse relations
    test.run_command_assert_success(&[
        "faction", "create", "army",
        "--set", "display_name=The Army"
    ])?;
    
    // Update faction with reverse relations
    test.run_command_assert_success(&[
        "faction", "update", "army",
        "--set", "character=knight*member",
        "--set", "location=fortress*controls",
        "--set", "event=war*participant"
    ])?;
    
    // Verify faction exists
    assert!(test.entity_exists("factions", "army")?);
    
    // Verify reverse relations were created
    assert!(test.query_count("character_faction_relations")? > 0);
    assert!(test.query_count("location_faction_relations")? > 0);
    assert!(test.query_count("event_faction_relations")? > 0);
    
    Ok(())
}

#[test]
fn test_faction_bidirectional_with_character() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionCharacterTest")?;
    
    // Create entities
    test.run_command_assert_success(&[
        "faction", "create", "guild",
        "--set", "display_name=Mages Guild"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "mage",
        "--set", "display_name=Master Mage"
    ])?;
    
    // Test forward relation: character -> faction
    test.run_command_assert_success(&[
        "character", "update", "mage",
        "--set", "faction=guild*guildmaster"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("character_faction_relations")? > 0);
    
    // Check the relation has correct data
    let relation_data = test.query("SELECT role FROM character_faction_relations")?;
    assert!(relation_data.contains("guildmaster"));
    
    Ok(())
}

#[test]
fn test_faction_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionListTest")?;
    
    // Create a faction first
    test.run_command_assert_success(&[
        "faction", "create", "test_faction",
        "--set", "display_name=Test Faction"
    ])?;
    
    // Test faction list
    test.run_command_assert_success(&["faction", "list"])?;
    
    // Test faction info
    test.run_command_assert_success(&["faction", "info", "test_faction"])?;
    
    Ok(())
}

#[test]
fn test_faction_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionUpdateTest")?;
    
    // Create faction
    test.run_command_assert_success(&[
        "faction", "create", "updatable_faction",
        "--set", "display_name=Original Name",
        "--set", "type=neutral"
    ])?;
    
    // Update faction
    test.run_command_assert_success(&[
        "faction", "update", "updatable_faction",
        "--set", "type=aggressive",
        "--set", "motto=Victory or Death"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("factions", "updatable_faction")?;
    assert!(metadata.contains("aggressive"));
    assert!(metadata.contains("Victory or Death"));
    
    Ok(())
}

#[test]
fn test_faction_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("FactionDeleteTest")?;
    
    // Create faction
    test.run_command_assert_success(&[
        "faction", "create", "temp_faction",
        "--set", "display_name=Temporary Faction"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("factions", "temp_faction")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "faction", "delete", "temp_faction", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("factions", "temp_faction")?);
    
    Ok(())
}