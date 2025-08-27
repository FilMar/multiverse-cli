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
fn test_event_create_basic() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "event", "create", "battle_of_helm", 
        "--set", "title=Battle of Helm's Deep"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Event 'battle_of_helm' created!"));
    assert!(stdout.contains("Title: Battle of Helm's Deep"));
}

#[test]
fn test_event_create_with_date() {
    let world = TestWorld::new();
    
    let output = world.run_command(&[
        "event", "create", "siege_of_minas_tirith",
        "--set", "title=Siege of Minas Tirith",
        "--set", "date=2024-03-15T00:00:00Z",
        "--set", "type=Battle",
        "--set", "importance=High"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Event 'siege_of_minas_tirith' created!"));
    assert!(stdout.contains("Title: Siege of Minas Tirith"));
    assert!(stdout.contains("Date: 2024-03-15T00:00:00Z"));
    assert!(stdout.contains("Sort key:"));
    assert!(stdout.contains("type: \"Battle\""));
    assert!(stdout.contains("importance: \"High\""));
}

#[test]
fn test_event_list_empty() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["event", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No events found"));
}

#[test]
fn test_event_list_with_events() {
    let world = TestWorld::new();
    
    // Create multiple events
    world.run_command(&["event", "create", "battle_of_pelennor", "--set", "title=Battle of Pelennor Fields"]);
    world.run_command(&["event", "create", "council_of_elrond", "--set", "title=Council of Elrond"]);
    
    let output = world.run_command(&["event", "list"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Events in current world:"));
    assert!(stdout.contains("battle_of_pelennor"));
    assert!(stdout.contains("council_of_elrond"));
    assert!(stdout.contains("Battle of Pelennor Fields"));
    assert!(stdout.contains("Council of Elrond"));
}

#[test]
fn test_event_info() {
    let world = TestWorld::new();
    
    // Create an event
    world.run_command(&[
        "event", "create", "coronation_of_aragorn",
        "--set", "title=Coronation of King Elessar",
        "--set", "date=2024-05-01T00:00:00Z", 
        "--set", "type=Political",
        "--set", "description=Aragorn is crowned King of Gondor"
    ]);
    
    let output = world.run_command(&["event", "info", "coronation_of_aragorn"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Event: coronation_of_aragorn - \"Coronation of King Elessar\""));
    assert!(stdout.contains("Status: Active"));
    assert!(stdout.contains("Date: 2024-05-01T00:00:00Z"));
    assert!(stdout.contains("Description: Aragorn is crowned King of Gondor"));
    assert!(stdout.contains("type: \"Political\""));
    assert!(stdout.contains("description: \"Aragorn is crowned King of Gondor\""));
}

#[test]
fn test_event_info_not_found() {
    let world = TestWorld::new();
    
    let output = world.run_command(&["event", "info", "nonexistent"]);
    
    assert!(!output.status.success(), "Command should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Event 'nonexistent' not found"));
}

#[test]
fn test_event_update() {
    let world = TestWorld::new();
    
    // Create an event
    world.run_command(&["event", "create", "fall_of_isengard", "--set", "title=Fall of Isengard"]);
    
    // Update it
    let output = world.run_command(&[
        "event", "update", "fall_of_isengard",
        "--set", "title=Destruction of Isengard",
        "--set", "date=2024-06-01T00:00:00Z",
        "--set", "outcome=Saruman defeated"
    ]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Event 'fall_of_isengard' updated!"));
    assert!(stdout.contains("Title: Destruction of Isengard"));
    assert!(stdout.contains("Date: 2024-06-01T00:00:00Z"));
    assert!(stdout.contains("outcome: \"Saruman defeated\""));
}

#[test]
fn test_event_delete_without_force() {
    let world = TestWorld::new();
    
    // Create an event
    world.run_command(&["event", "create", "wedding_of_faramir", "--set", "title=Wedding of Faramir and Eowyn"]);
    
    // Try to delete without force
    let output = world.run_command(&["event", "delete", "wedding_of_faramir"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Are you sure you want to delete"));
    assert!(stdout.contains("Use --force"));
    
    // Verify event still exists
    let info_output = world.run_command(&["event", "info", "wedding_of_faramir"]);
    assert!(info_output.status.success(), "Event should still exist");
}

#[test]
fn test_event_delete_with_force() {
    let world = TestWorld::new();
    
    // Create an event
    world.run_command(&["event", "create", "departure_of_gandalf", "--set", "title=Departure of Gandalf"]);
    
    // Delete with force
    let output = world.run_command(&["event", "delete", "departure_of_gandalf", "--force"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Event 'departure_of_gandalf' deleted!"));
    
    // Verify event no longer exists
    let info_output = world.run_command(&["event", "info", "departure_of_gandalf"]);
    assert!(!info_output.status.success(), "Event should not exist anymore");
}

#[test]
fn test_event_timeline() {
    let world = TestWorld::new();
    
    // Create events with dates in different chronological order
    world.run_command(&[
        "event", "create", "event_c",
        "--set", "title=Third Event",
        "--set", "date=2024-03-15T00:00:00Z"
    ]);
    world.run_command(&[
        "event", "create", "event_a", 
        "--set", "title=First Event",
        "--set", "date=2024-01-01T00:00:00Z"
    ]);
    world.run_command(&[
        "event", "create", "event_b",
        "--set", "title=Second Event", 
        "--set", "date=2024-02-15T00:00:00Z"
    ]);
    
    let output = world.run_command(&["event", "timeline"]);
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("DEBUG TIMELINE OUTPUT: {}", stdout);
    assert!(stdout.contains("Events Timeline (chronological order):"));
    
    // Events should be sorted chronologically
    let first_event_pos = stdout.find("First Event").unwrap();
    let second_event_pos = stdout.find("Second Event").unwrap();
    let third_event_pos = stdout.find("Third Event").unwrap();
    
    assert!(first_event_pos < second_event_pos);
    assert!(second_event_pos < third_event_pos);
}

#[test]
fn test_event_lifecycle_complete() {
    let world = TestWorld::new();
    
    // Create event with comprehensive metadata
    let output = world.run_command(&[
        "event", "create", "war_of_the_ring",
        "--set", "title=War of the Ring",
        "--set", "date=2024-12-25T00:00:00Z",
        "--set", "type=War",
        "--set", "duration=Several months",
        "--set", "participants=Fellowship, Rohan, Gondor",
        "--set", "outcome=Victory over Sauron"
    ]);
    assert!(output.status.success(), "Failed to create event");
    
    // List and verify it appears
    let output = world.run_command(&["event", "list"]);
    assert!(output.status.success(), "Failed to list events");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("war_of_the_ring"));
    assert!(stdout.contains("War of the Ring"));
    assert!(stdout.contains("Date: 2024-12-25"));
    
    // Get info and verify all fields
    let output = world.run_command(&["event", "info", "war_of_the_ring"]);
    assert!(output.status.success(), "Failed to get event info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("war_of_the_ring - \"War of the Ring\""));
    assert!(stdout.contains("Date: 2024-12-25T00:00:00Z"));
    assert!(stdout.contains("type: \"War\""));
    assert!(stdout.contains("duration: \"Several months\""));
    assert!(stdout.contains("participants: \"Fellowship, Rohan, Gondor\""));
    
    // Test timeline view
    let output = world.run_command(&["event", "timeline"]);
    assert!(output.status.success(), "Failed to get timeline");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2024-12-25T00:00:00Z - \"War of the Ring\""));
    
    // Update some fields
    let output = world.run_command(&[
        "event", "update", "war_of_the_ring",
        "--set", "outcome=Sauron defeated, Ring destroyed",
        "--set", "date=2024-12-31T00:00:00Z"
    ]);
    assert!(output.status.success(), "Failed to update event");
    
    // Verify updates including new sort key for new date
    let output = world.run_command(&["event", "info", "war_of_the_ring"]);
    assert!(output.status.success(), "Failed to get updated event info");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("outcome: \"Sauron defeated, Ring destroyed\""));
    assert!(stdout.contains("Date: 2024-12-31T00:00:00Z"));
    
    // Finally, delete
    let output = world.run_command(&["event", "delete", "war_of_the_ring", "--force"]);
    assert!(output.status.success(), "Failed to delete event");
    
    // Confirm deletion
    let output = world.run_command(&["event", "info", "war_of_the_ring"]);
    assert!(!output.status.success(), "Event should be deleted");
}