//! Integration tests for Story entity with full world setup
//! These tests simulate the complete user experience using various story scenarios

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
    
    fn story_directory_exists(&self, story_name: &str) -> bool {
        self.world_path.join("stories").join(story_name).exists()
    }
}

#[test]
fn test_world_initialization_with_story_support() {
    let world = TestWorld::new();
    
    // Verify world structure was created
    assert!(world.world_path.exists(), "World directory should exist");
    assert!(world.database_exists(), "World database should exist");
    assert!(world.world_path.join(".multiverse/config.toml").exists(), "Config should exist");
    assert!(world.world_path.join("README.md").exists(), "README should exist");
    
    // Check story types are available
    let output = world.run_command_success(&["story", "types"]);
    assert!(output.contains("Available story types"));
}

#[test]
fn test_create_simple_fantasy_story() {
    let world = TestWorld::new();
    
    // Create a basic book story
    let output = world.run_command_success(&[
        "story", "create", "dragon_quest", 
        "--set", "title=The Dragon's Quest",
        "--set", "type=book",
        "--set", "author=Tolkien Jr.",
        "--set", "genre=Fantasy",
        "--set", "description=A brave knight seeks the ancient dragon"
    ]);
    
    assert!(output.contains("Creating story 'dragon_quest'"));
    assert!(output.contains("The Dragon's Quest"));
    assert!(output.contains("Story 'dragon_quest' created!"));
    assert!(output.contains("author: \"Tolkien Jr.\""));
    
    // Verify directory was created
    assert!(world.story_directory_exists("dragon_quest"));
}

#[test]
fn test_create_multiple_story_types() {
    let world = TestWorld::new();
    
    // Create various story types
    world.run_command_success(&[
        "story", "create", "space_odyssey",
        "--set", "title=Journey Among the Stars", 
        "--set", "type=book",
        "--set", "author=Arthur C. Clarke Jr.",
        "--set", "genre=Sci-Fi",
        "--set", "setting=Deep Space",
        "--set", "year=2387"
    ]);
    
    world.run_command_success(&[
        "story", "create", "murder_mystery",
        "--set", "title=Death in the Library",
        "--set", "type=book",
        "--set", "author=Arthur Conan Doyle Jr.",
        "--set", "genre=Mystery",
        "--set", "detective=Inspector Holmes",
        "--set", "location=Victorian London"
    ]);
    
    world.run_command_success(&[
        "story", "create", "love_triangle",
        "--set", "title=Hearts Divided",
        "--set", "type=book",
        "--set", "author=Jane Austen Jr.",
        "--set", "genre=Romance",
        "--set", "protagonist=Emma",
        "--set", "setting=Modern NYC"
    ]);
    
    // List all stories
    let list_output = world.run_command_success(&["story", "list"]);
    
    assert!(list_output.contains("space_odyssey - \"Journey Among the Stars\""));
    assert!(list_output.contains("murder_mystery - \"Death in the Library\""));
    assert!(list_output.contains("love_triangle - \"Hearts Divided\""));
    assert!(list_output.contains("book"));
}

#[test]
fn test_story_info_and_metadata() {
    let world = TestWorld::new();
    
    // Create detailed story with rich metadata
    world.run_command_success(&[
        "story", "create", "epic_fantasy",
        "--set", "title=The Chronicles of Aethermoor",
        "--set", "type=book",
        "--set", "author=Brandon Sanderson Jr.",
        "--set", "genre=Epic Fantasy",
        "--set", "world=Aethermoor",
        "--set", "magic_system=Elemental Weaving",
        "--set", "protagonist=Kael the Stormcaller",
        "--set", "antagonist=Lord Voidheart",
        "--set", "themes=Power, Friendship, Sacrifice"
    ]);
    
    // Get story info
    let info_output = world.run_command_success(&["story", "info", "epic_fantasy"]);
    
    assert!(info_output.contains("Story: epic_fantasy - \"The Chronicles of Aethermoor\""));
    assert!(info_output.contains("Type: book"));
    assert!(info_output.contains("Status: Draft"));
    assert!(info_output.contains("author: \"Brandon Sanderson Jr.\""));
    assert!(info_output.contains("magic_system: \"Elemental Weaving\""));
    assert!(info_output.contains("protagonist: \"Kael the Stormcaller\""));
}

#[test]
fn test_story_lifecycle_and_status_updates() {
    let world = TestWorld::new();
    
    // Create story in Draft status
    world.run_command_success(&[
        "story", "create", "time_travel",
        "--set", "title=Echoes of Tomorrow",
        "--set", "type=book",
        "--set", "author=H.G. Wells Jr.",
        "--set", "genre=Science Fiction",
        "--set", "plot=Scientist discovers time machine",
        "--set", "timeline=Multiple timelines"
    ]);
    
    // Verify initial status
    let info = world.run_command_success(&["story", "info", "time_travel"]);
    assert!(info.contains("Status: Draft"));
    
    // Update to InProgress
    world.run_command_success(&[
        "story", "update", "time_travel",
        "--set", "status=InProgress",
        "--set", "current_chapter=Chapter 3: The First Jump",
        "--set", "word_count=15000"
    ]);
    
    let updated_info = world.run_command_success(&["story", "info", "time_travel"]);
    assert!(updated_info.contains("current_chapter: \"Chapter 3: The First Jump\""));
    assert!(updated_info.contains("Word Count: 15000"));
    
    // Progress to Review
    world.run_command_success(&[
        "story", "update", "time_travel",
        "--set", "status=Review",
        "--set", "beta_readers=Alice, Bob, Carol",
        "--set", "notes=Need to fix temporal paradox in chapter 7"
    ]);
    
    // Finally publish
    world.run_command_success(&[
        "story", "update", "time_travel",
        "--set", "status=Published",
        "--set", "publication_date=2024-03-15",
        "--set", "publisher=Sci-Fi Weekly"
    ]);
    
    let final_info = world.run_command_success(&["story", "info", "time_travel"]);
    assert!(final_info.contains("publication_date: \"2024-03-15\""));
    assert!(final_info.contains("publisher: \"Sci-Fi Weekly\""));
}

#[test]
fn test_story_series_management() {
    let world = TestWorld::new();
    
    // Create a fantasy series - The Dragonlance Chronicles
    world.run_command_success(&[
        "story", "create", "dragons_autumn", 
        "--set", "title=Dragons of Autumn Twilight",
        "--set", "type=book",
        "--set", "author=Margaret Weis",
        "--set", "series_name=Dragonlance Chronicles",
        "--set", "volume=1",
        "--set", "genre=Fantasy",
        "--set", "heroes=Tanis, Sturm, Caramon, Raistlin",
        "--set", "setting=Krynn"
    ]);
    
    world.run_command_success(&[
        "story", "create", "dragons_winter",
        "--set", "title=Dragons of Winter Night", 
        "--set", "type=book",
        "--set", "author=Margaret Weis",
        "--set", "series_name=Dragonlance Chronicles",
        "--set", "volume=2",
        "--set", "genre=Fantasy",
        "--set", "heroes=Tanis, Sturm, Caramon, Raistlin",
        "--set", "setting=Krynn"
    ]);
    
    world.run_command_success(&[
        "story", "create", "dragons_spring",
        "--set", "title=Dragons of Spring Dawning",
        "--set", "type=book",
        "--set", "author=Margaret Weis", 
        "--set", "series_name=Dragonlance Chronicles",
        "--set", "volume=3",
        "--set", "genre=Fantasy",
        "--set", "heroes=Tanis, Sturm, Caramon, Raistlin",
        "--set", "setting=Krynn"
    ]);
    
    // List all stories and verify series
    let list_output = world.run_command_success(&["story", "list"]);
    
    assert!(list_output.contains("dragons_autumn - \"Dragons of Autumn Twilight\""));
    assert!(list_output.contains("dragons_winter - \"Dragons of Winter Night\""));  
    assert!(list_output.contains("dragons_spring - \"Dragons of Spring Dawning\""));
}

#[test]
fn test_story_genres_and_subgenres() {
    let world = TestWorld::new();
    
    // Create stories across different genres  
    let stories = [
        ("cyberpunk_noir", "Neon Shadows", "William Gibson Jr.", "Cyberpunk", "Neo-Tokyo 2087"),
        ("space_opera", "Galactic Rebellion", "Isaac Asimov Jr.", "Space Opera", "Multiple star systems"),
        ("urban_fantasy", "Magic in Manhattan", "Jim Butcher Jr.", "Urban Fantasy", "Modern New York"),
        ("epic_fantasy", "The Sword of Kings", "Brandon Sanderson Jr.", "Epic Fantasy", "Medieval Valdris"),
        ("cozy_mystery", "Tea Shop Murders", "Agatha Christie Jr.", "Cozy Mystery", "Small English village"),
        ("psychological_thriller", "Mind Games", "Gillian Flynn Jr.", "Psychological Thriller", "Psychiatric hospital")
    ];
    
    for (name, title, author, genre, setting) in stories.iter() {
        world.run_command_success(&[
            "story", "create", name,
            "--set", &format!("title={}", title),
            "--set", "type=book",
            "--set", &format!("author={}", author),
            "--set", &format!("genre={}", genre),
            "--set", &format!("setting={}", setting)
        ]);
    }
    
    // Verify all were created successfully
    let list_output = world.run_command_success(&["story", "list"]);
    
    for (name, title, _, _, _) in stories.iter() {
        assert!(list_output.contains(&format!("{} - \"{}\"", name, title)));
    }
}

#[test]
fn test_story_collaboration_metadata() {
    let world = TestWorld::new();
    
    // Create collaborative story
    world.run_command_success(&[
        "story", "create", "shared_universe",
        "--set", "title=Tales of the Shared Realm",
        "--set", "type=book",
        "--set", "author=Sarah Chen",
        "--set", "genre=Fantasy",
        "--set", "collaborative=true",
        "--set", "co_authors=Mike Johnson, Lisa Park, Alex Rivera",
        "--set", "editor=Jennifer Smith",
        "--set", "world_bible=shared_realm_bible.md",
        "--set", "style_guide=fantasy_style_guide.md",
        "--set", "character_registry=shared_characters.json"
    ]);
    
    let info = world.run_command_success(&["story", "info", "shared_universe"]);
    
    assert!(info.contains("collaborative: \"true\""));
    assert!(info.contains("author: \"Sarah Chen\""));
    assert!(info.contains("co_authors: \"Mike Johnson, Lisa Park, Alex Rivera\""));
    assert!(info.contains("world_bible: \"shared_realm_bible.md\""));
}

#[test]
fn test_story_technical_specifications() {
    let world = TestWorld::new();
    
    // Create story with technical writing specifications
    world.run_command_success(&[
        "story", "create", "formatted_novel",
        "--set", "title=The Perfectly Formatted Novel",
        "--set", "type=book",
        "--set", "author=Literary Master",
        "--set", "genre=Literary Fiction",
        "--set", "target_word_count=80000",
        "--set", "current_word_count=25000",
        "--set", "chapters_planned=20",
        "--set", "chapters_written=7",
        "--set", "font=Times New Roman",
        "--set", "font_size=12pt",
        "--set", "line_spacing=double",
        "--set", "margin=1 inch",
        "--set", "format=manuscript"
    ]);
    
    let info = world.run_command_success(&["story", "info", "formatted_novel"]);
    
    assert!(info.contains("target_word_count: \"80000\""));
    assert!(info.contains("chapters_planned: \"20\""));
    assert!(info.contains("font: \"Times New Roman\""));
    assert!(info.contains("format: \"manuscript\""));
}

#[test]
fn test_story_delete_with_confirmation() {
    let world = TestWorld::new();
    
    // Create story to delete
    world.run_command_success(&[
        "story", "create", "temporary_story",
        "--set", "title=This Will Be Deleted",
        "--set", "type=book",
        "--set", "author=Temporary Author"
    ]);
    
    // Verify it exists
    let list_before = world.run_command_success(&["story", "list"]);
    assert!(list_before.contains("temporary_story"));
    assert!(world.story_directory_exists("temporary_story"));
    
    // Try delete without force (should show warning)
    let delete_output = world.run_command(&["story", "delete", "temporary_story"]);
    let delete_stdout = String::from_utf8_lossy(&delete_output.stdout);
    assert!(delete_stdout.contains("Are you sure you want to delete"));
    assert!(delete_stdout.contains("Use --force to skip"));
    
    // Still should exist
    let list_still = world.run_command_success(&["story", "list"]);
    assert!(list_still.contains("temporary_story"));
    
    // Delete with force
    world.run_command_success(&["story", "delete", "temporary_story", "--force"]);
    
    // Verify deletion
    let list_after = world.run_command_success(&["story", "list"]);
    assert!(!list_after.contains("temporary_story"));
    assert!(!world.story_directory_exists("temporary_story"));
}

#[test]
fn test_complex_story_workflow() {
    let world = TestWorld::new();
    
    // 1. Create initial story concept
    world.run_command_success(&[
        "story", "create", "magnum_opus",
        "--set", "title=The Chronicles of Seven Kingdoms",
        "--set", "type=book",
        "--set", "author=Epic Fantasy Master",
        "--set", "genre=Epic Fantasy",
        "--set", "concept=Epic fantasy spanning multiple kingdoms",
        "--set", "inspiration=Game of Thrones meets Lord of the Rings"
    ]);
    
    // 2. Add world-building details
    world.run_command_success(&[
        "story", "update", "magnum_opus",
        "--set", "kingdoms=Aethros, Valeria, Drakmoor, Solaria, Frostheim, Shadowvale, Goldspire",
        "--set", "magic_system=Elemental Binding with Runic Enhancement",
        "--set", "main_conflict=Ancient evil awakens as kingdoms war",
        "--set", "timeline=3000 years of history"
    ]);
    
    // 3. Character planning
    world.run_command_success(&[
        "story", "update", "magnum_opus",
        "--set", "protagonists=Prince Aldric, Mage Lyralei, Warrior Grimjaw",
        "--set", "antagonists=The Void King, Corrupted High Mage Malthorn",
        "--set", "supporting_cast=15+ named characters with full backstories"
    ]);
    
    // 4. Production planning
    world.run_command_success(&[
        "story", "update", "magnum_opus", 
        "--set", "target_length=150000 words",
        "--set", "planned_books=5",
        "--set", "current_book=1",
        "--set", "writing_schedule=1000 words per day",
        "--set", "estimated_completion=18 months"
    ]);
    
    // 5. Move to active development
    world.run_command_success(&[
        "story", "update", "magnum_opus",
        "--set", "status=InProgress",
        "--set", "start_date=2024-01-01",
        "--set", "daily_goal_met_streak=47"
    ]);
    
    // 6. Verify the complete workflow
    let final_info = world.run_command_success(&["story", "info", "magnum_opus"]);
    
    assert!(final_info.contains("The Chronicles of Seven Kingdoms"));
    assert!(final_info.contains("kingdoms: \"Aethros, Valeria, Drakmoor"));
    assert!(final_info.contains("target_length: \"150000 words\""));
    assert!(final_info.contains("daily_goal_met_streak: \"47\""));
}

#[test]
fn test_database_persistence_stories() {
    let world = TestWorld::new();
    
    // Create multiple stories with different configurations
    let test_stories = [
        ("scifi_classic", "Foundation's Edge", "Isaac Asimov Jr.", "Isaac Asimov style"),
        ("fantasy_epic", "The Name of the Wind", "Patrick Rothfuss Jr.", "Patrick Rothfuss inspired"), 
        ("mystery_noir", "The Big Sleep", "Raymond Chandler Jr.", "Raymond Chandler homage"),
        ("romance_contemporary", "Beach Read", "Emily Henry Jr.", "Emily Henry vibes"),
        ("horror_gothic", "Dracula's Return", "Bram Stoker Jr.", "Bram Stoker sequel")
    ];
    
    for (name, title, author, description) in test_stories.iter() {
        world.run_command_success(&[
            "story", "create", name,
            "--set", &format!("title={}", title),
            "--set", "type=book",
            "--set", &format!("author={}", author),
            "--set", &format!("description={}", description)
        ]);
    }
    
    // Verify database exists and contains all stories
    assert!(world.database_exists());
    
    let list_output = world.run_command_success(&["story", "list"]);
    for (name, title, _, _) in test_stories.iter() {
        assert!(list_output.contains(&format!("{} - \"{}\"", name, title)));
        assert!(list_output.contains("book"));
    }
    
    // Test individual story retrieval
    for (name, title, _, _) in test_stories.iter() {
        let info_output = world.run_command_success(&["story", "info", name]);
        assert!(info_output.contains(&format!("Story: {} - \"{}\"", name, title)));
    }
}