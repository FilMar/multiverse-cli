mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_event_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventTest")?;
    
    // Create an event
    test.run_command_assert_success(&[
        "event", "create", "great_battle",
        "--set", "display_name=The Great Battle",
        "--set", "type=military",
        "--set", "description=Epic battle between armies"
    ])?;
    
    // Verify event was created using query
    assert!(test.entity_exists("events", "great_battle")?);
    
    let metadata = test.get_metadata("events", "great_battle")?;
    assert!(metadata.contains("military"));
    assert!(metadata.contains("Epic battle"));
    
    Ok(())
}

#[test]
fn test_event_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventRelationTest")?;
    
    // Create dependencies for event relations
    // Based on handlers.rs: Events can relate to: character, location, faction
    test.run_command_assert_success(&[
        "character", "create", "hero",
        "--set", "display_name=The Hero"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "battlefield",
        "--set", "display_name=The Battlefield"
    ])?;
    
    test.run_command_assert_success(&[
        "faction", "create", "army",
        "--set", "display_name=Royal Army"
    ])?;
    
    // Create event with relations
    test.run_command_assert_success(&[
        "event", "create", "final_battle",
        "--set", "display_name=The Final Battle",
        "--set", "character=hero*protagonist",
        "--set", "location=battlefield*occurred_at",
        "--set", "faction=army*participant"
    ])?;
    
    // Verify event exists
    assert!(test.entity_exists("events", "final_battle")?);
    
    // Verify relations were created
    assert!(test.query_count("event_character_relations")? > 0);
    assert!(test.query_count("event_location_relations")? > 0);
    assert!(test.query_count("event_faction_relations")? > 0);
    
    Ok(())
}

#[test]
fn test_event_timeline_integration() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventTimelineTest")?;
    
    // Create event with timeline data
    test.run_command_assert_success(&[
        "event", "create", "coronation",
        "--set", "display_name=The Coronation",
        "--set", "date=1000-05-15",
        "--set", "importance=high"
    ])?;
    
    // Verify event exists
    assert!(test.entity_exists("events", "coronation")?);
    
    // Verify metadata includes timeline info
    let metadata = test.get_metadata("events", "coronation")?;
    assert!(metadata.contains("1000-05-15"));
    assert!(metadata.contains("high"));
    
    Ok(())
}

#[test]
fn test_event_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventListTest")?;
    
    // Create an event first
    test.run_command_assert_success(&[
        "event", "create", "test_event",
        "--set", "display_name=Test Event"
    ])?;
    
    // Test event list
    test.run_command_assert_success(&["event", "list"])?;
    
    // Test event info
    test.run_command_assert_success(&["event", "info", "test_event"])?;
    
    Ok(())
}

#[test]
fn test_event_timeline_command() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventTimelineCommandTest")?;
    
    // Create some events with dates
    test.run_command_assert_success(&[
        "event", "create", "birth",
        "--set", "display_name=Hero's Birth",
        "--set", "date=980-01-01"
    ])?;
    
    test.run_command_assert_success(&[
        "event", "create", "quest_start", 
        "--set", "display_name=Quest Begins",
        "--set", "date=1000-03-15"
    ])?;
    
    // Test timeline command
    test.run_command_assert_success(&["event", "timeline"])?;
    
    Ok(())
}

#[test]
fn test_event_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventUpdateTest")?;
    
    // Create event
    test.run_command_assert_success(&[
        "event", "create", "updatable_event",
        "--set", "display_name=Original Event",
        "--set", "importance=low"
    ])?;
    
    // Update event
    test.run_command_assert_success(&[
        "event", "update", "updatable_event",
        "--set", "importance=critical",
        "--set", "outcome=success"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("events", "updatable_event")?;
    assert!(metadata.contains("critical"));
    assert!(metadata.contains("success"));
    
    Ok(())
}

#[test]
fn test_event_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("EventDeleteTest")?;
    
    // Create event
    test.run_command_assert_success(&[
        "event", "create", "temp_event",
        "--set", "display_name=Temporary Event"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("events", "temp_event")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "event", "delete", "temp_event", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("events", "temp_event")?);
    
    Ok(())
}