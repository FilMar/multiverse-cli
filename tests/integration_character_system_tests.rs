//! Integration tests for Character-System relations
//! Tests who uses what magical or technological systems and how

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
fn test_character_system_basic_usage() {
    let world = TestWorld::new();
    
    // Create system first
    world.run_command_success(&[
        "system", "create", "wizardry", 
        "--set", "display_name=Art of Wizardry"
    ]);
    
    // Create character with system relation (default usage type)
    let output = world.run_command_success(&[
        "character", "create", "gandalf", 
        "--set", "display_name=Gandalf the Grey",
        "--set", "system=wizardry"
    ]);
    
    // Should show both character creation and relation creation
    assert!(output.contains("👤 Creating character 'gandalf'"));
    assert!(output.contains("🔗 Processed relations: system"));
    assert!(output.contains("✅ Created relation: gandalf -> wizardry (uses)"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT c.name as character_name, s.name as system_name, cr.usage_type FROM character_system_relations cr JOIN characters c ON cr.from_id = c.id JOIN systems s ON cr.to_id = s.id"
    ]);
    
    assert!(query_output.contains("gandalf"));
    assert!(query_output.contains("wizardry"));
    assert!(query_output.contains("uses"));
}

#[test]
fn test_character_system_specific_usage_types() {
    let world = TestWorld::new();
    
    // Create multiple systems
    world.run_command_success(&["system", "create", "magic", "--set", "display_name=Magic System"]);
    world.run_command_success(&["system", "create", "runes", "--set", "display_name=Runic Magic"]);
    world.run_command_success(&["system", "create", "alchemy", "--set", "display_name=Alchemy"]);
    world.run_command_success(&["system", "create", "healing", "--set", "display_name=Healing Arts"]);
    
    // Create character with multiple system usages with different types
    let output = world.run_command_success(&[
        "character", "create", "saruman", 
        "--set", "display_name=Saruman the White",
        "--set", "system=magic*mastery,runes*studies,alchemy*experiments,healing*forbidden"
    ]);
    
    // Should show relation processing
    assert!(output.contains("👤 Creating character 'saruman'"));
    assert!(output.contains("🔗 Processed relations: system"));
    assert!(output.contains("✅ Created relation: saruman -> magic (mastery)"));
    assert!(output.contains("✅ Created relation: saruman -> runes (studies)"));
    assert!(output.contains("✅ Created relation: saruman -> alchemy (experiments)"));
    assert!(output.contains("✅ Created relation: saruman -> healing (forbidden)"));
    
    // Verify all relations exist with correct usage types
    let query_output = world.run_command_success(&[
        "query",
        "SELECT s.name as system_name, cr.usage_type FROM character_system_relations cr JOIN characters c ON cr.from_id = c.id JOIN systems s ON cr.to_id = s.id WHERE c.name = 'saruman' ORDER BY s.name"
    ]);
    
    assert!(query_output.contains("mastery"));
    assert!(query_output.contains("studies"));
    assert!(query_output.contains("experiments"));
    assert!(query_output.contains("forbidden"));
    assert!(query_output.contains("📊 4 row(s) returned"));
}

#[test]
fn test_character_system_magical_specializations() {
    let world = TestWorld::new();
    
    // Create magical systems
    world.run_command_success(&["system", "create", "fire_magic", "--set", "display_name=Fire Magic"]);
    world.run_command_success(&["system", "create", "water_magic", "--set", "display_name=Water Magic"]);
    world.run_command_success(&["system", "create", "earth_magic", "--set", "display_name=Earth Magic"]);
    world.run_command_success(&["system", "create", "air_magic", "--set", "display_name=Air Magic"]);
    
    // Create elemental mages with different specializations
    world.run_command_success(&[
        "character", "create", "pyromancer", 
        "--set", "display_name=Master Pyromancer",
        "--set", "system=fire_magic*specializes"
    ]);
    
    world.run_command_success(&[
        "character", "create", "avatar", 
        "--set", "display_name=The Avatar",
        "--set", "system=fire_magic*mastery,water_magic*mastery,earth_magic*mastery,air_magic*mastery"
    ]);
    
    // Query elemental magic specialization analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           c.display_name as character_name, 
           s.display_name as system_name, 
           r.usage_type
         FROM character_system_relations r
         JOIN characters c ON r.from_id = c.name
         JOIN systems s ON r.to_id = s.name
         WHERE s.name LIKE '%_magic'
         ORDER BY c.display_name, s.display_name"
    ]);
    
    // Should show specialization data
    assert!(query_output.contains("Master Pyromancer"));
    assert!(query_output.contains("The Avatar"));
    assert!(query_output.contains("Fire Magic"));
    assert!(query_output.contains("Water Magic"));
    assert!(query_output.contains("specializes"));
    assert!(query_output.contains("mastery"));
}

#[test]
fn test_character_system_technological_systems() {
    let world = TestWorld::new();
    
    // Create technological systems
    world.run_command_success(&["system", "create", "steampunk", "--set", "display_name=Steam Technology"]);
    world.run_command_success(&["system", "create", "clockwork", "--set", "display_name=Clockwork Mechanisms"]);
    world.run_command_success(&["system", "create", "airships", "--set", "display_name=Airship Engineering"]);
    
    // Create engineer character
    let output = world.run_command_success(&[
        "character", "create", "inventor", 
        "--set", "display_name=Master Inventor",
        "--set", "system=steampunk*invents,clockwork*repairs,airships*pilots"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query technological expertise analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as technology,
           r.usage_type as expertise_level
         FROM character_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'inventor'
         ORDER BY s.display_name"
    ]);
    
    // Should show technological relationships
    assert!(query_output.contains("Steam Technology"));
    assert!(query_output.contains("Clockwork Mechanisms"));
    assert!(query_output.contains("Airship Engineering"));
    assert!(query_output.contains("invents"));
    assert!(query_output.contains("repairs"));
    assert!(query_output.contains("pilots"));
}

#[test]
fn test_character_system_validation() {
    let world = TestWorld::new();
    
    // Create character
    world.run_command_success(&["character", "create", "wizard", "--set", "display_name=Young Wizard"]);
    
    // Try to create relation with non-existent system
    let error = world.run_command_expect_failure(&[
        "character", "update", "wizard", 
        "--set", "system=necromancy*learns"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("System not found: 'necromancy'"));
}

#[test]
fn test_character_system_learning_progression() {
    let world = TestWorld::new();
    
    // Create learning systems
    world.run_command_success(&["system", "create", "swordsmanship", "--set", "display_name=Art of Swordsmanship"]);
    world.run_command_success(&["system", "create", "archery", "--set", "display_name=Archery"]);
    world.run_command_success(&["system", "create", "tactics", "--set", "display_name=Military Tactics"]);
    
    // Create warrior with initial skills
    world.run_command_success(&[
        "character", "create", "aragorn", 
        "--set", "display_name=Aragorn",
        "--set", "system=swordsmanship*practices"
    ]);
    
    // Later, gain more skills
    let output = world.run_command_success(&[
        "character", "update", "aragorn", 
        "--set", "system=archery*learns,tactics*studies"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query skill progression
    let query_output = world.run_command_success(&[
        "query",
        "SELECT s.name as system_name, cr.usage_type FROM character_system_relations cr JOIN characters c ON cr.from_id = c.id JOIN systems s ON cr.to_id = s.id WHERE c.name = 'aragorn' ORDER BY s.name"
    ]);
    
    assert!(query_output.contains("practices"));
    assert!(query_output.contains("learns"));
    assert!(query_output.contains("studies"));
    assert!(query_output.contains("📊 3 row(s) returned"));
}

#[test]
fn test_character_system_forbidden_arts() {
    let world = TestWorld::new();
    
    // Create forbidden magical systems
    world.run_command_success(&["system", "create", "necromancy", "--set", "display_name=Necromancy"]);
    world.run_command_success(&["system", "create", "shadow_magic", "--set", "display_name=Shadow Magic"]);
    world.run_command_success(&["system", "create", "soul_magic", "--set", "display_name=Soul Magic"]);
    
    // Create dark wizard character
    let output = world.run_command_success(&[
        "character", "create", "dark_lord", 
        "--set", "display_name=The Dark Lord",
        "--set", "system=necromancy*mastery,shadow_magic*corrupted_by,soul_magic*forbidden_knowledge"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query forbidden arts analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as forbidden_art,
           r.usage_type as relationship
         FROM character_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'dark_lord'
           AND r.usage_type IN ('mastery', 'corrupted_by', 'forbidden_knowledge')
         ORDER BY s.display_name"
    ]);
    
    // Should show forbidden relationships
    assert!(query_output.contains("mastery"));
    assert!(query_output.contains("corrupted_by"));
    assert!(query_output.contains("forbidden_knowledge"));
    assert!(query_output.contains("Necromancy"));
    assert!(query_output.contains("Shadow Magic"));
    assert!(query_output.contains("Soul Magic"));
}

#[test]
fn test_character_system_hybrid_magic_tech() {
    let world = TestWorld::new();
    
    // Create hybrid systems
    world.run_command_success(&["system", "create", "magitech", "--set", "display_name=Magical Technology"]);
    world.run_command_success(&["system", "create", "enchanting", "--set", "display_name=Item Enchanting"]);
    world.run_command_success(&["system", "create", "artifice", "--set", "display_name=Magical Artifice"]);
    
    // Create magitech engineer
    let output = world.run_command_success(&[
        "character", "create", "artificer", 
        "--set", "display_name=Master Artificer",
        "--set", "system=magitech*innovates,enchanting*enchants,artifice*creates"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query hybrid expertise analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as hybrid_system,
           r.usage_type as expertise_type
         FROM character_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'artificer'
         ORDER BY r.usage_type"
    ]);
    
    // Should show hybrid relationships
    assert!(query_output.contains("innovates"));
    assert!(query_output.contains("enchants"));
    assert!(query_output.contains("creates"));
    assert!(query_output.contains("Magical Technology"));
    assert!(query_output.contains("Item Enchanting"));
    assert!(query_output.contains("Magical Artifice"));
}

#[test]
fn test_character_system_incompatible_systems() {
    let world = TestWorld::new();
    
    // Create opposing systems
    world.run_command_success(&["system", "create", "holy_magic", "--set", "display_name=Holy Magic"]);
    world.run_command_success(&["system", "create", "dark_magic", "--set", "display_name=Dark Magic"]);
    world.run_command_success(&["system", "create", "nature_magic", "--set", "display_name=Nature Magic"]);
    
    // Create conflicted character who struggles with opposing forces
    let output = world.run_command_success(&[
        "character", "create", "torn_mage", 
        "--set", "display_name=The Torn Mage",
        "--set", "system=holy_magic*trained_in,dark_magic*tempted_by,nature_magic*seeks_balance"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query conflicting systems analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as magic_type,
           r.usage_type as relationship_nature
         FROM character_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'torn_mage'
         ORDER BY s.display_name"
    ]);
    
    // Should show conflicting relationships
    assert!(query_output.contains("trained_in"));
    assert!(query_output.contains("tempted_by"));
    assert!(query_output.contains("seeks_balance"));
    assert!(query_output.contains("Holy Magic"));
    assert!(query_output.contains("Dark Magic"));
    assert!(query_output.contains("Nature Magic"));
}