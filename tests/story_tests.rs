mod common;
use common::MultiverseTest;
use anyhow::Result;

#[test]
fn test_story_creation() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StoryTest")?;
    
    // Create a story
    test.run_command_assert_success(&[
        "story", "create", "my_adventure",
        "--set", "type=diary",
        "--set", "narrator=TestNarrator",
        "--set", "description=A test story"
    ])?;
    
    // Verify story was created using query
    assert!(test.entity_exists("stories", "my_adventure")?);
    
    let story_data = test.query("SELECT story_type FROM stories WHERE name = 'my_adventure'")?;
    assert!(story_data.contains("diary"));
    
    // Check metadata
    let metadata = test.get_metadata("stories", "my_adventure")?;
    assert!(metadata.contains("TestNarrator"));
    
    Ok(())
}

#[test]
fn test_story_types() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StoryTypesTest")?;
    
    // Test story types command
    test.run_command_assert_success(&["story", "types"])?;
    
    Ok(())
}

#[test]
fn test_story_list_and_info() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StoryListTest")?;
    
    // Create a story first
    test.run_command_assert_success(&[
        "story", "create", "test_story",
        "--set", "type=diary",
        "--set", "narrator=TestNarrator"
    ])?;
    
    // Test story list
    test.run_command_assert_success(&["story", "list"])?;
    
    // Test story info
    test.run_command_assert_success(&["story", "info", "test_story"])?;
    
    Ok(())
}

#[test]
fn test_story_update() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StoryUpdateTest")?;
    
    // Create story
    test.run_command_assert_success(&[
        "story", "create", "updatable_story",
        "--set", "type=diary", 
        "--set", "narrator=Original"
    ])?;
    
    // Update story
    test.run_command_assert_success(&[
        "story", "update", "updatable_story",
        "--set", "narrator=Updated",
        "--set", "mood=dark"
    ])?;
    
    // Verify update
    let metadata = test.get_metadata("stories", "updatable_story")?;
    assert!(metadata.contains("Updated"));
    assert!(metadata.contains("dark"));
    
    Ok(())
}

#[test]
fn test_story_delete() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("StoryDeleteTest")?;
    
    // Create story
    test.run_command_assert_success(&[
        "story", "create", "temp_story",
        "--set", "type=diary",
        "--set", "narrator=Temp"
    ])?;
    
    // Verify it exists
    assert!(test.entity_exists("stories", "temp_story")?);
    
    // Delete with force
    test.run_command_assert_success(&[
        "story", "delete", "temp_story", "--force"
    ])?;
    
    // Verify it's gone
    assert!(!test.entity_exists("stories", "temp_story")?);
    
    Ok(())
}

#[test]
fn test_book_story_type() -> Result<()> {
    let test = MultiverseTest::new()?;
    test.init_world("BookTest")?;
    
    // Create a book type story (requires author instead of narrator)
    test.run_command_assert_success(&[
        "story", "create", "my_novel",
        "--set", "type=book",
        "--set", "author=Test Author",
        "--set", "genre=fantasy"
    ])?;
    
    // Verify story was created
    assert!(test.entity_exists("stories", "my_novel")?);
    
    let story_data = test.query("SELECT story_type FROM stories WHERE name = 'my_novel'")?;
    assert!(story_data.contains("book"));
    
    let metadata = test.get_metadata("stories", "my_novel")?;
    assert!(metadata.contains("Test Author"));
    assert!(metadata.contains("fantasy"));
    
    Ok(())
}