//! Integration tests for Character entity with full world setup
//! These tests simulate the complete user experience using Middle-earth characters

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
        let world_path = temp_dir.path().to_path_buf(); // world init creates files in current_dir
        
        // Get path to multiverse binary
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        // Initialize world
        let output = Command::new(&multiverse_bin)
            .args(&["world", "init", "middle-earth"])
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
    
    fn run_command_success(&self, args: &[&str]) -> String {
        let output = self.run_command(args);
        if !output.status.success() {
            panic!(
                "Command failed: {:?}\nStderr: {}\nStdout: {}", 
                args,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        String::from_utf8_lossy(&output.stdout).to_string()
    }
    
    fn database_exists(&self) -> bool {
        self.world_path.join(".multiverse/world.db").exists()
    }
}

#[test]
fn test_middle_earth_world_initialization() {
    let world = TestWorld::new();
    
    // Verify world structure was created
    assert!(world.world_path.exists(), "Middle-earth directory should exist");
    assert!(world.database_exists(), "Middle-earth database should exist");
    assert!(world.world_path.join(".multiverse/config.toml").exists(), "Config should exist");
    assert!(world.world_path.join("README.md").exists(), "README should exist");
}

#[test]
fn test_create_aragorn() {
    let world = TestWorld::new();
    
    // Create Aragorn, Heir of Isildur
    let output = world.run_command_success(&[
        "character", "create", "aragorn", 
        "--set", "display_name=Aragorn, son of Arathorn", 
        "--set", "race=Man",
        "--set", "kingdom=Gondor",
        "--set", "title=King of Gondor",
        "--set", "weapon=Andúril",
        "--set", "status=Active"
    ]);
    
    assert!(output.contains("Created Character 'aragorn'"));
    assert!(output.contains("Aragorn, son of Arathorn"));
    assert!(output.contains("race: \"Man\""));
    assert!(output.contains("kingdom: \"Gondor\""));
    assert!(output.contains("weapon: \"Andúril\""));
}

#[test]
fn test_create_fellowship_members() {
    let world = TestWorld::new();
    
    // Create the Fellowship of the Ring
    world.run_command_success(&[
        "character", "create", "frodo",
        "--set", "display_name=Frodo Baggins",
        "--set", "race=Hobbit",
        "--set", "home=Bag End, Hobbiton",
        "--set", "burden=The One Ring",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "legolas", 
        "--set", "display_name=Legolas Greenleaf",
        "--set", "race=Elf",
        "--set", "realm=Woodland Realm",
        "--set", "father=Thranduil",
        "--set", "weapon=Bow of the Galadhrim",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "gimli",
        "--set", "display_name=Gimli, son of Glóin",
        "--set", "race=Dwarf",
        "--set", "home=Erebor",
        "--set", "weapon=Battle axe",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "gandalf",
        "--set", "display_name=Gandalf the Grey",
        "--set", "race=Maiar",
        "--set", "order=Istari",
        "--set", "color=Grey",
        "--set", "staff=Staff of Power",
        "--set", "status=Active"
    ]);
    
    // List the Fellowship
    let output = world.run_command_success(&["character", "list"]);
    
    assert!(output.contains("frodo - \"Frodo Baggins\""));
    assert!(output.contains("legolas - \"Legolas Greenleaf\""));
    assert!(output.contains("gimli - \"Gimli, son of Glóin\""));
    assert!(output.contains("gandalf - \"Gandalf the Grey\""));
}

#[test]
fn test_boromir_lifecycle() {
    let world = TestWorld::new();
    
    // Create Boromir of Gondor
    world.run_command_success(&[
        "character", "create", "boromir",
        "--set", "display_name=Boromir of Gondor",
        "--set", "race=Man",
        "--set", "realm=Gondor",
        "--set", "father=Denethor II",
        "--set", "title=Captain of the White Tower",
        "--set", "horn=Horn of Gondor",
        "--set", "status=Active"
    ]);
    
    // Get initial info
    let info = world.run_command_success(&["character", "info", "boromir"]);
    assert!(info.contains("Boromir of Gondor"));
    assert!(info.contains("Status: Active"));
    
    // Update after his fall and redemption
    world.run_command_success(&[
        "character", "update", "boromir",
        "--set", "fate=Died defending Merry and Pippin from Uruk-hai",
        "--set", "location_of_death=Amon Hen",
        "--set", "manner_of_death=Multiple orc arrows",
        "--set", "final_act=Attempted to save the hobbits",
        "--set", "status=Deceased"
    ]);
    
    // Verify the tragic update
    let updated_info = world.run_command_success(&["character", "info", "boromir"]);
    assert!(updated_info.contains("Status: Deceased"));
    assert!(updated_info.contains("Died defending Merry and Pippin"));
}

#[test]
fn test_sauron_and_evil_characters() {
    let world = TestWorld::new();
    
    // Create the Dark Lord
    world.run_command_success(&[
        "character", "create", "sauron",
        "--set", "display_name=Sauron the Dark Lord",
        "--set", "race=Maiar",
        "--set", "original_name=Mairon", 
        "--set", "realm=Mordor",
        "--set", "fortress=Barad-dûr",
        "--set", "weapon=The One Ring",
        "--set", "title=Lord of Mordor",
        "--set", "status=Active"
    ]);
    
    // Create the Witch-king
    world.run_command_success(&[
        "character", "create", "witch_king",
        "--set", "display_name=The Witch-king of Angmar",
        "--set", "race=Nazgûl",
        "--set", "original_race=Man",
        "--set", "realm=Angmar",
        "--set", "mount=Fell beast",
        "--set", "weapon=Morgul-blade",
        "--set", "status=Active"
    ]);
    
    // Create Saruman
    world.run_command_success(&[
        "character", "create", "saruman",
        "--set", "display_name=Saruman the White",
        "--set", "race=Maiar",
        "--set", "order=Istari",
        "--set", "color=White",
        "--set", "fortress=Isengard",
        "--set", "betrayal=Turned to evil",
        "--set", "status=Active"
    ]);
    
    let list_output = world.run_command_success(&["character", "list"]);
    assert!(list_output.contains("sauron - \"Sauron the Dark Lord\""));
    assert!(list_output.contains("witch_king - \"The Witch-king of Angmar\""));
    assert!(list_output.contains("saruman - \"Saruman the White\""));
}

#[test]
fn test_hobbit_characters() {
    let world = TestWorld::new();
    
    // Create the four hobbit members of the Fellowship
    world.run_command_success(&[
        "character", "create", "frodo",
        "--set", "display_name=Frodo Baggins",
        "--set", "race=Hobbit",
        "--set", "home=Bag End",
        "--set", "parents=Drogo and Primula Baggins",
        "--set", "guardian=Bilbo Baggins",
        "--set", "quest=Destroy the One Ring",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "sam",
        "--set", "display_name=Samwise Gamgee",
        "--set", "race=Hobbit", 
        "--set", "home=Hobbiton",
        "--set", "father=Hamfast Gamgee",
        "--set", "job=Gardener",
        "--set", "loyalty=Unwavering to Frodo",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "merry",
        "--set", "display_name=Meriadoc Brandybuck",
        "--set", "race=Hobbit",
        "--set", "home=Buckland",
        "--set", "family=Brandybuck",
        "--set", "nickname=Merry",
        "--set", "oath=Knight of Rohan",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "pippin",
        "--set", "display_name=Peregrin Took",
        "--set", "race=Hobbit",
        "--set", "home=Tookland",
        "--set", "family=Took",
        "--set", "nickname=Pippin",
        "--set", "oath=Guard of the Citadel",
        "--set", "status=Active"
    ]);
    
    // Verify all hobbits are created
    let list_output = world.run_command_success(&["character", "list"]);
    assert!(list_output.contains("frodo - \"Frodo Baggins\""));
    assert!(list_output.contains("sam - \"Samwise Gamgee\""));
    assert!(list_output.contains("merry - \"Meriadoc Brandybuck\""));
    assert!(list_output.contains("pippin - \"Peregrin Took\""));
}

#[test]
fn test_character_status_transitions() {
    let world = TestWorld::new();
    
    // Create Gandalf and track his transformation
    world.run_command_success(&[
        "character", "create", "gandalf",
        "--set", "display_name=Gandalf the Grey",
        "--set", "race=Maiar",
        "--set", "color=Grey",
        "--set", "status=Active"
    ]);
    
    // Gandalf "dies" fighting the Balrog
    world.run_command_success(&[
        "character", "update", "gandalf",
        "--set", "event=Fell fighting Balrog of Morgoth",
        "--set", "location=Bridge of Khazad-dûm",
        "--set", "last_words=Fly, you fools!",
        "--set", "status=Deceased"
    ]);
    
    let deceased_info = world.run_command_success(&["character", "info", "gandalf"]);
    assert!(deceased_info.contains("Status: Deceased"));
    
    // Gandalf returns as Gandalf the White
    world.run_command_success(&[
        "character", "update", "gandalf", 
        "--set", "display_name=Gandalf the White",
        "--set", "color=White",
        "--set", "transformation=Sent back as Gandalf the White",
        "--set", "enhanced_power=Yes",
        "--set", "status=Active"
    ]);
    
    let reborn_info = world.run_command_success(&["character", "info", "gandalf"]);
    assert!(reborn_info.contains("Gandalf the White"));
    assert!(reborn_info.contains("Status: Active"));
}

#[test]
fn test_complex_character_relationships() {
    let world = TestWorld::new();
    
    // Create Denethor and his sons
    world.run_command_success(&[
        "character", "create", "denethor",
        "--set", "display_name=Denethor II",
        "--set", "race=Man",
        "--set", "title=Ruling Steward of Gondor",
        "--set", "madness=Driven mad by Palantír",
        "--set", "sons=Boromir and Faramir",
        "--set", "status=Active"
    ]);
    
    world.run_command_success(&[
        "character", "create", "faramir",
        "--set", "display_name=Faramir, son of Denethor",
        "--set", "race=Man",
        "--set", "father=Denethor II",
        "--set", "brother=Boromir",
        "--set", "title=Captain of Ithilien Rangers",
        "--set", "resistance_to_ring=Strong",
        "--set", "status=Active"
    ]);
    
    // Test that both exist and have proper family connections
    let denethor_info = world.run_command_success(&["character", "info", "denethor"]);
    assert!(denethor_info.contains("sons: \"Boromir and Faramir\""));
    
    let faramir_info = world.run_command_success(&["character", "info", "faramir"]);
    assert!(faramir_info.contains("father: \"Denethor II\""));
    assert!(faramir_info.contains("brother: \"Boromir\""));
}

#[test]
fn test_middle_earth_full_workflow() {
    let world = TestWorld::new();
    
    // 1. Create the main protagonist
    world.run_command_success(&[
        "character", "create", "frodo",
        "--set", "display_name=Frodo Baggins of Bag End",
        "--set", "race=Hobbit",
        "--set", "age=50",
        "--set", "quest=Ring-bearer",
        "--set", "status=Active"
    ]);
    
    // 2. Create his loyal companion  
    world.run_command_success(&[
        "character", "create", "samwise",
        "--set", "display_name=Samwise Gamgee",
        "--set", "race=Hobbit",
        "--set", "relationship=Frodo's gardener and loyal friend",
        "--set", "courage=Greatest of all",
        "--set", "status=Active"
    ]);
    
    // 3. Verify they exist
    let list = world.run_command_success(&["character", "list"]);
    assert!(list.contains("frodo"));
    assert!(list.contains("samwise"));
    
    // 4. Update Frodo's journey progress
    world.run_command_success(&[
        "character", "update", "frodo",
        "--set", "current_location=Mount Doom",
        "--set", "ring_burden=Becoming overwhelming",
        "--set", "physical_state=Exhausted"
    ]);
    
    // 5. Complete the quest
    world.run_command_success(&[
        "character", "update", "frodo",
        "--set", "quest_status=Ring destroyed",
        "--set", "achievement=Saved Middle-earth",
        "--set", "consequence=Cannot find peace in Middle-earth"
    ]);
    
    // 6. Final departure
    world.run_command_success(&[
        "character", "update", "frodo",
        "--set", "final_journey=Sailed to Undying Lands",
        "--set", "departure_companions=Gandalf, Elrond, Galadriel, Bilbo",
        "--set", "status=Archived"
    ]);
    
    // 7. Verify the complete journey
    let final_info = world.run_command_success(&["character", "info", "frodo"]);
    assert!(final_info.contains("Status: Archived"));
    assert!(final_info.contains("Ring destroyed"));
    assert!(final_info.contains("Sailed to Undying Lands"));
}

#[test]
fn test_database_persistence_middle_earth() {
    let world = TestWorld::new();
    
    // Create a full cast of Middle-earth characters
    let characters = [
        ("aragorn", "Aragorn, King of Gondor", "Man"),
        ("legolas", "Legolas of the Woodland Realm", "Elf"),
        ("gimli", "Gimli the Dwarf", "Dwarf"),
        ("frodo", "Frodo Baggins", "Hobbit"),
        ("gandalf", "Gandalf the Grey", "Maiar"),
        ("sauron", "Sauron the Dark Lord", "Maiar"),
        ("galadriel", "Galadriel of Lothlórien", "Elf"),
        ("elrond", "Elrond Half-elven", "Half-elf")
    ];
    
    for (name, display_name, race) in characters.iter() {
        world.run_command_success(&[
            "character", "create", name,
            "--set", &format!("display_name={}", display_name),
            "--set", &format!("race={}", race),
            "--set", "status=Active"
        ]);
    }
    
    // Verify database exists and contains all characters
    assert!(world.database_exists());
    
    let list_output = world.run_command_success(&["character", "list"]);
    for (name, display_name, _) in characters.iter() {
        assert!(list_output.contains(name));
        assert!(list_output.contains(display_name));
    }
}