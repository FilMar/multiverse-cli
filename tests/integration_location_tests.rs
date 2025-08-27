use std::process::Command;
use tempfile::TempDir;

struct TestWorld {
    #[allow(dead_code)]
    temp_dir: TempDir,
    world_path: std::path::PathBuf,
}

impl TestWorld {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let world_path = temp_dir.path().to_path_buf(); // world init creates files in current_dir
        
        // Get path to multiverse binary
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        // Initialize world
        let output = Command::new(&multiverse_bin)
            .args(&["world", "init", "test-universe"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to run world init");
            
        if !output.status.success() {
            panic!("World init failed: {}\nStdout: {}", 
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        
        // World initialized successfully
        
        Self { temp_dir, world_path }
    }
    
    
    fn run_command(&self, args: &[&str]) -> std::process::Output {
        // Get path to multiverse binary
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        Command::new(&multiverse_bin)
            .args(args)
            .current_dir(&self.world_path)
            .output()
            .expect("Failed to run command")
    }
}

#[test]
fn test_location_create_basic() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "location", "create", "waterdeep", 
        "--set", "title=City of Splendors"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Location 'waterdeep' created!"));
    assert!(stdout.contains("Title: City of Splendors"));
}

#[test]
fn test_location_create_with_metadata() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "location", "create", "neverwinter",
        "--set", "title=The Jewel of the North",
        "--set", "type=City",
        "--set", "population=23000",
        "--set", "ruler=Lord Dagult Neverember"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Location 'neverwinter' created!"));
    assert!(stdout.contains("Title: The Jewel of the North"));
    assert!(stdout.contains("type: \"City\""));
    assert!(stdout.contains("population: \"23000\""));
}

#[test]
fn test_location_list_empty() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["location", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No locations found"));
}

#[test]
fn test_location_list_with_locations() {
    let world = TestWorld::new();
    
    // Create multiple locations
    world.run_command(&["location", "create", "waterdeep", "--set", "title=City of Splendors"]);
    world.run_command(&["location", "create", "baldursgate", "--set", "title=Gate to Baldur"]);
    
    let output = world.run_command(&["location", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Locations in current world:"));
    assert!(stdout.contains("waterdeep"));
    assert!(stdout.contains("baldursgate"));
    assert!(stdout.contains("City of Splendors"));
    assert!(stdout.contains("Gate to Baldur"));
}

#[test]
fn test_location_info() {
    let world = TestWorld::new();
    
    // Create a location
    world.run_command(&[
        "location", "create", "silverymoon",
        "--set", "title=City of Love",
        "--set", "type=City", 
        "--set", "description=A beautiful elven city"
    ]);
    
    let output = world.run_command(&["location", "info", "silverymoon"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Location: silverymoon - \"City of Love\""));
    assert!(stdout.contains("Status: Active"));
    assert!(stdout.contains("type: \"City\""));
    assert!(stdout.contains("description: \"A beautiful elven city\""));
}

#[test]
fn test_location_info_not_found() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["location", "info", "nonexistent"]);
    
    assert!(!output.status.success(), "Command should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Location 'nonexistent' not found"));
}

#[test]
fn test_location_update() {
    let world = TestWorld::new();
    
    // Create a location
    world.run_command(&["location", "create", "luskan", "--set", "title=City of Sails"]);
    
    // Update it
    let output = world.run_command(&[
        "location", "update", "luskan",
        "--set", "title=The Pirate City",
        "--set", "danger_level=High"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Location 'luskan' updated!"));
    assert!(stdout.contains("title: \"The Pirate City\""));
    assert!(stdout.contains("danger_level: \"High\""));
}

#[test]
fn test_location_delete_without_force() {
    let world = TestWorld::new();
    
    // Create a location
    world.run_command(&["location", "create", "mirabar", "--set", "title=The Mining City"]);
    
    // Try to delete without force
    let output = world.run_command(&["location", "delete", "mirabar"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Are you sure you want to delete"));
    assert!(stdout.contains("Use --force"));
    
    // Verify location still exists
    let info_output = world.run_command(&["location", "info", "mirabar"]);
    assert!(info_output.status.success(), "Location should still exist");
}

#[test]
fn test_location_delete_with_force() {
    let world = TestWorld::new();
    
    // Create a location
    world.run_command(&["location", "create", "sundabar", "--set", "title=The Shield Dwarf City"]);
    
    // Delete with force
    let output = world.run_command(&["location", "delete", "sundabar", "--force"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Location 'sundabar' deleted!"));
    
    // Verify location no longer exists
    let info_output = world.run_command(&["location", "info", "sundabar"]);
    assert!(!info_output.status.success(), "Location should not exist anymore");
}

#[test]
fn test_location_lifecycle_complete() {
    let world = TestWorld::new();
    
    // Create location with comprehensive metadata
    let output = world.run_command(&[
        "location", "create", "candlekeep",
        "--set", "title=Fortress of Knowledge",
        "--set", "type=Library Fortress",
        "--set", "founded=Year of the Prince",
        "--set", "notable_feature=Great Library",
        "--set", "access=Restricted"
    ]);
    assert!(output.status.success(), "Failed to create location");
    
    // List and verify it appears
    let output = world.run_command(&["location", "list"]);
    assert!(output.status.success(), "Failed to list locations");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candlekeep"));
    assert!(stdout.contains("Fortress of Knowledge"));
    
    // Get info and verify all fields
    let output = world.run_command(&["location", "info", "candlekeep"]);
    assert!(output.status.success(), "Failed to get location info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candlekeep - \"Fortress of Knowledge\""));
    assert!(stdout.contains("type: \"Library Fortress\""));
    assert!(stdout.contains("founded: \"Year of the Prince\""));
    assert!(stdout.contains("notable_feature: \"Great Library\""));
    
    // Update some fields
    let output = world.run_command(&[
        "location", "update", "candlekeep",
        "--set", "current_keeper=Alaundo",
        "--set", "security_level=Maximum"
    ]);
    assert!(output.status.success(), "Failed to update location");
    
    // Verify updates
    let output = world.run_command(&["location", "info", "candlekeep"]);
    assert!(output.status.success(), "Failed to get updated location info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("current_keeper: \"Alaundo\""));
    assert!(stdout.contains("security_level: \"Maximum\""));
    
    // Finally, delete
    let output = world.run_command(&["location", "delete", "candlekeep", "--force"]);
    assert!(output.status.success(), "Failed to delete location");
    
    // Confirm deletion
    let output = world.run_command(&["location", "info", "candlekeep"]);
    assert!(!output.status.success(), "Location should be deleted");
}