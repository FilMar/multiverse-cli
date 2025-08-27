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
fn test_faction_create_basic() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "faction", "create", "harpers", 
        "--set", "title=The Harpers"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Faction 'harpers' created!"));
    assert!(stdout.contains("Title: The Harpers"));
}

#[test]
fn test_faction_create_with_metadata() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "faction", "create", "zhentarim",
        "--set", "title=Black Network",
        "--set", "type=Criminal Organization",
        "--set", "size=Large",
        "--set", "alignment=Lawful Evil"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Faction 'zhentarim' created!"));
    assert!(stdout.contains("Title: Black Network"));
    assert!(stdout.contains("type: \"Criminal Organization\""));
    assert!(stdout.contains("size: \"Large\""));
    assert!(stdout.contains("alignment: \"Lawful Evil\""));
}

#[test]
fn test_faction_list_empty() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["faction", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No factions found"));
}

#[test]
fn test_faction_list_with_factions() {
    let world = TestWorld::new();
    
    // Create multiple factions
    world.run_command(&["faction", "create", "lords_alliance", "--set", "title=Lords' Alliance"]);
    world.run_command(&["faction", "create", "emerald_enclave", "--set", "title=Emerald Enclave"]);
    
    let output = world.run_command(&["faction", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Factions in current world:"));
    assert!(stdout.contains("lords_alliance"));
    assert!(stdout.contains("emerald_enclave"));
    assert!(stdout.contains("Lords' Alliance"));
    assert!(stdout.contains("Emerald Enclave"));
}

#[test]
fn test_faction_info() {
    let world = TestWorld::new();
    
    // Create a faction
    world.run_command(&[
        "faction", "create", "order_of_gauntlet",
        "--set", "title=Order of the Gauntlet",
        "--set", "type=Paladin Order", 
        "--set", "description=A holy order dedicated to fighting evil"
    ]);
    
    let output = world.run_command(&["faction", "info", "order_of_gauntlet"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Faction: order_of_gauntlet - \"Order of the Gauntlet\""));
    assert!(stdout.contains("Status: Active"));
    assert!(stdout.contains("type: \"Paladin Order\""));
    assert!(stdout.contains("description: \"A holy order dedicated to fighting evil\""));
}

#[test]
fn test_faction_info_not_found() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["faction", "info", "nonexistent"]);
    
    assert!(!output.status.success(), "Command should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Faction 'nonexistent' not found"));
}

#[test]
fn test_faction_update() {
    let world = TestWorld::new();
    
    // Create a faction
    world.run_command(&["faction", "create", "red_wizards", "--set", "title=Red Wizards of Thay"]);
    
    // Update it
    let output = world.run_command(&[
        "faction", "update", "red_wizards",
        "--set", "title=Red Wizards",
        "--set", "threat_level=High",
        "--set", "specialization=Necromancy"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Faction 'red_wizards' updated!"));
    assert!(stdout.contains("Title: Red Wizards"));
    assert!(stdout.contains("threat_level: \"High\""));
    assert!(stdout.contains("specialization: \"Necromancy\""));
}

#[test]
fn test_faction_delete_without_force() {
    let world = TestWorld::new();
    
    // Create a faction
    world.run_command(&["faction", "create", "flaming_fist", "--set", "title=Flaming Fist Mercenaries"]);
    
    // Try to delete without force
    let output = world.run_command(&["faction", "delete", "flaming_fist"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Are you sure you want to delete"));
    assert!(stdout.contains("Use --force"));
    
    // Verify faction still exists
    let info_output = world.run_command(&["faction", "info", "flaming_fist"]);
    assert!(info_output.status.success(), "Faction should still exist");
}

#[test]
fn test_faction_delete_with_force() {
    let world = TestWorld::new();
    
    // Create a faction
    world.run_command(&["faction", "create", "cult_of_dragon", "--set", "title=Cult of the Dragon"]);
    
    // Delete with force
    let output = world.run_command(&["faction", "delete", "cult_of_dragon", "--force"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Faction 'cult_of_dragon' deleted!"));
    
    // Verify faction no longer exists
    let info_output = world.run_command(&["faction", "info", "cult_of_dragon"]);
    assert!(!info_output.status.success(), "Faction should not exist anymore");
}

#[test]
fn test_faction_lifecycle_complete() {
    let world = TestWorld::new();
    
    // Create faction with comprehensive metadata
    let output = world.run_command(&[
        "faction", "create", "thieves_guild",
        "--set", "title=Waterdeep Thieves' Guild",
        "--set", "type=Criminal Organization",
        "--set", "size=Medium",
        "--set", "base=Waterdeep",
        "--set", "leader=The Xanathar",
        "--set", "primary_activity=Smuggling and Theft"
    ]);
    assert!(output.status.success(), "Failed to create faction");
    
    // List and verify it appears
    let output = world.run_command(&["faction", "list"]);
    assert!(output.status.success(), "Failed to list factions");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("thieves_guild"));
    assert!(stdout.contains("Waterdeep Thieves' Guild"));
    
    // Get info and verify all fields
    let output = world.run_command(&["faction", "info", "thieves_guild"]);
    assert!(output.status.success(), "Failed to get faction info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("thieves_guild - \"Waterdeep Thieves' Guild\""));
    assert!(stdout.contains("type: \"Criminal Organization\""));
    assert!(stdout.contains("size: \"Medium\""));
    assert!(stdout.contains("base: \"Waterdeep\""));
    assert!(stdout.contains("leader: \"The Xanathar\""));
    
    // Update some fields
    let output = world.run_command(&[
        "faction", "update", "thieves_guild",
        "--set", "leader=New Leader",
        "--set", "status=Under Investigation"
    ]);
    assert!(output.status.success(), "Failed to update faction");
    
    // Verify updates
    let output = world.run_command(&["faction", "info", "thieves_guild"]);
    assert!(output.status.success(), "Failed to get updated faction info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("leader: \"New Leader\""));
    assert!(stdout.contains("status: \"Under Investigation\""));
    
    // Finally, delete
    let output = world.run_command(&["faction", "delete", "thieves_guild", "--force"]);
    assert!(output.status.success(), "Failed to delete faction");
    
    // Confirm deletion
    let output = world.run_command(&["faction", "info", "thieves_guild"]);
    assert!(!output.status.success(), "Faction should be deleted");
}