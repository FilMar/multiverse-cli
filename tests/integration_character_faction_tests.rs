//! Integration tests for Character-Faction relations
//! Tests the complete flow of creating and querying character-faction relationships

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
fn test_character_faction_basic_relation() {
    let world = TestWorld::new();
    
    // Create faction first
    world.run_command_success(&[
        "faction", "create", "fellowship", 
        "--set", "display_name=Fellowship of the Ring"
    ]);
    
    // Create character with faction relation
    let output = world.run_command_success(&[
        "character", "create", "aragorn", 
        "--set", "display_name=Aragorn",
        "--set", "faction=fellowship*leader"
    ]);
    
    // Should show relation was processed
    assert!(output.contains("🔗 Processed relations: faction"));
    assert!(output.contains("✅ Created relation: aragorn -> fellowship"));
    
    // Query the relation using JOIN to show names
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, f.name as faction_name, r.role 
         FROM character_faction_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN factions f ON r.to_id = f.id"
    ]);
    
    assert!(query_output.contains("aragorn"));
    assert!(query_output.contains("fellowship"));
    assert!(query_output.contains("leader"));
}

#[test]
fn test_character_faction_multiple_affiliations() {
    let world = TestWorld::new();
    
    // Create multiple factions
    world.run_command_success(&["faction", "create", "rangers", "--set", "display_name=Rangers of the North"]);
    world.run_command_success(&["faction", "create", "gondor", "--set", "display_name=Kingdom of Gondor"]);
    
    // Create character with multiple faction relations
    let output = world.run_command_success(&[
        "character", "create", "boromir", 
        "--set", "display_name=Boromir of Gondor",
        "--set", "faction=gondor*captain,rangers*ally"
    ]);
    
    // Should process relations
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Query should show both relations using JOIN
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, f.name as faction_name, r.role 
         FROM character_faction_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN factions f ON r.to_id = f.id 
         WHERE c.name = 'boromir' ORDER BY f.name"
    ]);
    
    assert!(query_output.contains("captain"));
    assert!(query_output.contains("ally"));
    assert!(query_output.contains("📊 2 row(s) returned"));
}

#[test]
fn test_character_faction_default_role() {
    let world = TestWorld::new();
    
    // Create faction
    world.run_command_success(&["faction", "create", "hobbits", "--set", "display_name=The Hobbits"]);
    
    // Create character without specifying role (should default to "member")
    let output = world.run_command_success(&[
        "character", "create", "bilbo", 
        "--set", "display_name=Bilbo Baggins",
        "--set", "faction=hobbits"
    ]);
    
    assert!(output.contains("✅ Created relation: bilbo -> hobbits"));
    
    // Query should show default role using JOIN
    let query_output = world.run_command_success(&[
        "query",
        "SELECT r.role 
         FROM character_faction_relations r 
         JOIN characters c ON r.from_id = c.id 
         WHERE c.name = 'bilbo'"
    ]);
    
    assert!(query_output.contains("member"));
}

#[test]
fn test_character_faction_validation() {
    let world = TestWorld::new();
    
    // Try to create character with non-existent faction
    let error = world.run_command_expect_failure(&[
        "character", "create", "legolas", 
        "--set", "display_name=Legolas",
        "--set", "faction=elves*prince"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Faction not found: 'elves'"));
}

#[test]
fn test_character_faction_update_relations() {
    let world = TestWorld::new();
    
    // Create factions
    world.run_command_success(&["faction", "create", "isengard", "--set", "display_name=Isengard"]);
    world.run_command_success(&["faction", "create", "white_council", "--set", "display_name=White Council"]);
    
    // Create character
    world.run_command_success(&[
        "character", "create", "saruman", 
        "--set", "display_name=Saruman the White"
    ]);
    
    // Add faction relation via update (initially good)
    world.run_command_success(&[
        "character", "update", "saruman", 
        "--set", "faction=white_council*member"
    ]);
    
    // Later update to show corruption (different faction)
    let output = world.run_command_success(&[
        "character", "update", "saruman", 
        "--set", "faction=isengard*lord"
    ]);
    
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Query should show the new relation using JOIN
    let query_output = world.run_command_success(&[
        "query",
        "SELECT f.name as faction_name, r.role 
         FROM character_faction_relations r 
         JOIN characters c ON r.from_id = c.id 
         JOIN factions f ON r.to_id = f.id 
         WHERE c.name = 'saruman' ORDER BY f.name"
    ]);
    
    // Should show both affiliations (old and new)
    assert!(query_output.contains("isengard"));
    assert!(query_output.contains("lord"));
}

#[test]
fn test_character_faction_complex_join() {
    let world = TestWorld::new();
    
    // Create test data representing conflict
    world.run_command_success(&["faction", "create", "rohan", "--set", "display_name=Kingdom of Rohan"]);
    world.run_command_success(&[
        "character", "create", "theoden", 
        "--set", "display_name=Théoden King",
        "--set", "faction=rohan*king"
    ]);
    
    // Complex JOIN query to analyze faction leadership
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as character_name, 
           f.display_name as faction_name, 
           r.role 
         FROM characters c 
         JOIN character_faction_relations r ON c.id = r.from_id 
         JOIN factions f ON f.id = r.to_id
         WHERE r.role IN ('king', 'lord', 'leader')"
    ]);
    
    // Should show the leadership data
    assert!(query_output.contains("Théoden King"));
    assert!(query_output.contains("Kingdom of Rohan"));
    assert!(query_output.contains("king"));
}

#[test]
fn test_character_faction_mixed_relations() {
    let world = TestWorld::new();
    
    // Create location and faction
    world.run_command_success(&["location", "create", "minas_tirith", "--set", "display_name=Minas Tirith"]);
    world.run_command_success(&["faction", "create", "gondor_army", "--set", "display_name=Army of Gondor"]);
    
    // Create character with BOTH location and faction relations
    let output = world.run_command_success(&[
        "character", "create", "denethor", 
        "--set", "display_name=Denethor II",
        "--set", "location=minas_tirith*ruler",
        "--set", "faction=gondor_army*commander"
    ]);
    
    // Should process both types of relations (order doesn't matter)
    assert!(output.contains("🔗 Processed relations:") && 
            output.contains("location") && output.contains("faction"));
    
    // Query both relations to verify they exist
    let location_query = world.run_command_success(&[
        "query", 
        "SELECT COUNT(*) as count FROM character_location_relations WHERE from_id = 'denethor'"
    ]);
    assert!(location_query.contains("1"));
    
    let faction_query = world.run_command_success(&[
        "query", 
        "SELECT COUNT(*) as count FROM character_faction_relations WHERE from_id = 'denethor'"
    ]);
    assert!(faction_query.contains("1"));
}