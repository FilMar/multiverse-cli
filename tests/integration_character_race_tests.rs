//! Integration tests for Character-Race relations
//! Tests the complete flow of creating and querying character-race relationships

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
fn test_character_race_basic_relation() {
    let world = TestWorld::new();
    
    // Create race first
    world.run_command_success(&[
        "race", "create", "elves", 
        "--set", "display_name=High Elves",
        "--set", "lifespan=3000"
    ]);
    
    // Create character with race relation
    let output = world.run_command_success(&[
        "character", "create", "legolas", 
        "--set", "display_name=Legolas Greenleaf",
        "--set", "race=elves*pureblooded"
    ]);
    
    // Should show relation was processed
    assert!(output.contains("🔗 Processed relations: race"));
    assert!(output.contains("✅ Created relation: legolas -> elves"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, r.name as race_name, cr.heritage FROM character_race_relations cr JOIN characters c ON cr.from_id = c.id JOIN races r ON cr.to_id = r.id"
    ]);
    
    assert!(query_output.contains("legolas"));
    assert!(query_output.contains("elves"));
    assert!(query_output.contains("pureblooded"));
}

#[test]
fn test_character_race_default_heritage() {
    let world = TestWorld::new();
    
    // Create race
    world.run_command_success(&[
        "race", "create", "humans", 
        "--set", "display_name=Humans",
        "--set", "lifespan=80"
    ]);
    
    // Create character without specifying heritage (should default to "standard")
    let output = world.run_command_success(&[
        "character", "create", "aragorn", 
        "--set", "display_name=Aragorn",
        "--set", "race=humans"
    ]);
    
    assert!(output.contains("✅ Created relation: aragorn -> humans"));
    
    // Query should show default heritage
    let query_output = world.run_command_success(&[
        "query",
        "SELECT cr.heritage FROM character_race_relations cr JOIN characters c ON cr.from_id = c.id WHERE c.name = 'aragorn'"
    ]);
    
    assert!(query_output.contains("standard"));
}

#[test]
fn test_character_race_mixed_heritage() {
    let world = TestWorld::new();
    
    // Create multiple races
    world.run_command_success(&["race", "create", "humans", "--set", "display_name=Humans"]);
    world.run_command_success(&["race", "create", "elves", "--set", "display_name=Elves"]);
    
    // Create character with mixed heritage (multiple race relations)
    let output = world.run_command_success(&[
        "character", "create", "elrond", 
        "--set", "display_name=Elrond Half-elven",
        "--set", "race=elves*half_blood,humans*half_blood"
    ]);
    
    // Should process relations
    assert!(output.contains("🔗 Processed relations: race"));
    
    // Query should show both racial heritages
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, r.name as race_name, cr.heritage FROM character_race_relations cr JOIN characters c ON cr.from_id = c.id JOIN races r ON cr.to_id = r.id WHERE c.name = 'elrond' ORDER BY r.name"
    ]);
    
    assert!(query_output.contains("half_blood"));
    assert!(query_output.contains("📊 2 row(s) returned"));
}

#[test]
fn test_character_race_validation() {
    let world = TestWorld::new();
    
    // Try to create character with non-existent race
    let error = world.run_command_expect_failure(&[
        "character", "create", "gimli", 
        "--set", "display_name=Gimli",
        "--set", "race=dwarves*noble"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Race not found: 'dwarves'"));
}

#[test]
fn test_character_race_update_relations() {
    let world = TestWorld::new();
    
    // Create races
    world.run_command_success(&["race", "create", "maiar", "--set", "display_name=Maiar"]);
    world.run_command_success(&["race", "create", "istari", "--set", "display_name=Istari"]);
    
    // Create character
    world.run_command_success(&[
        "character", "create", "gandalf", 
        "--set", "display_name=Gandalf"
    ]);
    
    // Add race relation via update
    world.run_command_success(&[
        "character", "update", "gandalf", 
        "--set", "race=maiar*divine"
    ]);
    
    // Later add specialized form
    let output = world.run_command_success(&[
        "character", "update", "gandalf", 
        "--set", "race=istari*incarnate"
    ]);
    
    assert!(output.contains("🔗 Processed relations: race"));
    
    // Query should show both racial forms
    let query_output = world.run_command_success(&[
        "query",
        "SELECT r.name as race_name, cr.heritage FROM character_race_relations cr JOIN characters c ON cr.from_id = c.id JOIN races r ON cr.to_id = r.id WHERE c.name = 'gandalf' ORDER BY r.name"
    ]);
    
    assert!(query_output.contains("divine"));
    assert!(query_output.contains("incarnate"));
}

#[test]
fn test_character_race_complex_heritage_analysis() {
    let world = TestWorld::new();
    
    // Create races with different traits
    world.run_command_success(&[
        "race", "create", "numenoreans", 
        "--set", "display_name=Númenóreans",
        "--set", "lifespan=300"
    ]);
    
    world.run_command_success(&[
        "character", "create", "isildur", 
        "--set", "display_name=Isildur",
        "--set", "race=numenoreans*royal_blood"
    ]);
    
    // Complex JOIN query to analyze racial characteristics
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as character_name, 
           r.display_name as race_name, 
           cr.heritage,
           r.metadata
         FROM characters c 
         JOIN character_race_relations cr ON c.id = cr.from_id 
         JOIN races r ON cr.to_id = r.id
         WHERE cr.heritage LIKE '%royal%'"
    ]);
    
    // Should show the noble heritage analysis
    assert!(query_output.contains("Isildur"));
    assert!(query_output.contains("Númenóreans"));
    assert!(query_output.contains("royal_blood"));
}

#[test]
fn test_character_multiple_relation_types() {
    let world = TestWorld::new();
    
    // Create entities for multiple relation types
    world.run_command_success(&["race", "create", "hobbits", "--set", "display_name=Hobbits"]);
    world.run_command_success(&["location", "create", "shire", "--set", "display_name=The Shire"]);
    world.run_command_success(&["faction", "create", "fellowship", "--set", "display_name=Fellowship"]);
    
    // Create character with multiple relation types
    let output = world.run_command_success(&[
        "character", "create", "frodo", 
        "--set", "display_name=Frodo Baggins",
        "--set", "race=hobbits*standard",
        "--set", "location=shire*resident", 
        "--set", "faction=fellowship*ring_bearer"
    ]);
    
    println!("Multiple relations output:\n{}", output);
    
    // Should process all relation types (order may vary)
    assert!(output.contains("🔗 Processed relations:"));
    assert!(output.contains("race"));
    assert!(output.contains("location"));  
    assert!(output.contains("faction"));
    
    // Verify each relation exists
    let race_query = world.run_command_success(&[
        "query", 
        "SELECT COUNT(*) as count FROM character_race_relations cr JOIN characters c ON cr.from_id = c.id WHERE c.name = 'frodo'"
    ]);
    assert!(race_query.contains("1"));
    
    let location_query = world.run_command_success(&[
        "query", 
        "SELECT COUNT(*) as count FROM character_location_relations WHERE from_id = 'frodo'"
    ]);
    assert!(location_query.contains("1"));
    
    let faction_query = world.run_command_success(&[
        "query", 
        "SELECT COUNT(*) as count FROM character_faction_relations WHERE from_id = 'frodo'"
    ]);
    assert!(faction_query.contains("1"));
}