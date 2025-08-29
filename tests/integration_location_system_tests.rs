//! Integration tests for Location-System relations
//! Tests magical and technological infrastructure in locations

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
fn test_location_system_basic_infrastructure() {
    let world = TestWorld::new();
    
    // Create system first
    world.run_command_success(&[
        "system", "create", "magic_wards", 
        "--set", "display_name=Magical Ward System"
    ]);
    
    // Create location with system infrastructure (default type)
    let output = world.run_command_success(&[
        "location", "create", "minas_tirith", 
        "--set", "display_name=Minas Tirith",
        "--set", "system=magic_wards"
    ]);
    
    // Should show both location creation and relation creation
    assert!(output.contains("📍 Creating location 'minas_tirith'"));
    assert!(output.contains("🔗 Processed relations: system"));
    assert!(output.contains("✅ Created relation: minas_tirith -> magic_wards (has)"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, infrastructure_type FROM location_system_relations"
    ]);
    
    assert!(query_output.contains("minas_tirith"));
    assert!(query_output.contains("magic_wards"));
    assert!(query_output.contains("has"));
}

#[test]
fn test_location_system_magical_infrastructure_types() {
    let world = TestWorld::new();
    
    // Create multiple magical systems
    world.run_command_success(&["system", "create", "wards", "--set", "display_name=Protective Wards"]);
    world.run_command_success(&["system", "create", "teleportation", "--set", "display_name=Teleportation Network"]);
    world.run_command_success(&["system", "create", "scrying", "--set", "display_name=Scrying Network"]);
    world.run_command_success(&["system", "create", "healing", "--set", "display_name=Healing Springs"]);
    
    // Create magically advanced city with various infrastructure types
    let output = world.run_command_success(&[
        "location", "create", "rivendell", 
        "--set", "display_name=Rivendell",
        "--set", "system=wards*protected_by,teleportation*connected_to,scrying*monitored_by,healing*blessed_with"
    ]);
    
    // Should show relation processing
    assert!(output.contains("📍 Creating location 'rivendell'"));
    assert!(output.contains("🔗 Processed relations: system"));
    assert!(output.contains("✅ Created relation: rivendell -> wards (protected_by)"));
    assert!(output.contains("✅ Created relation: rivendell -> teleportation (connected_to)"));
    assert!(output.contains("✅ Created relation: rivendell -> scrying (monitored_by)"));
    assert!(output.contains("✅ Created relation: rivendell -> healing (blessed_with)"));
    
    // Verify all relations exist with correct infrastructure types
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, infrastructure_type FROM location_system_relations WHERE from_id = 'rivendell' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("protected_by"));
    assert!(query_output.contains("connected_to"));
    assert!(query_output.contains("monitored_by"));
    assert!(query_output.contains("blessed_with"));
    assert!(query_output.contains("📊 4 row(s) returned"));
}

#[test]
fn test_location_system_technological_infrastructure() {
    let world = TestWorld::new();
    
    // Create technological systems
    world.run_command_success(&["system", "create", "steam_power", "--set", "display_name=Steam Power Grid"]);
    world.run_command_success(&["system", "create", "clockwork", "--set", "display_name=Clockwork Automation"]);
    world.run_command_success(&["system", "create", "airship_docks", "--set", "display_name=Airship Docking System"]);
    world.run_command_success(&["system", "create", "communication", "--set", "display_name=Telegraph Network"]);
    
    // Create steampunk city
    let output = world.run_command_success(&[
        "location", "create", "brass_city", 
        "--set", "display_name=The Brass City",
        "--set", "system=steam_power*powered_by,clockwork*automated_by,airship_docks*served_by,communication*wired_with"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query technological infrastructure analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as technology, 
           r.infrastructure_type as integration_type
         FROM location_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'brass_city'
         ORDER BY s.display_name"
    ]);
    
    // Should show technological infrastructure
    assert!(query_output.contains("Steam Power Grid"));
    assert!(query_output.contains("Clockwork Automation"));
    assert!(query_output.contains("Airship Docking System"));
    assert!(query_output.contains("Telegraph Network"));
    assert!(query_output.contains("powered_by"));
    assert!(query_output.contains("automated_by"));
    assert!(query_output.contains("served_by"));
    assert!(query_output.contains("wired_with"));
}

#[test]
fn test_location_system_defensive_systems() {
    let world = TestWorld::new();
    
    // Create defensive systems
    world.run_command_success(&["system", "create", "barriers", "--set", "display_name=Magical Barriers"]);
    world.run_command_success(&["system", "create", "turrets", "--set", "display_name=Automated Turrets"]);
    world.run_command_success(&["system", "create", "early_warning", "--set", "display_name=Early Warning System"]);
    world.run_command_success(&["system", "create", "counterspells", "--set", "display_name=Counter-spell Matrix"]);
    
    // Create fortress with defensive infrastructure
    let output = world.run_command_success(&[
        "location", "create", "fortress", 
        "--set", "display_name=The Impregnable Fortress",
        "--set", "system=barriers*defended_by,turrets*armed_with,early_warning*watched_by,counterspells*shielded_by"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query defensive systems analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as defense_system,
           r.infrastructure_type as defense_type
         FROM location_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'fortress'
           AND r.infrastructure_type IN ('defended_by', 'armed_with', 'watched_by', 'shielded_by')
         ORDER BY s.display_name"
    ]);
    
    // Should show defensive infrastructure
    assert!(query_output.contains("defended_by"));
    assert!(query_output.contains("armed_with"));
    assert!(query_output.contains("watched_by"));
    assert!(query_output.contains("shielded_by"));
    assert!(query_output.contains("Magical Barriers"));
    assert!(query_output.contains("Automated Turrets"));
}

#[test]
fn test_location_system_validation() {
    let world = TestWorld::new();
    
    // Create location
    world.run_command_success(&["location", "create", "tower", "--set", "display_name=Wizard Tower"]);
    
    // Try to create relation with non-existent system
    let error = world.run_command_expect_failure(&[
        "location", "update", "tower", 
        "--set", "system=void_magic*corrupted_by"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("System 'void_magic' does not exist"));
    assert!(error.contains("multiverse system create void_magic"));
}

#[test]
fn test_location_system_research_facilities() {
    let world = TestWorld::new();
    
    // Create research systems
    world.run_command_success(&["system", "create", "alchemy_lab", "--set", "display_name=Alchemy Laboratory"]);
    world.run_command_success(&["system", "create", "library_magic", "--set", "display_name=Magical Library System"]);
    world.run_command_success(&["system", "create", "observatory", "--set", "display_name=Astronomical Observatory"]);
    world.run_command_success(&["system", "create", "testing_grounds", "--set", "display_name=Spell Testing Grounds"]);
    
    // Create magical research university
    let output = world.run_command_success(&[
        "location", "create", "arcane_university", 
        "--set", "display_name=Arcane University",
        "--set", "system=alchemy_lab*equipped_with,library_magic*archives_in,observatory*studies_with,testing_grounds*experiments_in"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query research infrastructure analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as facility,
           r.infrastructure_type as purpose
         FROM location_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'arcane_university'
         ORDER BY r.infrastructure_type"
    ]);
    
    // Should show research infrastructure
    assert!(query_output.contains("equipped_with"));
    assert!(query_output.contains("archives_in"));
    assert!(query_output.contains("studies_with"));
    assert!(query_output.contains("experiments_in"));
    assert!(query_output.contains("Alchemy Laboratory"));
    assert!(query_output.contains("Magical Library System"));
}

#[test]
fn test_location_system_corrupted_infrastructure() {
    let world = TestWorld::new();
    
    // Create corruption systems
    world.run_command_success(&["system", "create", "dark_magic", "--set", "display_name=Dark Magic Corruption"]);
    world.run_command_success(&["system", "create", "undeath", "--set", "display_name=Undeath Nexus"]);
    world.run_command_success(&["system", "create", "shadow", "--set", "display_name=Shadow Network"]);
    
    // Create corrupted location
    let output = world.run_command_success(&[
        "location", "create", "cursed_city", 
        "--set", "display_name=The Cursed City",
        "--set", "system=dark_magic*tainted_by,undeath*infested_with,shadow*consumed_by"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query corruption analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as corruption_source,
           r.infrastructure_type as corruption_type
         FROM location_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'cursed_city'
         ORDER BY s.display_name"
    ]);
    
    // Should show corruption infrastructure
    assert!(query_output.contains("tainted_by"));
    assert!(query_output.contains("infested_with"));
    assert!(query_output.contains("consumed_by"));
    assert!(query_output.contains("Dark Magic Corruption"));
    assert!(query_output.contains("Undeath Nexus"));
    assert!(query_output.contains("Shadow Network"));
}

#[test]
fn test_location_system_hybrid_magitech_city() {
    let world = TestWorld::new();
    
    // Create hybrid systems
    world.run_command_success(&["system", "create", "magitech_power", "--set", "display_name=Magitech Power Grid"]);
    world.run_command_success(&["system", "create", "enchanted_transport", "--set", "display_name=Enchanted Transportation"]);
    world.run_command_success(&["system", "create", "crystal_network", "--set", "display_name=Crystal Communication Network"]);
    world.run_command_success(&["system", "create", "weather_control", "--set", "display_name=Weather Control System"]);
    
    // Create advanced magitech city
    let output = world.run_command_success(&[
        "location", "create", "sky_city", 
        "--set", "display_name=Floating Sky City",
        "--set", "system=magitech_power*levitated_by,enchanted_transport*served_by,crystal_network*coordinated_by,weather_control*protected_by"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query magitech infrastructure analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           s.display_name as magitech_system,
           r.infrastructure_type as function_type
         FROM location_system_relations r
         JOIN systems s ON r.to_id = s.name
         WHERE r.from_id = 'sky_city'
         ORDER BY r.infrastructure_type"
    ]);
    
    // Should show magitech infrastructure
    assert!(query_output.contains("levitated_by"));
    assert!(query_output.contains("served_by"));
    assert!(query_output.contains("coordinated_by"));
    assert!(query_output.contains("protected_by"));
    assert!(query_output.contains("Magitech Power Grid"));
    assert!(query_output.contains("Enchanted Transportation"));
}

#[test]
fn test_location_system_infrastructure_upgrade() {
    let world = TestWorld::new();
    
    // Create upgrade systems
    world.run_command_success(&["system", "create", "basic_wards", "--set", "display_name=Basic Ward System"]);
    world.run_command_success(&["system", "create", "advanced_wards", "--set", "display_name=Advanced Ward Matrix"]);
    world.run_command_success(&["system", "create", "automation", "--set", "display_name=Magical Automation"]);
    
    // Create settlement with basic infrastructure
    world.run_command_success(&[
        "location", "create", "growing_town", 
        "--set", "display_name=Growing Town",
        "--set", "system=basic_wards*protected_by"
    ]);
    
    // Later, upgrade infrastructure
    let output = world.run_command_success(&[
        "location", "update", "growing_town", 
        "--set", "system=advanced_wards*upgraded_to,automation*enhanced_with"
    ]);
    
    assert!(output.contains("🔗 Processed relations: system"));
    
    // Query infrastructure evolution
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, infrastructure_type FROM location_system_relations WHERE from_id = 'growing_town' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("protected_by"));
    assert!(query_output.contains("upgraded_to"));
    assert!(query_output.contains("enhanced_with"));
    assert!(query_output.contains("📊 3 row(s) returned"));
}