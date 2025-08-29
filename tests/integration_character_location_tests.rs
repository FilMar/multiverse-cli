//! Integration tests for Character-Location relations
//! Tests the complete flow of creating and querying character-location relationships

use std::process::Command;
use tempfile::TempDir;

// Helper struct to manage test world
struct TestWorld {
    #[allow(dead_code)]
    temp_dir: TempDir,
    world_path: std::path::PathBuf,
}

impl TestWorld {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let world_path = temp_dir.path().to_path_buf();
        
        // Get path to multiverse binary
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        // Initialize world
        let output = Command::new(&multiverse_bin)
            .args(&["world", "init", "test-world"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to run world init");
            
        if !output.status.success() {
            panic!("World init failed: {}\nStdout: {}", 
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        
        Self { temp_dir, world_path }
    }
    
    fn run_command_success(&self, args: &[&str]) -> String {
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        let output = Command::new(&multiverse_bin)
            .args(args)
            .current_dir(&self.world_path)
            .output()
            .expect("Failed to run command");
        
        if !output.status.success() {
            panic!("Command failed: {:?}\nStderr: {}\nStdout: {}", 
                args,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        
        String::from_utf8_lossy(&output.stdout).to_string()
    }
    
    fn run_command_expect_failure(&self, args: &[&str]) -> String {
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        let output = Command::new(&multiverse_bin)
            .args(args)
            .current_dir(&self.world_path)
            .output()
            .expect("Failed to run command");
        
        // This should fail
        assert!(!output.status.success(), "Command should have failed but succeeded");
        
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

#[test]
fn test_character_location_basic_relation() {
    let world = TestWorld::new();
    
    // Create location first
    world.run_command_success(&[
        "location", "create", "tavern", 
        "--set", "display_name=The Prancing Pony"
    ]);
    
    // Create character with location relation
    let output = world.run_command_success(&[
        "character", "create", "barliman", 
        "--set", "display_name=Barliman Butterbur",
        "--set", "location=tavern*owner"
    ]);
    
    println!("Character create output:\n{}", output);
    
    // Should show relation was processed
    assert!(output.contains("🔗 Processed relations: location"));
    assert!(output.contains("✅ Created relation: barliman -> tavern"));
    
    // Query the relation using JOIN to show names instead of IDs
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, l.name as location_name, r.relationship_type 
         FROM character_location_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN locations l ON r.to_id = l.id"
    ]);
    
    assert!(query_output.contains("barliman"));
    assert!(query_output.contains("tavern"));
    assert!(query_output.contains("owner"));
}

#[test]
fn test_character_location_multiple_relations() {
    let world = TestWorld::new();
    
    // Create multiple locations
    world.run_command_success(&["location", "create", "shire", "--set", "display_name=The Shire"]);
    world.run_command_success(&["location", "create", "rivendell", "--set", "display_name=Rivendell"]);
    
    // Create character with multiple location relations
    let output = world.run_command_success(&[
        "character", "create", "frodo", 
        "--set", "display_name=Frodo Baggins",
        "--set", "location=shire*resident,rivendell*visitor"
    ]);
    
    // Should process relations
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query should show both relations using JOIN
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, l.name as location_name, r.relationship_type 
         FROM character_location_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN locations l ON r.to_id = l.id 
         WHERE c.name = 'frodo' ORDER BY l.name"
    ]);
    
    assert!(query_output.contains("resident"));
    assert!(query_output.contains("visitor"));
    assert!(query_output.contains("📊 2 row(s) returned"));
}

#[test]
fn test_character_location_validation() {
    let world = TestWorld::new();
    
    // Try to create character with non-existent location
    let error = world.run_command_expect_failure(&[
        "character", "create", "gandalf", 
        "--set", "display_name=Gandalf the Grey",
        "--set", "location=nonexistent*visitor"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Location not found: 'nonexistent'"));
}

#[test]
fn test_character_location_update_relations() {
    let world = TestWorld::new();
    
    // Create locations
    world.run_command_success(&["location", "create", "moria", "--set", "display_name=Mines of Moria"]);
    world.run_command_success(&["location", "create", "lothlórien", "--set", "display_name=Lothlórien"]);
    
    // Create character
    world.run_command_success(&[
        "character", "create", "gimli", 
        "--set", "display_name=Gimli the Dwarf"
    ]);
    
    // Add location relation via update
    let output = world.run_command_success(&[
        "character", "update", "gimli", 
        "--set", "location=moria*heir"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query should show the relation using JOIN
    let query_output = world.run_command_success(&[
        "query",
        "SELECT r.relationship_type 
         FROM character_location_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN locations l ON r.to_id = l.id 
         WHERE c.name = 'gimli' AND l.name = 'moria'"
    ]);
    
    assert!(query_output.contains("heir"));
}

#[test]
fn test_character_location_join_query() {
    let world = TestWorld::new();
    
    // Create test data
    world.run_command_success(&["location", "create", "isengard", "--set", "display_name=Isengard"]);
    world.run_command_success(&[
        "character", "create", "saruman", 
        "--set", "display_name=Saruman the White",
        "--set", "location=isengard*ruler"
    ]);
    
    // Complex JOIN query to test data integrity
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.display_name as character, l.display_name as location, r.relationship_type 
         FROM characters c 
         JOIN character_location_relations r ON c.id = r.from_id 
         JOIN locations l ON l.id = r.to_id"
    ]);
    
    // Should show joined data with proper display names
    assert!(query_output.contains("Saruman the White"));
    assert!(query_output.contains("Isengard"));
    assert!(query_output.contains("ruler"));
}