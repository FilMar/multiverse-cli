//! Integration tests for Event-Faction relations
//! Tests the complete flow of creating and querying event-faction relationships

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
            panic!("World init failed: {}
Stdout: {}", 
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
fn test_event_faction_basic_political_event() {
    let world = TestWorld::new();

    // Create a faction
    let output = world.run_command_success(&["faction", "create", "crown-council", "--set", "display_name=The Crown Council"]);
    assert!(output.contains("Faction 'crown-council' created"));

    // Create an event with faction relation
    let output = world.run_command_success(&["event", "create", "succession-crisis", 
            "--set", "display_name=The Great Succession Crisis",
            "--set", "faction=crown-council*organizer"]);
    assert!(output.contains("Event 'succession-crisis' created"));
    assert!(output.contains("🔗 Processed relations: faction"));

    // Verify the relation was created
    let output = world.run_command_success(&["event", "info", "succession-crisis"]);
    assert!(output.contains("succession-crisis"));
    assert!(output.contains("The Great Succession Crisis"));
}

#[test]
fn test_event_faction_military_campaign() {
    let world = TestWorld::new();

    // Create military factions
    world.run_command_success(&["faction", "create", "imperial-legion", "--set", "display_name=The Imperial Legion"]);
    world.run_command_success(&["faction", "create", "rebel-alliance", "--set", "display_name=Rebel Alliance"]);

    // Create military event with multiple faction relations
    let output = world.run_command_success(&["event", "create", "battle-of-redfield",
                "--set", "display_name=Battle of Redfield",
                "--set", "faction=imperial-legion*attacker,rebel-alliance*defender"]);
    assert!(output.contains("Event 'battle-of-redfield' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_diplomatic_summit() {
    let world = TestWorld::new();

    // Create diplomatic factions
    world.run_command_success(&["faction", "create", "merchants-guild", "--set", "display_name=Merchants Guild"]);
    world.run_command_success(&["faction", "create", "nobles-court", "--set", "display_name=Noble Court"]);

    // Create diplomatic event
    let output = world.run_command_success(&["event", "create", "trade-negotiations",
                "--set", "display_name=Trade Negotiations of 1425",
                "--set", "faction=merchants-guild*negotiator,nobles-court*mediator"]);
    assert!(output.contains("Event 'trade-negotiations' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_festival_celebration() {
    let world = TestWorld::new();

    // Create cultural factions
    world.run_command_success(&["faction", "create", "temple-of-light", "--set", "display_name=Temple of Light"]);
    world.run_command_success(&["faction", "create", "artisans-circle", "--set", "display_name=Circle of Artisans"]);

    // Create festival event
    let output = world.run_command_success(&["event", "create", "harvest-festival",
                "--set", "display_name=Grand Harvest Festival",
                "--set", "faction=temple-of-light*sponsor,artisans-circle*organizer"]);
    assert!(output.contains("Event 'harvest-festival' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_conspiracy() {
    let world = TestWorld::new();

    // Create secretive factions
    world.run_command_success(&["faction", "create", "shadow-brotherhood", "--set", "display_name=Shadow Brotherhood"]);

    // Create conspiracy event
    let output = world.run_command_success(&["event", "create", "palace-coup",
                "--set", "display_name=The Palace Coup Attempt",
                "--set", "faction=shadow-brotherhood*conspirator"]);
    assert!(output.contains("Event 'palace-coup' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_validation_nonexistent() {
    let world = TestWorld::new();

    // Try to create event with non-existent faction
    let error = world.run_command_expect_failure(&["event", "create", "failed-event",
                "--set", "display_name=This Should Fail",
                "--set", "faction=nonexistent-faction*participant"]);
    assert!(error.contains("Faction 'nonexistent-faction' does not exist"));
}

#[test]
fn test_event_faction_update_relations() {
    let world = TestWorld::new();

    // Create factions
    world.run_command_success(&["faction", "create", "royal-guard", "--set", "display_name=Royal Guard"]);
    world.run_command_success(&["faction", "create", "city-watch", "--set", "display_name=City Watch"]);

    // Create event
    world.run_command_success(&["event", "create", "security-meeting",
                "--set", "display_name=Security Council Meeting"]);

    // Update with faction relations
    let output = world.run_command_success(&["event", "update", "security-meeting",
                "--set", "faction=royal-guard*attendee,city-watch*observer"]);
    assert!(output.contains("Event 'security-meeting' updated"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_economic_crisis() {
    let world = TestWorld::new();

    // Create economic factions
    world.run_command_success(&["faction", "create", "banking-consortium", "--set", "display_name=Banking Consortium"]);
    world.run_command_success(&["faction", "create", "workers-union", "--set", "display_name=Workers Union"]);

    // Create economic crisis event
    let output = world.run_command_success(&["event", "create", "market-crash",
                "--set", "display_name=The Great Market Crash",
                "--set", "faction=banking-consortium*victim,workers-union*affected_party"]);
    assert!(output.contains("Event 'market-crash' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_religious_ceremony() {
    let world = TestWorld::new();

    // Create religious factions
    world.run_command_success(&["faction", "create", "high-priests", "--set", "display_name=Order of High Priests"]);
    world.run_command_success(&["faction", "create", "temple-guards", "--set", "display_name=Temple Guard"]);

    // Create religious event
    let output = world.run_command_success(&["event", "create", "solar-blessing",
                "--set", "display_name=Great Solar Blessing Ceremony",
                "--set", "faction=high-priests*officiant,temple-guards*protector"]);
    assert!(output.contains("Event 'solar-blessing' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}

#[test]
fn test_event_faction_complex_alliance() {
    let world = TestWorld::new();

    // Create multiple factions for complex alliance scenario
    world.run_command_success(&["faction", "create", "northern-lords", "--set", "display_name=Northern Lords"]);
    world.run_command_success(&["faction", "create", "eastern-kingdoms", "--set", "display_name=Eastern Kingdoms"]);
    world.run_command_success(&["faction", "create", "free-cities", "--set", "display_name=Alliance of Free Cities"]);

    // Create alliance formation event
    let output = world.run_command_success(&["event", "create", "great-alliance",
                "--set", "display_name=Formation of the Great Alliance",
                "--set", "faction=northern-lords*founding_member,eastern-kingdoms*founding_member,free-cities*founding_member"]);
    assert!(output.contains("Event 'great-alliance' created"));
    assert!(output.contains("🔗 Processed relations: faction"));
}