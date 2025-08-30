mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_location_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationTest")?;
    
    // Create a location
    test.run_command_assert_success(&[
        "location", "create", "castle",
        "--set", "display_name=The Great Castle",
        "--set", "type=fortress",
        "--set", "description=A magnificent castle"
    ])?;
    
    // Verify location was created using query
    assert!(test.entity_exists("locations", "castle")?);
    
    let metadata = test.get_metadata("locations", "castle")?;
    assert!(metadata.contains("fortress"));
    assert!(metadata.contains("magnificent castle"));
    
    Ok(())
}

#[test]
fn test_location_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationRelationTest")?;
    
    // Create dependencies for location relations
    // Based on handlers.rs: Locations can relate to: faction, location (parent/child), system, character (reverse)
    test.run_command_assert_success(&[
        "faction", "create", "city_guard",
        "--set", "display_name=City Guard"
    ])?;
    
    test.run_command_assert_success(&[
        "location", "create", "kingdom",
        "--set", "display_name=The Kingdom"
    ])?;
    
    test.run_command_assert_success(&[
        "system", "create", "world_system",
        "--set", "display_name=World System"
    ])?;
    
    test.run_command_assert_success(&[
        "character", "create", "guard_captain",
        "--set", "display_name=Guard Captain"
    ])?;
    
    // Create location with relations
    test.run_command_assert_success(&[
        "location", "create", "city",
        "--set", "display_name=Capital City",
        "--set", "faction=city_guard*controls",
        "--set", "location=kingdom*part_of",
        "--set", "system=world_system*located_in"
    ])?;
    
    // Add reverse relation via update
    test.run_command_assert_success(&[
        "location", "update", "city",
        "--set", "character=guard_captain*stationed_at"
    ])?;
    
    // Verify location exists
    assert!(test.entity_exists("locations", "city")?);
    
    // Verify relations were created
    let lf_relations = test.query("SELECT COUNT(*) FROM location_faction_relations")?;
    println!("Location-faction relations: {}", lf_relations);
    
    let ll_relations = test.query("SELECT COUNT(*) FROM location_location_relations")?; 
    println!("Location-location relations: {}", ll_relations);
    
    let ls_relations = test.query("SELECT COUNT(*) FROM location_system_relations")?;
    println!("Location-system relations: {}", ls_relations);
    
    let cl_relations = test.query("SELECT COUNT(*) FROM character_location_relations")?;
    println!("Character-location relations: {}", cl_relations);
    
    assert!(test.query_count("location_faction_relations")? > 0);
    assert!(test.query_count("location_location_relations")? > 0);
    assert!(test.query_count("location_system_relations")? > 0);
    assert!(test.query_count("character_location_relations")? > 0); // reverse relation
    
    Ok(())
}

#[test]
fn test_location_bidirectional_relations() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationBidirectionalTest")?;
    
    // Create entities
    test.run_command_assert_success(&[
        "location", "create", "tavern",
        "--set", "display_name=The Prancing Pony"
    ])?;
    
    test.run_command_assert_success(&[
        "faction", "create", "innkeepers",
        "--set", "display_name=Innkeepers Guild"
    ])?;
    
    // Test bidirectional: faction can reference location (reverse relation)
    test.run_command_assert_success(&[
        "faction", "update", "innkeepers",
        "--set", "location=tavern*headquarters"
    ])?;
    
    // Verify relation exists
    assert!(test.query_count("location_faction_relations")? > 0);
    
    // Check the relation has correct data
    let relation_data = test.query("SELECT relationship_type FROM location_faction_relations")?;
    assert!(relation_data.contains("headquarters"));
    
    Ok(())
}

#[test]
fn test_location_hierarchy() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationHierarchyTest")?;
    
    // Create parent location
    test.run_command_assert_success(&[
        "location", "create", "continent",
        "--set", "display_name=The Great Continent"
    ])?;
    
    // Create child location that references parent
    test.run_command_assert_success(&[
        "location", "create", "country",
        "--set", "display_name=The Country",
        "--set", "location=continent*part_of"
    ])?;
    
    // Create grandchild location
    test.run_command_assert_success(&[
        "location", "create", "city",
        "--set", "display_name=Capital City",
        "--set", "location=country*capital_of"
    ])?;
    
    // Verify all locations exist
    assert!(test.entity_exists("locations", "continent")?);
    assert!(test.entity_exists("locations", "country")?);
    assert!(test.entity_exists("locations", "city")?);
    
    // Verify hierarchical relations
    assert!(test.query_count("location_location_relations")? >= 2);
    
    Ok(())
}

#[test]
fn test_location_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationListTest")?;
    
    // Create a location first
    test.run_command_assert_success(&[
        "location", "create", "test_location",
        "--set", "display_name=Test Location"
    ])?;
    
    // Test location list
    test.run_command_assert_success(&["location", "list"])?;
    
    // Test location info
    test.run_command_assert_success(&["location", "info", "test_location"])?;
    
    Ok(())
}

#[test]
fn test_location_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("LocationDeleteTest")?;
    
    // Create location
    test.run_command_assert_success(&[
        "location", "create", "temp_location",
        "--set", "display_name=Temporary Location"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("locations", "temp_location")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "location", "delete", "temp_location", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("locations", "temp_location")?);
    
    Ok(())
}