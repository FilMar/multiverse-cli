//! Integration tests for Event-Character relations
//! Tests participation of characters in events

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
fn test_event_character_basic_participation() {
    let world = TestWorld::new();
    
    // Create character first
    world.run_command_success(&[
        "character", "create", "aragorn", 
        "--set", "display_name=Aragorn, Son of Arathorn"
    ]);
    
    // Create event with character participation (default type)
    let output = world.run_command_success(&[
        "event", "create", "battle_helms_deep", 
        "--set", "display_name=Battle of Helm's Deep",
        "--set", "character=aragorn"
    ]);
    
    // Should show both event creation and relation creation
    assert!(output.contains("📅 Creating event 'battle_helms_deep'"));
    assert!(output.contains("🔗 Processed relations: character"));
    assert!(output.contains("✅ Created relation: battle_helms_deep -> aragorn (participant)"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, participation_type FROM event_character_relations"
    ]);
    
    assert!(query_output.contains("battle_helms_deep"));
    assert!(query_output.contains("aragorn"));
    assert!(query_output.contains("participant"));
}

#[test]
fn test_event_character_specific_participation_types() {
    let world = TestWorld::new();
    
    // Create multiple characters
    world.run_command_success(&["character", "create", "frodo", "--set", "display_name=Frodo Baggins"]);
    world.run_command_success(&["character", "create", "gandalf", "--set", "display_name=Gandalf the Grey"]);
    world.run_command_success(&["character", "create", "sauron", "--set", "display_name=The Dark Lord Sauron"]);
    world.run_command_success(&["character", "create", "sam", "--set", "display_name=Samwise Gamgee"]);
    
    // Create major event with various participation types
    let output = world.run_command_success(&[
        "event", "create", "destruction_ring", 
        "--set", "display_name=Destruction of the One Ring",
        "--set", "character=frodo*protagonist,gandalf*mentor,sauron*antagonist,sam*supporter"
    ]);
    
    // Should show relation processing
    assert!(output.contains("📅 Creating event 'destruction_ring'"));
    assert!(output.contains("🔗 Processed relations: character"));
    assert!(output.contains("✅ Created relation: destruction_ring -> frodo (protagonist)"));
    assert!(output.contains("✅ Created relation: destruction_ring -> gandalf (mentor)"));
    assert!(output.contains("✅ Created relation: destruction_ring -> sauron (antagonist)"));
    assert!(output.contains("✅ Created relation: destruction_ring -> sam (supporter)"));
    
    // Verify all relations exist with correct participation types
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, participation_type FROM event_character_relations WHERE from_id = 'destruction_ring' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("protagonist"));
    assert!(query_output.contains("mentor"));
    assert!(query_output.contains("antagonist"));
    assert!(query_output.contains("supporter"));
    assert!(query_output.contains("📊 4 row(s) returned"));
}

#[test]
fn test_event_character_battle_participants() {
    let world = TestWorld::new();
    
    // Create warriors
    world.run_command_success(&["character", "create", "legolas", "--set", "display_name=Legolas Greenleaf"]);
    world.run_command_success(&["character", "create", "gimli", "--set", "display_name=Gimli son of Gloin"]);
    world.run_command_success(&["character", "create", "boromir", "--set", "display_name=Boromir of Gondor"]);
    world.run_command_success(&["character", "create", "lurtz", "--set", "display_name=Lurtz"]);
    
    // Create battle event with different combat roles
    let output = world.run_command_success(&[
        "event", "create", "battle_amon_hen", 
        "--set", "display_name=Battle at Amon Hen",
        "--set", "character=legolas*archer,gimli*warrior,boromir*defender,lurtz*enemy"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query battle participation analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as character_name, 
           r.participation_type as role
         FROM event_character_relations r
         JOIN characters c ON r.to_id = c.name
         WHERE r.from_id = 'battle_amon_hen'
         ORDER BY c.display_name"
    ]);
    
    // Should show battle roles
    assert!(query_output.contains("Legolas Greenleaf"));
    assert!(query_output.contains("Gimli son of Gloin"));
    assert!(query_output.contains("Boromir of Gondor"));
    assert!(query_output.contains("Lurtz"));
    assert!(query_output.contains("archer"));
    assert!(query_output.contains("warrior"));
    assert!(query_output.contains("defender"));
    assert!(query_output.contains("enemy"));
}

#[test]
fn test_event_character_validation() {
    let world = TestWorld::new();
    
    // Try to create event with non-existent character
    let error = world.run_command_expect_failure(&[
        "event", "create", "meeting", 
        "--set", "display_name=Council Meeting",
        "--set", "character=elrond*host"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Character 'elrond' does not exist"));
    assert!(error.contains("multiverse character create elrond"));
}

#[test]
fn test_event_character_political_event() {
    let world = TestWorld::new();
    
    // Create political characters
    world.run_command_success(&["character", "create", "theoden", "--set", "display_name=King Théoden"]);
    world.run_command_success(&["character", "create", "denethor", "--set", "display_name=Denethor II"]);
    world.run_command_success(&["character", "create", "elrond", "--set", "display_name=Lord Elrond"]);
    world.run_command_success(&["character", "create", "gandalf_w", "--set", "display_name=Gandalf the White"]);
    
    // Create council event with political roles
    let output = world.run_command_success(&[
        "event", "create", "last_debate", 
        "--set", "display_name=The Last Debate",
        "--set", "character=theoden*king,denethor*steward,elrond*lord,gandalf_w*advisor"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query political roles analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as leader,
           r.participation_type as political_role
         FROM event_character_relations r
         JOIN characters c ON r.to_id = c.name
         WHERE r.from_id = 'last_debate'
           AND r.participation_type IN ('king', 'steward', 'lord', 'advisor')
         ORDER BY r.participation_type"
    ]);
    
    // Should show political roles
    assert!(query_output.contains("king"));
    assert!(query_output.contains("steward"));
    assert!(query_output.contains("lord"));
    assert!(query_output.contains("advisor"));
    assert!(query_output.contains("King Théoden"));
    assert!(query_output.contains("Denethor II"));
}

#[test]
fn test_event_character_tragic_events() {
    let world = TestWorld::new();
    
    // Create characters for tragic event
    world.run_command_success(&["character", "create", "boromir_t", "--set", "display_name=Boromir"]);
    world.run_command_success(&["character", "create", "aragorn_t", "--set", "display_name=Aragorn"]);
    world.run_command_success(&["character", "create", "merry", "--set", "display_name=Meriadoc Brandybuck"]);
    world.run_command_success(&["character", "create", "pippin", "--set", "display_name=Peregrin Took"]);
    
    // Create tragic event with emotional roles
    let output = world.run_command_success(&[
        "event", "create", "death_boromir", 
        "--set", "display_name=Death of Boromir",
        "--set", "character=boromir_t*victim,aragorn_t*mourner,merry*rescued,pippin*rescued"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query tragic event analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as character_name,
           r.participation_type as emotional_role
         FROM event_character_relations r
         JOIN characters c ON r.to_id = c.name
         WHERE r.from_id = 'death_boromir'
         ORDER BY r.participation_type"
    ]);
    
    // Should show emotional roles
    assert!(query_output.contains("victim"));
    assert!(query_output.contains("mourner"));
    assert!(query_output.contains("rescued"));
    assert!(query_output.contains("Boromir"));
    assert!(query_output.contains("Aragorn"));
}

#[test]
fn test_event_character_ceremonial_event() {
    let world = TestWorld::new();
    
    // Create royal characters
    world.run_command_success(&["character", "create", "aragorn_k", "--set", "display_name=King Elessar"]);
    world.run_command_success(&["character", "create", "arwen", "--set", "display_name=Queen Arwen"]);
    world.run_command_success(&["character", "create", "faramir", "--set", "display_name=Prince Faramir"]);
    world.run_command_success(&["character", "create", "eowyn", "--set", "display_name=Lady Éowyn"]);
    
    // Create coronation ceremony with ceremonial roles
    let output = world.run_command_success(&[
        "event", "create", "coronation", 
        "--set", "display_name=Coronation of King Elessar",
        "--set", "character=aragorn_k*crowned,arwen*bride,faramir*prince,eowyn*witness"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query ceremonial roles analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as noble,
           r.participation_type as ceremonial_role
         FROM event_character_relations r
         JOIN characters c ON r.to_id = c.name
         WHERE r.from_id = 'coronation'
         ORDER BY c.display_name"
    ]);
    
    // Should show ceremonial roles
    assert!(query_output.contains("crowned"));
    assert!(query_output.contains("bride"));
    assert!(query_output.contains("prince"));
    assert!(query_output.contains("witness"));
    assert!(query_output.contains("King Elessar"));
    assert!(query_output.contains("Queen Arwen"));
}

#[test]
fn test_event_character_update_participation() {
    let world = TestWorld::new();
    
    // Create characters
    world.run_command_success(&["character", "create", "gandalf_g", "--set", "display_name=Gandalf the Grey"]);
    world.run_command_success(&["character", "create", "saruman", "--set", "display_name=Saruman the White"]);
    world.run_command_success(&["character", "create", "radagast", "--set", "display_name=Radagast the Brown"]);
    
    // Create initial council event
    world.run_command_success(&[
        "event", "create", "white_council", 
        "--set", "display_name=Meeting of the White Council",
        "--set", "character=gandalf_g*member,saruman*leader"
    ]);
    
    // Later, add more participants
    let output = world.run_command_success(&[
        "event", "update", "white_council", 
        "--set", "character=radagast*member"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query updated participation
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, participation_type FROM event_character_relations WHERE from_id = 'white_council' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("gandalf_g"));
    assert!(query_output.contains("saruman"));
    assert!(query_output.contains("radagast"));
    assert!(query_output.contains("member"));
    assert!(query_output.contains("leader"));
    assert!(query_output.contains("📊 3 row(s) returned"));
}

#[test]
fn test_event_character_fellowship_formation() {
    let world = TestWorld::new();
    
    // Create Fellowship members
    world.run_command_success(&["character", "create", "frodo_f", "--set", "display_name=Frodo Baggins"]);
    world.run_command_success(&["character", "create", "sam_f", "--set", "display_name=Sam Gamgee"]);
    world.run_command_success(&["character", "create", "gandalf_f", "--set", "display_name=Gandalf"]);
    world.run_command_success(&["character", "create", "aragorn_f", "--set", "display_name=Strider"]);
    world.run_command_success(&["character", "create", "legolas_f", "--set", "display_name=Legolas"]);
    world.run_command_success(&["character", "create", "gimli_f", "--set", "display_name=Gimli"]);
    world.run_command_success(&["character", "create", "boromir_f", "--set", "display_name=Boromir"]);
    world.run_command_success(&["character", "create", "merry_f", "--set", "display_name=Merry"]);
    world.run_command_success(&["character", "create", "pippin_f", "--set", "display_name=Pippin"]);
    
    // Create Fellowship formation event
    let output = world.run_command_success(&[
        "event", "create", "fellowship_formed", 
        "--set", "display_name=Formation of the Fellowship",
        "--set", "character=frodo_f*ring_bearer,sam_f*gardener,gandalf_f*guide,aragorn_f*ranger,legolas_f*elf,gimli_f*dwarf,boromir_f*gondorian,merry_f*hobbit,pippin_f*hobbit"
    ]);
    
    assert!(output.contains("🔗 Processed relations: character"));
    
    // Query Fellowship composition
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as fellowship_member,
           r.participation_type as role_in_fellowship
         FROM event_character_relations r
         JOIN characters c ON r.to_id = c.name
         WHERE r.from_id = 'fellowship_formed'
         ORDER BY r.participation_type, c.display_name"
    ]);
    
    // Should show all Fellowship roles
    assert!(query_output.contains("ring_bearer"));
    assert!(query_output.contains("gardener"));
    assert!(query_output.contains("guide"));
    assert!(query_output.contains("ranger"));
    assert!(query_output.contains("elf"));
    assert!(query_output.contains("dwarf"));
    assert!(query_output.contains("gondorian"));
    assert!(query_output.contains("hobbit"));
    assert!(query_output.contains("📊 9 row(s) returned"));
}