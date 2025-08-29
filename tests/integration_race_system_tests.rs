//! Integration tests for Race-System relations
//! Tests the complete flow of creating and querying race-system relationships

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
fn test_race_system_basic_relation() {
    let world = TestWorld::new();
    
    // Create system first
    world.run_command_success(&[
        "system", "create", "elemental_magic", 
        "--set", "display_name=Elemental Magic"
    ]);
    
    // Create race with system relation
    let output = world.run_command_success(&[
        "race", "create", "elves", 
        "--set", "display_name=High Elves",
        "--set", "system=elemental_magic*natural"
    ]);
    
    // Should show relation was processed
    assert!(output.contains("🔗 Processed relations: system"));
    assert!(output.contains("✅ Created relation: elves -> elemental_magic"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, affinity FROM race_system_relations"
    ]);
    
    assert!(query_output.contains("elves"));
    assert!(query_output.contains("elemental_magic"));
    assert!(query_output.contains("natural"));
}

#[test]
fn test_race_system_default_affinity() {
    let world = TestWorld::new();
    
    // Create system
    world.run_command_success(&[
        "system", "create", "forge_craft", 
        "--set", "display_name=Forge Craft"
    ]);
    
    // Create race without specifying affinity (should default to "compatible")
    let output = world.run_command_success(&[
        "race", "create", "dwarves", 
        "--set", "display_name=Mountain Dwarves",
        "--set", "system=forge_craft"
    ]);
    
    assert!(output.contains("✅ Created relation: dwarves -> forge_craft"));
    
    // Query should show default affinity
    let query_output = world.run_command_success(&[
        "query",
        "SELECT affinity FROM race_system_relations WHERE from_id = 'dwarves'"
    ]);
    
    assert!(query_output.contains("compatible"));
}

#[test]
fn test_race_system_multiple_affinities() {
    let world = TestWorld::new();
    
    // Create multiple systems
    world.run_command_success(&["system", "create", "arcane_magic", "--set", "display_name=Arcane Magic"]);
    world.run_command_success(&["system", "create", "divine_magic", "--set", "display_name=Divine Magic"]);
    
    // Create race with multiple system affinities
    let output = world.run_command_success(&[
        "race", "create", "humans", 
        "--set", "display_name=Humans",
        "--set", "system=arcane_magic*adaptable,divine_magic*faithful"
    ]);
    
    // Should process relations
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query should show both affinities
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, affinity FROM race_system_relations WHERE from_id = 'humans' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("adaptable"));
    assert!(query_output.contains("faithful"));
    assert!(query_output.contains("📊 2 row(s) returned"));
}

#[test]
fn test_race_system_validation() {
    let world = TestWorld::new();
    
    // Try to create race with non-existent system
    let error = world.run_command_expect_failure(&[
        "race", "create", "orcs", 
        "--set", "display_name=Orcs",
        "--set", "system=dark_magic*corrupted"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("System 'dark_magic' does not exist"));
    assert!(error.contains("multiverse system create dark_magic"));
}

#[test]
fn test_race_system_update_relations() {
    let world = TestWorld::new();
    
    // Create systems
    world.run_command_success(&["system", "create", "nature_magic", "--set", "display_name=Nature Magic"]);
    world.run_command_success(&["system", "create", "wild_magic", "--set", "display_name=Wild Magic"]);
    
    // Create race first
    world.run_command_success(&[
        "race", "create", "wood_elves", 
        "--set", "display_name=Wood Elves"
    ]);
    
    // Add system relation via update
    world.run_command_success(&[
        "race", "update", "wood_elves", 
        "--set", "system=nature_magic*innate"
    ]);
    
    // Later add another system affinity
    let output = world.run_command_success(&[
        "race", "update", "wood_elves", 
        "--set", "system=wild_magic*chaotic"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query should show both affinities
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, affinity FROM race_system_relations WHERE from_id = 'wood_elves' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("innate"));
    assert!(query_output.contains("chaotic"));
}

#[test]
fn test_race_system_affinity_analysis() {
    let world = TestWorld::new();
    
    // Create system with specific properties
    world.run_command_success(&[
        "system", "create", "time_magic", 
        "--set", "display_name=Temporal Magic",
        "--set", "complexity=extreme"
    ]);
    
    world.run_command_success(&[
        "race", "create", "time_weavers", 
        "--set", "display_name=Time Weavers",
        "--set", "lifespan=immortal",
        "--set", "system=time_magic*mastery"
    ]);
    
    // Complex JOIN query to analyze race-system compatibility
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           r.display_name as race_name, 
           s.display_name as system_name, 
           rs.affinity,
           s.metadata
         FROM races r 
         JOIN race_system_relations rs ON r.name = rs.from_id 
         JOIN systems s ON rs.to_id = s.name
         WHERE rs.affinity IN ('mastery', 'natural', 'innate')"
    ]);
    
    // Should show the mastery analysis
    assert!(query_output.contains("Time Weavers"));
    assert!(query_output.contains("Temporal Magic"));
    assert!(query_output.contains("mastery"));
}

#[test]
fn test_race_system_worldbuilding_scenario() {
    let world = TestWorld::new();
    
    // Create comprehensive worldbuilding scenario
    world.run_command_success(&["system", "create", "fire_magic", "--set", "display_name=Fire Magic"]);
    world.run_command_success(&["system", "create", "ice_magic", "--set", "display_name=Ice Magic"]);
    world.run_command_success(&["system", "create", "technology", "--set", "display_name=Advanced Technology"]);
    
    // Create races with opposing and complementary affinities
    world.run_command_success(&[
        "race", "create", "fire_giants", 
        "--set", "display_name=Fire Giants",
        "--set", "system=fire_magic*innate,ice_magic*vulnerable,technology*incompatible"
    ]);
    
    world.run_command_success(&[
        "race", "create", "tech_humans", 
        "--set", "display_name=Tech-enhanced Humans",
        "--set", "system=technology*enhanced,fire_magic*tools,ice_magic*tools"
    ]);
    
    // Query to analyze magical-technological balance
    let balance_query = world.run_command_success(&[
        "query",
        "SELECT 
           rs.from_id as race, 
           COUNT(*) as system_count,
           GROUP_CONCAT(rs.affinity) as affinities
         FROM race_system_relations rs 
         GROUP BY rs.from_id
         ORDER BY system_count DESC"
    ]);
    
    // Both races should have 3 system relations each
    assert!(balance_query.contains("3"));
    assert!(balance_query.contains("fire_giants"));
    assert!(balance_query.contains("tech_humans"));
}

#[test]
fn test_race_system_create_with_system_relation() {
    let world = TestWorld::new();
    
    // Create system
    world.run_command_success(&[
        "system", "create", "shadow_magic", 
        "--set", "display_name=Shadow Magic"
    ]);
    
    // Create race with system relation during creation
    let output = world.run_command_success(&[
        "race", "create", "shadow_elves", 
        "--set", "display_name=Shadow Elves",
        "--set", "lifespan=2000",
        "--set", "system=shadow_magic*born_with"
    ]);
    
    // Should show both race creation and relation creation
    assert!(output.contains("✨ Creating race: shadow_elves"));
    assert!(output.contains("✅ Created relation: shadow_elves -> shadow_magic"));
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Verify both race and relation exist
    let race_query = world.run_command_success(&[
        "query", 
        "SELECT display_name FROM races WHERE name = 'shadow_elves'"
    ]);
    assert!(race_query.contains("Shadow Elves"));
    
    let relation_query = world.run_command_success(&[
        "query", 
        "SELECT affinity FROM race_system_relations WHERE from_id = 'shadow_elves'"
    ]);
    assert!(relation_query.contains("born_with"));
}