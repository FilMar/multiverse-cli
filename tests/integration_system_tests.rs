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
fn test_system_create_basic() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "system", "create", "magic_system", 
        "--set", "display_name=Magic System",
        "--set", "system_type=Magic"
    ]);
    
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created System 'magic_system'"));
    assert!(stdout.contains("Display name: Magic System"));
    assert!(stdout.contains("Type: Magic"));
}

#[test]
fn test_system_create_with_metadata() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "system", "create", "spell_system",
        "--set", "display_name=Spell Casting System",
        "--set", "system_type=Magic",
        "--set", "complexity=High",
        "--set", "school=Wizardry",
        "--set", "description=A complex magic system based on spell schools"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created System 'spell_system'"));
    assert!(stdout.contains("Display name: Spell Casting System"));
    assert!(stdout.contains("Type: Magic"));
    assert!(stdout.contains("complexity: \"High\""));
    assert!(stdout.contains("school: \"Wizardry\""));
    assert!(stdout.contains("Description: A complex magic system based on spell schools"));
}

#[test]
fn test_system_list_empty() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["system", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No systems found"));
}

#[test]
fn test_system_list_with_systems() {
    let world = TestWorld::new();
    
    // Create multiple systems
    world.run_command(&["system", "create", "magic_system", "--set", "display_name=Magic System", "--set", "system_type=Magic"]);
    world.run_command(&["system", "create", "tech_system", "--set", "display_name=Technology System", "--set", "system_type=Technology"]);
    
    let output = world.run_command(&["system", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Systems in current world:"));
    assert!(stdout.contains("magic_system"));
    assert!(stdout.contains("tech_system"));
    assert!(stdout.contains("Magic System"));
    assert!(stdout.contains("Technology System"));
}

#[test]
fn test_system_info() {
    let world = TestWorld::new();
    
    // Create a system
    world.run_command(&[
        "system", "create", "divine_magic",
        "--set", "display_name=Divine Magic System",
        "--set", "system_type=Magic", 
        "--set", "complexity=Medium",
        "--set", "description=Magic granted by deities"
    ]);
    
    let output = world.run_command(&["system", "info", "divine_magic"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("System: divine_magic - \"Divine Magic System\""));
    assert!(stdout.contains("Type: Magic"));
    assert!(stdout.contains("Status: Active"));
    assert!(stdout.contains("complexity: \"Medium\""));
    assert!(stdout.contains("description: \"Magic granted by deities\""));
}

#[test]
fn test_system_info_not_found() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["system", "info", "nonexistent"]);
    
    assert!(!output.status.success(), "Command should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("System 'nonexistent' not found"));
}

#[test]
fn test_system_update() {
    let world = TestWorld::new();
    
    // Create a system
    world.run_command(&["system", "create", "arcane_magic", "--set", "display_name=Arcane Magic", "--set", "system_type=Magic"]);
    
    // Update it
    let output = world.run_command(&[
        "system", "update", "arcane_magic",
        "--set", "display_name=Arcane Magic System",
        "--set", "complexity=Very High",
        "--set", "school=Arcane Arts"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("System 'arcane_magic' updated!"));
    assert!(stdout.contains("Display name: Arcane Magic System"));
    assert!(stdout.contains("complexity: \"Very High\""));
    assert!(stdout.contains("school: \"Arcane Arts\""));
}

#[test]
fn test_system_delete_without_force() {
    let world = TestWorld::new();
    
    // Create a system
    world.run_command(&["system", "create", "alchemy_system", "--set", "display_name=Alchemy System", "--set", "system_type=Crafting"]);
    
    // Try to delete without force
    let output = world.run_command(&["system", "delete", "alchemy_system"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Are you sure you want to delete"));
    assert!(stdout.contains("Use --force"));
    
    // Verify system still exists
    let info_output = world.run_command(&["system", "info", "alchemy_system"]);
    assert!(info_output.status.success(), "System should still exist");
}

#[test]
fn test_system_delete_with_force() {
    let world = TestWorld::new();
    
    // Create a system
    world.run_command(&["system", "create", "psionics_system", "--set", "display_name=Psionics System", "--set", "system_type=Psionic"]);
    
    // Delete with force
    let output = world.run_command(&["system", "delete", "psionics_system", "--force"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("System 'psionics_system' deleted!"));
    
    // Verify system no longer exists
    let info_output = world.run_command(&["system", "info", "psionics_system"]);
    assert!(!info_output.status.success(), "System should not exist anymore");
}

#[test]
fn test_system_lifecycle_complete() {
    let world = TestWorld::new();
    
    // Create system with comprehensive metadata
    let output = world.run_command(&[
        "system", "create", "martial_arts",
        "--set", "display_name=Martial Arts System",
        "--set", "system_type=Combat",
        "--set", "complexity=High",
        "--set", "schools=Multiple",
        "--set", "origin=Ancient Traditions",
        "--set", "description=Various fighting techniques and disciplines"
    ]);
    assert!(output.status.success(), "Failed to create system");
    
    // List and verify it appears
    let output = world.run_command(&["system", "list"]);
    assert!(output.status.success(), "Failed to list systems");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("martial_arts"));
    assert!(stdout.contains("Martial Arts System"));
    
    // Get info and verify all fields
    let output = world.run_command(&["system", "info", "martial_arts"]);
    assert!(output.status.success(), "Failed to get system info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("martial_arts - \"Martial Arts System\""));
    assert!(stdout.contains("Type: Combat"));
    assert!(stdout.contains("complexity: \"High\""));
    assert!(stdout.contains("schools: \"Multiple\""));
    assert!(stdout.contains("origin: \"Ancient Traditions\""));
    
    // Update some fields
    let output = world.run_command(&[
        "system", "update", "martial_arts",
        "--set", "status=Inactive",
        "--set", "modernization=In Progress"
    ]);
    assert!(output.status.success(), "Failed to update system");
    
    // Verify updates
    let output = world.run_command(&["system", "info", "martial_arts"]);
    assert!(output.status.success(), "Failed to get updated system info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Status: Inactive"));
    assert!(stdout.contains("modernization: \"In Progress\""));
    
    // Finally, delete
    let output = world.run_command(&["system", "delete", "martial_arts", "--force"]);
    assert!(output.status.success(), "Failed to delete system");
    
    // Confirm deletion
    let output = world.run_command(&["system", "info", "martial_arts"]);
    assert!(!output.status.success(), "System should be deleted");
}