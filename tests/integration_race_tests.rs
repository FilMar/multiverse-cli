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
fn test_race_create_basic() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "race", "create", "elves", 
        "--set", "display_name=High Elves"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created Race 'elves'"));
    assert!(stdout.contains("Display name: High Elves"));
    assert!(stdout.contains("Status: Active"));
}

#[test]
fn test_race_create_with_metadata() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "race", "create", "dwarves",
        "--set", "display_name=Mountain Dwarves",
        "--set", "lifespan=400",
        "--set", "origin=The Great Mountains",
        "--set", "abilities=Darkvision, Stonecunning",
        "--set", "description=Hardy folk who live in mountain strongholds"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created Race 'dwarves'"));
    assert!(stdout.contains("Display name: Mountain Dwarves"));
    assert!(stdout.contains("lifespan: \"400\""));
    assert!(stdout.contains("origin: \"The Great Mountains\""));
    assert!(stdout.contains("abilities: \"Darkvision, Stonecunning\""));
    assert!(stdout.contains("Description: Hardy folk who live in mountain strongholds"));
}

#[test]
fn test_race_list_empty() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["race", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No races found"));
}

#[test]
fn test_race_list_with_races() {
    let world = TestWorld::new();
    
    // Create multiple races
    world.run_command(&["race", "create", "humans", "--set", "display_name=Humans"]);
    world.run_command(&["race", "create", "halflings", "--set", "display_name=Halflings", "--set", "description=Small folk"]);
    
    let output = world.run_command(&["race", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Races in current world:"));
    assert!(stdout.contains("humans"));
    assert!(stdout.contains("halflings"));
    assert!(stdout.contains("Humans"));
    assert!(stdout.contains("Halflings"));
    assert!(stdout.contains("Small folk"));
}

#[test]
fn test_race_info() {
    let world = TestWorld::new();
    
    // Create a race
    world.run_command(&[
        "race", "create", "orcs",
        "--set", "display_name=Savage Orcs",
        "--set", "lifespan=80", 
        "--set", "origin=The Dark Lands",
        "--set", "description=Fierce warriors from the wasteland"
    ]);
    
    let output = world.run_command(&["race", "info", "orcs"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Race: orcs - \"Savage Orcs\""));
    assert!(stdout.contains("Status: Active"));
    assert!(stdout.contains("lifespan: \"80\""));
    assert!(stdout.contains("origin: \"The Dark Lands\""));
    assert!(stdout.contains("Description: Fierce warriors from the wasteland"));
}

#[test]
fn test_race_info_not_found() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["race", "info", "nonexistent"]);
    
    assert!(!output.status.success(), "Command should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Race 'nonexistent' not found"));
}

#[test]
fn test_race_update() {
    let world = TestWorld::new();
    
    // Create a race
    world.run_command(&["race", "create", "gnomes", "--set", "display_name=Forest Gnomes"]);
    
    // Update it
    let output = world.run_command(&[
        "race", "update", "gnomes",
        "--set", "display_name=Garden Gnomes",
        "--set", "habitat=Gardens and Forests",
        "--set", "magic_affinity=Nature Magic"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Race 'gnomes' updated!"));
    assert!(stdout.contains("Display name: Garden Gnomes"));
    assert!(stdout.contains("habitat: \"Gardens and Forests\""));
    assert!(stdout.contains("magic_affinity: \"Nature Magic\""));
}

#[test]
fn test_race_delete_without_force() {
    let world = TestWorld::new();
    
    // Create a race
    world.run_command(&["race", "create", "goblins", "--set", "display_name=Cave Goblins"]);
    
    // Try to delete without force
    let output = world.run_command(&["race", "delete", "goblins"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Are you sure you want to delete"));
    assert!(stdout.contains("Use --force"));
    
    // Verify race still exists
    let info_output = world.run_command(&["race", "info", "goblins"]);
    assert!(info_output.status.success(), "Race should still exist");
}

#[test]
fn test_race_delete_with_force() {
    let world = TestWorld::new();
    
    // Create a race
    world.run_command(&["race", "create", "trolls", "--set", "display_name=Stone Trolls"]);
    
    // Delete with force
    let output = world.run_command(&["race", "delete", "trolls", "--force"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Race 'trolls' deleted!"));
    
    // Verify race no longer exists
    let info_output = world.run_command(&["race", "info", "trolls"]);
    assert!(!info_output.status.success(), "Race should not exist anymore");
}

#[test]
fn test_race_lifecycle_complete() {
    let world = TestWorld::new();
    
    // Create race with comprehensive metadata
    let output = world.run_command(&[
        "race", "create", "dragonborn",
        "--set", "display_name=Dragonborn Warriors",
        "--set", "lifespan=80",
        "--set", "origin=Ancient Dragon Empire",
        "--set", "abilities=Dragon Breath, Draconic Heritage",
        "--set", "culture=Honor-bound warrior society",
        "--set", "description=Descendants of dragons with humanoid form"
    ]);
    assert!(output.status.success(), "Failed to create race");
    
    // List and verify it appears
    let output = world.run_command(&["race", "list"]);
    assert!(output.status.success(), "Failed to list races");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dragonborn"));
    assert!(stdout.contains("Dragonborn Warriors"));
    
    // Get info and verify all fields
    let output = world.run_command(&["race", "info", "dragonborn"]);
    assert!(output.status.success(), "Failed to get race info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dragonborn - \"Dragonborn Warriors\""));
    assert!(stdout.contains("lifespan: \"80\""));
    assert!(stdout.contains("origin: \"Ancient Dragon Empire\""));
    assert!(stdout.contains("abilities: \"Dragon Breath, Draconic Heritage\""));
    assert!(stdout.contains("culture: \"Honor-bound warrior society\""));
    
    // Update some fields
    let output = world.run_command(&[
        "race", "update", "dragonborn",
        "--set", "status=Legendary",
        "--set", "current_population=Few"
    ]);
    assert!(output.status.success(), "Failed to update race");
    
    // Verify updates
    let output = world.run_command(&["race", "info", "dragonborn"]);
    assert!(output.status.success(), "Failed to get updated race info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Status: Legendary"));
    assert!(stdout.contains("current_population: \"Few\""));
    
    // Finally, delete
    let output = world.run_command(&["race", "delete", "dragonborn", "--force"]);
    assert!(output.status.success(), "Failed to delete race");
    
    // Confirm deletion
    let output = world.run_command(&["race", "info", "dragonborn"]);
    assert!(!output.status.success(), "Race should be deleted");
}