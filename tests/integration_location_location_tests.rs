//! Integration tests for Location-Location relations
//! Tests geographical and political relationships between locations

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
fn test_location_location_basic_neighbor_relation() {
    let world = TestWorld::new();
    
    // Create two neighboring locations
    world.run_command_success(&[
        "location", "create", "rohan", 
        "--set", "display_name=Kingdom of Rohan"
    ]);
    
    world.run_command_success(&[
        "location", "create", "gondor", 
        "--set", "display_name=Kingdom of Gondor"
    ]);
    
    // Create neighbor relation (default relationship type)
    let output = world.run_command_success(&[
        "location", "update", "rohan", 
        "--set", "location=gondor"
    ]);
    
    // Should show relation was processed
    assert!(output.contains("🔗 Processed relations: location"));
    assert!(output.contains("✅ Created relation: rohan -> gondor (neighbor)"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, relationship_type FROM location_location_relations"
    ]);
    
    assert!(query_output.contains("rohan"));
    assert!(query_output.contains("gondor"));
    assert!(query_output.contains("neighbor"));
}

#[test]
fn test_location_location_specific_relation_types() {
    let world = TestWorld::new();
    
    // Create locations for different relationship types
    world.run_command_success(&["location", "create", "minas_tirith", "--set", "display_name=Minas Tirith"]);
    world.run_command_success(&["location", "create", "gondor", "--set", "display_name=Kingdom of Gondor"]);
    world.run_command_success(&["location", "create", "osgiliath", "--set", "display_name=Osgiliath"]);
    world.run_command_success(&["location", "create", "pelennor_fields", "--set", "display_name=Pelennor Fields"]);
    
    // Create various relation types
    let output = world.run_command_success(&[
        "location", "create", "citadel", 
        "--set", "display_name=Citadel of Minas Tirith",
        "--set", "location=minas_tirith*within,gondor*capital_of,osgiliath*overlooks,pelennor_fields*guards"
    ]);
    
    // Should show relation processing
    assert!(output.contains("📍 Creating location 'citadel'"));
    assert!(output.contains("🔗 Processed relations: location"));
    assert!(output.contains("✅ Created relation: citadel -> minas_tirith (within)"));
    assert!(output.contains("✅ Created relation: citadel -> gondor (capital_of)"));
    assert!(output.contains("✅ Created relation: citadel -> osgiliath (overlooks)"));
    assert!(output.contains("✅ Created relation: citadel -> pelennor_fields (guards)"));
    
    // Verify all relations exist with correct types
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, relationship_type FROM location_location_relations WHERE from_id = 'citadel' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("within"));
    assert!(query_output.contains("capital_of"));
    assert!(query_output.contains("overlooks"));
    assert!(query_output.contains("guards"));
    assert!(query_output.contains("📊 4 row(s) returned"));
}

#[test]
fn test_location_location_geographical_hierarchy() {
    let world = TestWorld::new();
    
    // Create geographical hierarchy: continent -> kingdom -> region -> city
    world.run_command_success(&["location", "create", "middle_earth", "--set", "display_name=Middle-earth"]);
    world.run_command_success(&["location", "create", "gondor", "--set", "display_name=Kingdom of Gondor"]);  
    world.run_command_success(&["location", "create", "ithilien", "--set", "display_name=Ithilien"]);
    
    // Create city within region, region within kingdom, kingdom within continent
    world.run_command_success(&[
        "location", "create", "osgiliath", 
        "--set", "display_name=Osgiliath",
        "--set", "location=ithilien*within"
    ]);
    
    world.run_command_success(&[
        "location", "update", "ithilien", 
        "--set", "location=gondor*within"
    ]);
    
    world.run_command_success(&[
        "location", "update", "gondor", 
        "--set", "location=middle_earth*within"
    ]);
    
    // Query the geographical hierarchy
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l1.display_name as location_name, 
           l2.display_name as parent_name, 
           r.relationship_type
         FROM location_location_relations r
         JOIN locations l1 ON r.from_id = l1.name
         JOIN locations l2 ON r.to_id = l2.name  
         WHERE r.relationship_type = 'within'
         ORDER BY l1.display_name"
    ]);
    
    // Should show hierarchical containment
    assert!(query_output.contains("Gondor")); 
    assert!(query_output.contains("Middle-earth"));
    assert!(query_output.contains("Ithilien"));
    assert!(query_output.contains("Osgiliath"));
    assert!(query_output.contains("within"));
}

#[test]
fn test_location_location_borders_and_neighbors() {
    let world = TestWorld::new();
    
    // Create neighboring kingdoms
    world.run_command_success(&["location", "create", "rohan", "--set", "display_name=Kingdom of Rohan"]);
    world.run_command_success(&["location", "create", "gondor", "--set", "display_name=Kingdom of Gondor"]);
    world.run_command_success(&["location", "create", "mordor", "--set", "display_name=Mordor"]);
    world.run_command_success(&["location", "create", "isengard", "--set", "display_name=Isengard"]);
    
    // Create border relationships
    let output = world.run_command_success(&[
        "location", "update", "rohan", 
        "--set", "location=gondor*ally_neighbor,isengard*hostile_border,mordor*enemy_territory"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query neighbor analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           r.relationship_type,
           l.display_name as neighboring_location
         FROM location_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'rohan'
         ORDER BY r.relationship_type"
    ]);
    
    // Should show different types of neighboring relationships
    assert!(query_output.contains("ally_neighbor"));
    assert!(query_output.contains("hostile_border"));
    assert!(query_output.contains("enemy_territory"));
    assert!(query_output.contains("Kingdom of Gondor"));
    assert!(query_output.contains("Isengard"));
    assert!(query_output.contains("Mordor"));
}

#[test]
fn test_location_location_validation() {
    let world = TestWorld::new();
    
    // Create one location
    world.run_command_success(&["location", "create", "gondor", "--set", "display_name=Kingdom of Gondor"]);
    
    // Try to create relation with non-existent location
    let error = world.run_command_expect_failure(&[
        "location", "update", "gondor", 
        "--set", "location=atlantis*neighbor"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Location 'atlantis' does not exist"));
    assert!(error.contains("multiverse location create atlantis"));
}

#[test]
fn test_location_location_trade_routes() {
    let world = TestWorld::new();
    
    // Create trade network
    world.run_command_success(&["location", "create", "minas_tirith", "--set", "display_name=Minas Tirith"]);
    world.run_command_success(&["location", "create", "dol_amroth", "--set", "display_name=Dol Amroth"]);
    world.run_command_success(&["location", "create", "pelargir", "--set", "display_name=Pelargir"]);
    world.run_command_success(&["location", "create", "osgiliath", "--set", "display_name=Osgiliath"]);
    
    // Create trade route network
    let output = world.run_command_success(&[
        "location", "create", "great_west_road", 
        "--set", "display_name=Great West Road",
        "--set", "location=minas_tirith*connects,dol_amroth*connects,pelargir*trade_route,osgiliath*passes_through"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query trade network analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as connected_city,
           r.relationship_type as connection_type
         FROM location_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'great_west_road'
           AND r.relationship_type IN ('connects', 'trade_route', 'passes_through')
         ORDER BY l.display_name"
    ]);
    
    // Should show trade connections
    assert!(query_output.contains("connects"));
    assert!(query_output.contains("trade_route"));
    assert!(query_output.contains("passes_through"));
    assert!(query_output.contains("Minas Tirith"));
    assert!(query_output.contains("Dol Amroth"));
    assert!(query_output.contains("Pelargir"));
}

#[test]
fn test_location_location_defensive_positions() {
    let world = TestWorld::new();
    
    // Create defensive network
    world.run_command_success(&["location", "create", "minas_tirith", "--set", "display_name=Minas Tirith"]);
    world.run_command_success(&["location", "create", "osgiliath", "--set", "display_name=Osgiliath"]);
    world.run_command_success(&["location", "create", "amon_din", "--set", "display_name=Amon Dîn"]);
    world.run_command_success(&["location", "create", "dol_amroth", "--set", "display_name=Dol Amroth"]);
    
    // Create defensive position relationships  
    let output = world.run_command_success(&[
        "location", "create", "beacon_network", 
        "--set", "display_name=Beacon Network of Gondor",
        "--set", "location=minas_tirith*defends,osgiliath*guards,amon_din*watchtower,dol_amroth*coastal_defense"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query defensive analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as defended_location,
           r.relationship_type as defense_type
         FROM location_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'beacon_network'
           AND r.relationship_type IN ('defends', 'guards', 'watchtower', 'coastal_defense')
         ORDER BY r.relationship_type"
    ]);
    
    // Should show defensive relationships
    assert!(query_output.contains("defends"));
    assert!(query_output.contains("guards")); 
    assert!(query_output.contains("watchtower"));
    assert!(query_output.contains("coastal_defense"));
}

#[test]
fn test_location_location_update_relations() {
    let world = TestWorld::new();
    
    // Create locations
    world.run_command_success(&["location", "create", "isengard", "--set", "display_name=Isengard"]);
    world.run_command_success(&["location", "create", "saruman_tower", "--set", "display_name=Tower of Saruman"]);
    world.run_command_success(&["location", "create", "fangorn", "--set", "display_name=Fangorn Forest"]);
    
    // Initially, Isengard dominates surrounding area
    world.run_command_success(&[
        "location", "update", "isengard", 
        "--set", "location=saruman_tower*controls,fangorn*threatens"
    ]);
    
    // Later, create a new relation to show relationship evolution
    world.run_command_success(&["location", "create", "ent_watch", "--set", "display_name=Ent Watch Post"]);
    
    let output = world.run_command_success(&[
        "location", "update", "isengard", 
        "--set", "location=ent_watch*surrounded_by"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query should show all relationships
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, relationship_type FROM location_location_relations WHERE from_id = 'isengard' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("controls"));
    assert!(query_output.contains("threatens"));
    assert!(query_output.contains("surrounded_by"));
}