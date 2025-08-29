//! Integration tests for Location-Faction relations
//! Tests the complete flow of creating and querying location-faction relationships

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
fn test_location_faction_basic_relation() {
    let world = TestWorld::new();
    
    // Create faction first
    world.run_command_success(&[
        "faction", "create", "gondor", 
        "--set", "display_name=Kingdom of Gondor"
    ]);
    
    // Create location with faction relation
    let output = world.run_command_success(&[
        "location", "create", "minas_tirith", 
        "--set", "display_name=Minas Tirith",
        "--set", "faction=gondor*capital"
    ]);
    
    // Should show both location creation and relation creation
    assert!(output.contains("📍 Creating location 'minas_tirith'"));
    assert!(output.contains("🔗 Processed relations: faction"));
    assert!(output.contains("✅ Created relation: minas_tirith -> gondor"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, control_type FROM location_faction_relations"
    ]);
    
    assert!(query_output.contains("minas_tirith"));
    assert!(query_output.contains("gondor"));
    assert!(query_output.contains("capital"));
}

#[test]
fn test_location_faction_create_with_relation() {
    let world = TestWorld::new();
    
    // Create faction
    world.run_command_success(&["faction", "create", "rohan", "--set", "display_name=Kingdom of Rohan"]);
    
    // Create location with faction relation during creation
    let output = world.run_command_success(&[
        "location", "create", "edoras", 
        "--set", "display_name=Edoras",
        "--set", "faction=rohan*capital"
    ]);
    
    // Should show both location creation and relation creation
    assert!(output.contains("📍 Creating location 'edoras'"));
    assert!(output.contains("✅ Created relation: edoras -> rohan"));
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Verify relation exists
    let query_output = world.run_command_success(&[
        "query", 
        "SELECT control_type FROM location_faction_relations WHERE from_id = 'edoras'"
    ]);
    assert!(query_output.contains("capital"));
}

#[test]
fn test_location_faction_default_control() {
    let world = TestWorld::new();
    
    // Create faction
    world.run_command_success(&["faction", "create", "isengard", "--set", "display_name=Isengard"]);
    
    // Create location without specifying control type (should default to "controlled")
    let output = world.run_command_success(&[
        "location", "create", "orthanc", 
        "--set", "display_name=Orthanc Tower",
        "--set", "faction=isengard"
    ]);
    
    assert!(output.contains("✅ Created relation: orthanc -> isengard"));
    
    // Query should show default control
    let query_output = world.run_command_success(&[
        "query",
        "SELECT control_type FROM location_faction_relations WHERE from_id = 'orthanc'"
    ]);
    
    assert!(query_output.contains("controlled"));
}

#[test]
fn test_location_faction_multiple_controls() {
    let world = TestWorld::new();
    
    // Create multiple factions
    world.run_command_success(&["faction", "create", "white_council", "--set", "display_name=White Council"]);
    world.run_command_success(&["faction", "create", "shire_folk", "--set", "display_name=Shire Folk"]);
    
    // Create location with multiple faction controls
    let output = world.run_command_success(&[
        "location", "create", "rivendell", 
        "--set", "display_name=Rivendell",
        "--set", "faction=white_council*sanctuary,shire_folk*refuge"
    ]);
    
    // Should process relations
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Query should show both controls
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, control_type FROM location_faction_relations WHERE from_id = 'rivendell' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("sanctuary"));
    assert!(query_output.contains("refuge"));
    assert!(query_output.contains("📊 2 row(s) returned"));
}

#[test]
fn test_location_faction_validation() {
    let world = TestWorld::new();
    
    // Try to create location with non-existent faction
    let error = world.run_command_expect_failure(&[
        "location", "create", "barad_dur", 
        "--set", "display_name=Barad-dûr",
        "--set", "faction=mordor*fortress"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Faction 'mordor' does not exist"));
    assert!(error.contains("multiverse faction create mordor"));
}

#[test]
fn test_location_faction_territorial_control() {
    let world = TestWorld::new();
    
    // Create faction representing territorial control
    world.run_command_success(&[
        "faction", "create", "rangers_north", 
        "--set", "display_name=Rangers of the North"
    ]);
    
    // Create locations with different control types
    world.run_command_success(&[
        "location", "create", "fornost", 
        "--set", "display_name=Fornost Ruins",
        "--set", "faction=rangers_north*patrolled"
    ]);
    
    world.run_command_success(&[
        "location", "create", "weathertop", 
        "--set", "display_name=Weathertop",
        "--set", "faction=rangers_north*watchtower"
    ]);
    
    // Query territorial control analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as location_name, 
           f.display_name as faction_name, 
           lf.control_type
         FROM locations l 
         JOIN location_faction_relations lf ON l.name = lf.from_id 
         JOIN factions f ON lf.to_id = f.name
         WHERE f.name = 'rangers_north'"
    ]);
    
    // Should show territorial control data
    assert!(query_output.contains("Fornost Ruins"));
    assert!(query_output.contains("Weathertop"));
    assert!(query_output.contains("Rangers of the North"));
    assert!(query_output.contains("patrolled"));
    assert!(query_output.contains("watchtower"));
}

#[test]
fn test_location_faction_contested_territory() {
    let world = TestWorld::new();
    
    // Create opposing factions
    world.run_command_success(&["faction", "create", "gondor_army", "--set", "display_name=Army of Gondor"]);
    world.run_command_success(&["faction", "create", "orc_raiders", "--set", "display_name=Orc Raiders"]);
    
    // Create contested location
    let output = world.run_command_success(&[
        "location", "create", "osgiliath", 
        "--set", "display_name=Osgiliath",
        "--set", "faction=gondor_army*contested,orc_raiders*occupied"
    ]);
    
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Query contested control analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           lf.control_type,
           f.display_name as controlling_faction
         FROM location_faction_relations lf
         JOIN factions f ON lf.to_id = f.name
         WHERE lf.from_id = 'osgiliath'
         ORDER BY lf.control_type"
    ]);
    
    // Should show both contested and occupied control
    assert!(query_output.contains("contested"));
    assert!(query_output.contains("occupied"));
    assert!(query_output.contains("Army of Gondor"));
    assert!(query_output.contains("Orc Raiders"));
}

#[test]
fn test_location_faction_update_control() {
    let world = TestWorld::new();
    
    // Create factions for changing control
    world.run_command_success(&["faction", "create", "saruman", "--set", "display_name=Saruman's Forces"]);
    world.run_command_success(&["faction", "create", "ents", "--set", "display_name=Ents of Fangorn"]);
    
    // Create location initially controlled by Saruman
    world.run_command_success(&[
        "location", "create", "isengard_vale", 
        "--set", "display_name=Isengard Vale"
    ]);
    
    // Add initial control
    world.run_command_success(&[
        "location", "update", "isengard_vale", 
        "--set", "faction=saruman*dominion"
    ]);
    
    // Later, Ents take control
    let output = world.run_command_success(&[
        "location", "update", "isengard_vale", 
        "--set", "faction=ents*reclaimed"
    ]);
    
    assert!(output.contains("🔗 Processed relations: faction"));
    
    // Query should show both historical controls
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, control_type FROM location_faction_relations WHERE from_id = 'isengard_vale' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("dominion"));
    assert!(query_output.contains("reclaimed"));
}