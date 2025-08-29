//! Integration tests for Event-Location relations
//! Tests where events take place and their geographical context

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
fn test_event_location_basic_venue() {
    let world = TestWorld::new();
    
    // Create location first
    world.run_command_success(&[
        "location", "create", "helms_deep", 
        "--set", "display_name=Helm's Deep"
    ]);
    
    // Create event with location (default role)
    let output = world.run_command_success(&[
        "event", "create", "battle_helms_deep", 
        "--set", "display_name=Battle of Helm's Deep",
        "--set", "location=helms_deep"
    ]);
    
    // Should show both event creation and relation creation
    assert!(output.contains("📅 Creating event 'battle_helms_deep'"));
    assert!(output.contains("🔗 Processed relations: location"));
    assert!(output.contains("✅ Created relation: battle_helms_deep -> helms_deep (takes_place_at)"));
    
    // Query the relation using the query command
    let query_output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, location_role FROM event_location_relations"
    ]);
    
    assert!(query_output.contains("battle_helms_deep"));
    assert!(query_output.contains("helms_deep"));
    assert!(query_output.contains("takes_place_at"));
}

#[test]
fn test_event_location_specific_venue_roles() {
    let world = TestWorld::new();
    
    // Create multiple locations
    world.run_command_success(&["location", "create", "minas_tirith", "--set", "display_name=Minas Tirith"]);
    world.run_command_success(&["location", "create", "pelennor", "--set", "display_name=Pelennor Fields"]);
    world.run_command_success(&["location", "create", "osgiliath", "--set", "display_name=Osgiliath"]);
    world.run_command_success(&["location", "create", "anduin", "--set", "display_name=River Anduin"]);
    
    // Create multi-location battle event with different venue roles
    let output = world.run_command_success(&[
        "event", "create", "siege_minas_tirith", 
        "--set", "display_name=Siege of Minas Tirith",
        "--set", "location=minas_tirith*besieged,pelennor*battlefield,osgiliath*staging_area,anduin*supply_route"
    ]);
    
    // Should show relation processing
    assert!(output.contains("📅 Creating event 'siege_minas_tirith'"));
    assert!(output.contains("🔗 Processed relations: location"));
    assert!(output.contains("✅ Created relation: siege_minas_tirith -> minas_tirith (besieged)"));
    assert!(output.contains("✅ Created relation: siege_minas_tirith -> pelennor (battlefield)"));
    assert!(output.contains("✅ Created relation: siege_minas_tirith -> osgiliath (staging_area)"));
    assert!(output.contains("✅ Created relation: siege_minas_tirith -> anduin (supply_route)"));
    
    // Verify all relations exist with correct venue roles
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, location_role FROM event_location_relations WHERE from_id = 'siege_minas_tirith' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("besieged"));
    assert!(query_output.contains("battlefield"));
    assert!(query_output.contains("staging_area"));
    assert!(query_output.contains("supply_route"));
    assert!(query_output.contains("📊 4 row(s) returned"));
}

#[test]
fn test_event_location_ceremonial_venues() {
    let world = TestWorld::new();
    
    // Create ceremonial locations
    world.run_command_success(&["location", "create", "citadel", "--set", "display_name=Citadel of Minas Tirith"]);
    world.run_command_success(&["location", "create", "court_kings", "--set", "display_name=Court of the Kings"]);
    world.run_command_success(&["location", "create", "houses_healing", "--set", "display_name=Houses of Healing"]);
    world.run_command_success(&["location", "create", "white_tree", "--set", "display_name=Court of the White Tree"]);
    
    // Create coronation ceremony with ceremonial venue roles
    let output = world.run_command_success(&[
        "event", "create", "coronation_elessar", 
        "--set", "display_name=Coronation of King Elessar",
        "--set", "location=citadel*ceremonial_site,court_kings*throne_room,houses_healing*preparation,white_tree*blessing"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query ceremonial venues analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as venue, 
           r.location_role as ceremonial_purpose
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'coronation_elessar'
         ORDER BY l.display_name"
    ]);
    
    // Should show ceremonial venue roles
    assert!(query_output.contains("ceremonial_site"));
    assert!(query_output.contains("throne_room"));
    assert!(query_output.contains("preparation"));
    assert!(query_output.contains("blessing"));
    assert!(query_output.contains("Citadel of Minas Tirith"));
    assert!(query_output.contains("Court of the Kings"));
}

#[test]
fn test_event_location_validation() {
    let world = TestWorld::new();
    
    // Try to create event with non-existent location
    let error = world.run_command_expect_failure(&[
        "event", "create", "council_meeting", 
        "--set", "display_name=Council of Elrond",
        "--set", "location=rivendell*meeting_hall"
    ]);
    
    // Should fail with helpful error
    assert!(error.contains("Location 'rivendell' does not exist"));
    assert!(error.contains("multiverse location create rivendell"));
}

#[test]
fn test_event_location_journey_events() {
    let world = TestWorld::new();
    
    // Create journey waypoints
    world.run_command_success(&["location", "create", "bag_end", "--set", "display_name=Bag End"]);
    world.run_command_success(&["location", "create", "bree", "--set", "display_name=Bree"]);
    world.run_command_success(&["location", "create", "weathertop", "--set", "display_name=Weathertop"]);
    world.run_command_success(&["location", "create", "rivendell_j", "--set", "display_name=Rivendell"]);
    
    // Create journey event with waypoint roles
    let output = world.run_command_success(&[
        "event", "create", "frodo_journey", 
        "--set", "display_name=Frodo's Journey to Rivendell",
        "--set", "location=bag_end*departure,bree*waypoint,weathertop*danger_point,rivendell_j*destination"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query journey analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as place,
           r.location_role as journey_role
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'frodo_journey'
         ORDER BY CASE r.location_role 
           WHEN 'departure' THEN 1
           WHEN 'waypoint' THEN 2  
           WHEN 'danger_point' THEN 3
           WHEN 'destination' THEN 4
           ELSE 5 END"
    ]);
    
    // Should show journey progression
    assert!(query_output.contains("departure"));
    assert!(query_output.contains("waypoint"));
    assert!(query_output.contains("danger_point"));
    assert!(query_output.contains("destination"));
    assert!(query_output.contains("Bag End"));
    assert!(query_output.contains("Bree"));
    assert!(query_output.contains("Weathertop"));
}

#[test]
fn test_event_location_diplomatic_meetings() {
    let world = TestWorld::new();
    
    // Create diplomatic venues
    world.run_command_success(&["location", "create", "isengard_d", "--set", "display_name=Isengard"]);
    world.run_command_success(&["location", "create", "fangorn", "--set", "display_name=Fangorn Forest"]);
    world.run_command_success(&["location", "create", "edoras", "--set", "display_name=Edoras"]);
    world.run_command_success(&["location", "create", "hornburg", "--set", "display_name=The Hornburg"]);
    
    // Create diplomatic negotiation event
    let output = world.run_command_success(&[
        "event", "create", "last_march_ents", 
        "--set", "display_name=The Last March of the Ents",
        "--set", "location=isengard_d*target,fangorn*origin,edoras*ally_territory,hornburg*strategic_point"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query diplomatic venues analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as diplomatic_venue,
           r.location_role as strategic_importance
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'last_march_ents'
         ORDER BY r.location_role"
    ]);
    
    // Should show strategic roles
    assert!(query_output.contains("target"));
    assert!(query_output.contains("origin"));
    assert!(query_output.contains("ally_territory"));
    assert!(query_output.contains("strategic_point"));
    assert!(query_output.contains("Isengard"));
    assert!(query_output.contains("Fangorn Forest"));
}

#[test]
fn test_event_location_magical_events() {
    let world = TestWorld::new();
    
    // Create magical locations
    world.run_command_success(&["location", "create", "mount_doom", "--set", "display_name=Mount Doom"]);
    world.run_command_success(&["location", "create", "cracks_doom", "--set", "display_name=Cracks of Doom"]);
    world.run_command_success(&["location", "create", "sammath_naur", "--set", "display_name=Sammath Naur"]);
    world.run_command_success(&["location", "create", "barad_dur", "--set", "display_name=Barad-dûr"]);
    
    // Create magical destruction event
    let output = world.run_command_success(&[
        "event", "create", "ring_destruction", 
        "--set", "display_name=Destruction of the One Ring",
        "--set", "location=mount_doom*magical_nexus,cracks_doom*destruction_site,sammath_naur*chamber,barad_dur*affected_fortress"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query magical event venues analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as magical_place,
           r.location_role as magical_function
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'ring_destruction'
         ORDER BY l.display_name"
    ]);
    
    // Should show magical functions
    assert!(query_output.contains("magical_nexus"));
    assert!(query_output.contains("destruction_site"));
    assert!(query_output.contains("chamber"));
    assert!(query_output.contains("affected_fortress"));
    assert!(query_output.contains("Mount Doom"));
    assert!(query_output.contains("Cracks of Doom"));
}

#[test]
fn test_event_location_multi_realm_council() {
    let world = TestWorld::new();
    
    // Create realm locations
    world.run_command_success(&["location", "create", "rivendell_c", "--set", "display_name=Rivendell"]);
    world.run_command_success(&["location", "create", "lothlorien", "--set", "display_name=Lothlórien"]);
    world.run_command_success(&["location", "create", "mirkwood", "--set", "display_name=Mirkwood"]);
    world.run_command_success(&["location", "create", "grey_havens", "--set", "display_name=Grey Havens"]);
    
    // Create inter-realm council event
    let output = world.run_command_success(&[
        "event", "create", "white_council_meets", 
        "--set", "display_name=Meeting of the White Council",
        "--set", "location=rivendell_c*host_realm,lothlorien*participating_realm,mirkwood*participating_realm,grey_havens*represented_realm"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query inter-realm council analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as elven_realm,
           r.location_role as participation_type,
           CASE r.location_role 
             WHEN 'host_realm' THEN 'Primary'
             WHEN 'participating_realm' THEN 'Active'
             WHEN 'represented_realm' THEN 'Represented'
             ELSE 'Other'
           END as involvement_level
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'white_council_meets'
         ORDER BY involvement_level, l.display_name"
    ]);
    
    // Should show realm participation
    assert!(query_output.contains("host_realm"));
    assert!(query_output.contains("participating_realm"));
    assert!(query_output.contains("represented_realm"));
    assert!(query_output.contains("Rivendell"));
    assert!(query_output.contains("Lothlórien"));
    assert!(query_output.contains("Mirkwood"));
}

#[test]
fn test_event_location_update_venues() {
    let world = TestWorld::new();
    
    // Create locations
    world.run_command_success(&["location", "create", "orthanc", "--set", "display_name=Orthanc"]);
    world.run_command_success(&["location", "create", "isengard_u", "--set", "display_name=Isengard"]);
    world.run_command_success(&["location", "create", "fangorn_u", "--set", "display_name=Fangorn Forest"]);
    
    // Create initial event
    world.run_command_success(&[
        "event", "create", "saruman_betrayal", 
        "--set", "display_name=Saruman's Betrayal",
        "--set", "location=orthanc*plotting_place,isengard_u*power_base"
    ]);
    
    // Later, add consequence location
    let output = world.run_command_success(&[
        "event", "update", "saruman_betrayal", 
        "--set", "location=fangorn_u*retribution_source"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query updated venue roles
    let query_output = world.run_command_success(&[
        "query",
        "SELECT to_id, location_role FROM event_location_relations WHERE from_id = 'saruman_betrayal' ORDER BY to_id"
    ]);
    
    assert!(query_output.contains("plotting_place"));
    assert!(query_output.contains("power_base"));
    assert!(query_output.contains("retribution_source"));
    assert!(query_output.contains("📊 3 row(s) returned"));
}

#[test]
fn test_event_location_environmental_disaster() {
    let world = TestWorld::new();
    
    // Create environmental locations
    world.run_command_success(&["location", "create", "old_forest", "--set", "display_name=Old Forest"]);
    world.run_command_success(&["location", "create", "barrow_downs", "--set", "display_name=Barrow-downs"]);
    world.run_command_success(&["location", "create", "withywindle", "--set", "display_name=River Withywindle"]);
    world.run_command_success(&["location", "create", "bombadil_house", "--set", "display_name=Tom Bombadil's House"]);
    
    // Create environmental event with ecological roles
    let output = world.run_command_success(&[
        "event", "create", "old_forest_danger", 
        "--set", "display_name=Danger in the Old Forest",
        "--set", "location=old_forest*hostile_environment,barrow_downs*cursed_area,withywindle*treacherous_waterway,bombadil_house*sanctuary"
    ]);
    
    assert!(output.contains("🔗 Processed relations: location"));
    
    // Query environmental roles analysis
    let query_output = world.run_command_success(&[
        "query",
        "SELECT 
           l.display_name as natural_area,
           r.location_role as environmental_aspect,
           CASE 
             WHEN r.location_role LIKE '%hostile%' OR r.location_role LIKE '%cursed%' OR r.location_role LIKE '%treacherous%' THEN 'Dangerous'
             WHEN r.location_role LIKE '%sanctuary%' OR r.location_role LIKE '%safe%' THEN 'Safe'
             ELSE 'Neutral'
           END as safety_level
         FROM event_location_relations r
         JOIN locations l ON r.to_id = l.name
         WHERE r.from_id = 'old_forest_danger'
         ORDER BY safety_level DESC"
    ]);
    
    // Should show environmental aspects
    assert!(query_output.contains("hostile_environment"));
    assert!(query_output.contains("cursed_area"));
    assert!(query_output.contains("treacherous_waterway"));
    assert!(query_output.contains("sanctuary"));
    assert!(query_output.contains("Old Forest"));
    assert!(query_output.contains("Barrow-downs"));
}