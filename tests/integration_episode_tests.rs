//! Integration tests for Episode entity with full world setup
//! These tests simulate the complete user experience for episode management

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
            .args(&["world", "init", "episode-test-world"])
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
    
    fn run_command(&self, args: &[&str]) -> std::process::Output {
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
    
    fn episode_file_exists(&self, story_name: &str, episode_number: i32) -> bool {
        let episode_file = format!("{:03}.md", episode_number);
        self.world_path
            .join("stories")
            .join(story_name)
            .join(episode_file)
            .exists()
    }
    
    fn create_test_story(&self, story_name: &str, title: &str) -> String {
        self.run_command_success(&[
            "story", "create", story_name,
            "--set", &format!("title={}", title),
            "--set", "type=book",
            "--set", "author=Test Author"
        ])
    }
}

#[test]
fn test_world_initialization_with_episode_support() {
    let world = TestWorld::new();
    
    // Verify world structure was created
    assert!(world.world_path.exists(), "World directory should exist");
    assert!(world.database_exists(), "World database should exist");
    assert!(world.world_path.join(".multiverse/config.toml").exists(), "Config should exist");
    assert!(world.world_path.join("README.md").exists(), "README should exist");
}

#[test]
fn test_create_first_episode_in_story() {
    let world = TestWorld::new();
    
    // Create a story first
    world.create_test_story("adventure-novel", "The Great Adventure");
    
    // Create first episode
    let output = world.run_command_success(&[
        "episode", "create", 
        "--story", "adventure-novel",
        "--set", "title=The Journey Begins"
    ]);
    
    assert!(output.contains("Creating episode in story 'adventure-novel'"));
    assert!(output.contains("Episode 1 created!"));
    assert!(output.contains("Story: adventure-novel"));
    assert!(output.contains("Title: The Journey Begins"));
    
    // Verify episode file was created
    assert!(world.episode_file_exists("adventure-novel", 1));
}

#[test]
fn test_create_multiple_episodes_sequential_numbering() {
    let world = TestWorld::new();
    
    // Create a story
    world.create_test_story("fantasy-saga", "The Chronicles of Magic");
    
    // Create multiple episodes
    let episodes = [
        ("The Awakening", "A young mage discovers their power"),
        ("The Mentor", "Finding a wise teacher"),
        ("The Quest", "Embarking on a dangerous journey"),
        ("The Betrayal", "An unexpected enemy reveals themselves"),
        ("The Resolution", "The final confrontation")
    ];
    
    for (i, (title, description)) in episodes.iter().enumerate() {
        let expected_number = i + 1;
        let output = world.run_command_success(&[
            "episode", "create",
            "--story", "fantasy-saga",
            "--set", &format!("title={}", title),
            "--set", &format!("description={}", description)
        ]);
        
        assert!(output.contains(&format!("Episode {} created!", expected_number)));
        assert!(world.episode_file_exists("fantasy-saga", expected_number as i32));
    }
}

#[test]
fn test_episode_list_for_story() {
    let world = TestWorld::new();
    
    // Create story and episodes
    world.create_test_story("detective-series", "Murder Mystery Series");
    
    let episodes = [
        "The Locked Room",
        "The Missing Witness", 
        "The Final Clue"
    ];
    
    for title in episodes.iter() {
        world.run_command_success(&[
            "episode", "create",
            "--story", "detective-series",
            "--set", &format!("title={}", title)
        ]);
    }
    
    // List episodes
    let list_output = world.run_command_success(&["episode", "list", "--story", "detective-series"]);
    
    assert!(list_output.contains("Episodes in story 'detective-series'"));
    assert!(list_output.contains("001. The Locked Room"));
    assert!(list_output.contains("002. The Missing Witness"));
    assert!(list_output.contains("003. The Final Clue"));
}

#[test]
fn test_episode_info_detailed_view() {
    let world = TestWorld::new();
    
    // Create story and episode
    world.create_test_story("sci-fi-epic", "Space Opera Chronicles");
    
    world.run_command_success(&[
        "episode", "create",
        "--story", "sci-fi-epic",
        "--set", "title=The Alien Encounter",
        "--set", "setting=Deep Space Station Alpha",
        "--set", "characters=Captain Nova, Dr. Chen, Alien Ambassador"
    ]);
    
    // Get episode info
    let info_output = world.run_command_success(&[
        "episode", "info", 
        "--story", "sci-fi-epic", 
        "--number", "1"
    ]);
    
    assert!(info_output.contains("Episode 1: sci-fi-epic"));
    assert!(info_output.contains("Title: The Alien Encounter"));
    assert!(info_output.contains("Status: Draft"));
    assert!(info_output.contains("setting: \"Deep Space Station Alpha\""));
    assert!(info_output.contains("characters: \"Captain Nova, Dr. Chen, Alien Ambassador\""));
}

#[test]
fn test_episode_status_lifecycle() {
    let world = TestWorld::new();
    
    // Create story and episode
    world.create_test_story("romance-novel", "Love in the City");
    
    world.run_command_success(&[
        "episode", "create",
        "--story", "romance-novel",
        "--set", "title=First Meeting"
    ]);
    
    // Verify initial status
    let info = world.run_command_success(&["episode", "info", "--story", "romance-novel", "--number", "1"]);
    assert!(info.contains("Status: Draft"));
    
    // Update to InProgress
    world.run_command_success(&[
        "episode", "update",
        "--story", "romance-novel",
        "--number", "1",
        "--set", "status=InProgress",
        "--set", "current_scene=Coffee shop meeting"
    ]);
    
    let updated_info = world.run_command_success(&["episode", "info", "--story", "romance-novel", "--number", "1"]);
    assert!(updated_info.contains("Status: InProgress"));
    assert!(updated_info.contains("current_scene: \"Coffee shop meeting\""));
    
    // Progress to Review
    world.run_command_success(&[
        "episode", "update",
        "--story", "romance-novel",
        "--number", "1",
        "--set", "status=Review",
        "--set", "word_count=2500",
        "--set", "reviewer=Beta Reader Alice"
    ]);
    
    // Finally publish
    world.run_command_success(&[
        "episode", "update",
        "--story", "romance-novel",
        "--number", "1",
        "--set", "status=Published",
        "--set", "publish_date=2024-03-15"
    ]);
    
    let final_info = world.run_command_success(&["episode", "info", "--story", "romance-novel", "--number", "1"]);
    assert!(final_info.contains("Status: Published"));
    assert!(final_info.contains("Word Count: 2500"));
    assert!(final_info.contains("publish_date: \"2024-03-15\""));
}

#[test]
fn test_episode_word_count_tracking() {
    let world = TestWorld::new();
    
    // Create story and episodes with word counts
    world.create_test_story("writing-challenge", "Daily Writing Challenge");
    
    let episodes_with_counts = [
        ("Day 1: The Beginning", 1200),
        ("Day 2: Character Development", 1500),
        ("Day 3: Plot Twist", 1800),
        ("Day 4: Conflict Resolution", 1300),
        ("Day 5: The Ending", 1100)
    ];
    
    for (title, word_count) in episodes_with_counts.iter() {
        world.run_command_success(&[
            "episode", "create",
            "--story", "writing-challenge",
            "--set", &format!("title={}", title),
            "--set", &format!("word_count={}", word_count)
        ]);
    }
    
    // Check each episode shows correct word count
    for (i, (title, expected_count)) in episodes_with_counts.iter().enumerate() {
        let episode_number = i + 1;
        let info_output = world.run_command_success(&[
            "episode", "info",
            "--story", "writing-challenge",
            "--number", &episode_number.to_string()
        ]);
        
        assert!(info_output.contains(&format!("Word Count: {}", expected_count)));
        assert!(info_output.contains(&format!("Title: {}", title)));
    }
    
    // List should show word counts
    let list_output = world.run_command_success(&["episode", "list", "--story", "writing-challenge"]);
    assert!(list_output.contains("(1200 words)"));
    assert!(list_output.contains("(1500 words)"));
    assert!(list_output.contains("(1800 words)"));
}

#[test]
fn test_episode_file_content_generation() {
    let world = TestWorld::new();
    
    // Create story and episode
    world.create_test_story("test-story", "Test Story for Content");
    
    world.run_command_success(&[
        "episode", "create",
        "--story", "test-story",
        "--set", "title=Content Test Episode",
        "--set", "theme=Testing",
        "--set", "mood=Experimental"
    ]);
    
    // Verify episode file exists and has expected structure
    let episode_path = world.world_path
        .join("stories")
        .join("test-story")
        .join("001.md");
    
    assert!(episode_path.exists(), "Episode file should exist");
    
    let content = std::fs::read_to_string(&episode_path)
        .expect("Should be able to read episode file");
    
    assert!(content.contains("# Content Test Episode"));
    assert!(content.contains("**Story:** Test Story for Content"));
    assert!(content.contains("**Episode:** 1"));
    assert!(content.contains("**Status:** Draft"));
    assert!(content.contains("**Word Count:** 0"));
    assert!(content.contains("[Episode content goes here]"));
}

#[test]
fn test_episode_delete_with_confirmation() {
    let world = TestWorld::new();
    
    // Create story and episode
    world.create_test_story("temp-story", "Temporary Story");
    
    world.run_command_success(&[
        "episode", "create",
        "--story", "temp-story",
        "--set", "title=Episode to Delete"
    ]);
    
    // Verify episode exists
    assert!(world.episode_file_exists("temp-story", 1));
    
    // Try delete without force (should show warning)
    let delete_output = world.run_command(&[
        "episode", "delete", 
        "--story", "temp-story", 
        "--number", "1"
    ]);
    let delete_stdout = String::from_utf8_lossy(&delete_output.stdout);
    assert!(delete_stdout.contains("Are you sure you want to delete episode 1"));
    assert!(delete_stdout.contains("Use --force to skip"));
    
    // Episode should still exist
    assert!(world.episode_file_exists("temp-story", 1));
    
    // Delete with force
    world.run_command_success(&[
        "episode", "delete", 
        "--story", "temp-story", 
        "--number", "1", 
        "--force"
    ]);
    
    // Verify deletion
    assert!(!world.episode_file_exists("temp-story", 1));
}

#[test]
fn test_multiple_stories_episodes() {
    let world = TestWorld::new();
    
    // Create multiple stories
    world.create_test_story("story-a", "Story A");
    world.create_test_story("story-b", "Story B");
    world.create_test_story("story-c", "Story C");
    
    // Create episodes in each story
    let stories_episodes = [
        ("story-a", vec!["A1: Beginning", "A2: Middle"]),
        ("story-b", vec!["B1: Setup", "B2: Development", "B3: Climax"]), 
        ("story-c", vec!["C1: Introduction"])
    ];
    
    for (story_name, episodes) in stories_episodes.iter() {
        for episode_title in episodes.iter() {
            world.run_command_success(&[
                "episode", "create",
                "--story", story_name,
                "--set", &format!("title={}", episode_title)
            ]);
        }
    }
    
    // Verify each story has correct episodes
    for (story_name, expected_episodes) in stories_episodes.iter() {
        let list_output = world.run_command_success(&["episode", "list", "--story", story_name]);
        
        for (i, expected_title) in expected_episodes.iter().enumerate() {
            let episode_number = i + 1;
            assert!(list_output.contains(&format!("{:03}. {}", episode_number, expected_title)));
        }
        
        // Verify episodes from other stories are not listed
        for (other_story, other_episodes) in stories_episodes.iter() {
            if other_story != story_name {
                for other_title in other_episodes.iter() {
                    assert!(!list_output.contains(other_title));
                }
            }
        }
    }
}

#[test]
fn test_episode_complex_workflow() {
    let world = TestWorld::new();
    
    // 1. Create a story
    world.create_test_story("novel-project", "My Great Novel");
    
    // 2. Create first chapter with planning metadata
    world.run_command_success(&[
        "episode", "create",
        "--story", "novel-project",
        "--set", "title=Chapter 1: The Awakening",
        "--set", "pov=First person",
        "--set", "main_character=Sarah Chen",
        "--set", "setting=Modern San Francisco",
        "--set", "conflict=Discovery of supernatural abilities",
        "--set", "target_words=3000"
    ]);
    
    // 3. Start writing (update status and add progress)
    world.run_command_success(&[
        "episode", "update",
        "--story", "novel-project",
        "--number", "1",
        "--set", "status=InProgress",
        "--set", "current_scene=Opening scene - coffee shop",
        "--set", "word_count=1200",
        "--set", "notes=Strong opening, good hook established"
    ]);
    
    // 4. Continue writing progress
    world.run_command_success(&[
        "episode", "update",
        "--story", "novel-project",
        "--number", "1",
        "--set", "word_count=2800",
        "--set", "current_scene=Climax - powers manifest",
        "--set", "completion_percent=93"
    ]);
    
    // 5. Finish chapter
    world.run_command_success(&[
        "episode", "update",
        "--story", "novel-project",
        "--number", "1",
        "--set", "status=Review",
        "--set", "word_count=3150",
        "--set", "completion_percent=100",
        "--set", "finished_date=2024-03-01",
        "--set", "revision_notes=Ready for beta reader feedback"
    ]);
    
    // 6. After review, publish
    world.run_command_success(&[
        "episode", "update",
        "--story", "novel-project",
        "--number", "1",
        "--set", "status=Published",
        "--set", "word_count=3200",
        "--set", "published_date=2024-03-15",
        "--set", "reader_rating=4.8"
    ]);
    
    // 7. Verify the complete workflow
    let final_info = world.run_command_success(&[
        "episode", "info", 
        "--story", "novel-project", 
        "--number", "1"
    ]);
    
    assert!(final_info.contains("Chapter 1: The Awakening"));
    assert!(final_info.contains("Status: Published"));
    assert!(final_info.contains("Word Count: 3200"));
    assert!(final_info.contains("pov: \"First person\""));
    assert!(final_info.contains("main_character: \"Sarah Chen\""));
    assert!(final_info.contains("published_date: \"2024-03-15\""));
    assert!(final_info.contains("reader_rating: \"4.8\""));
}

#[test]
fn test_database_persistence_episodes() {
    let world = TestWorld::new();
    
    // Create multiple stories with episodes
    let test_data = [
        ("adventure", "Adventure Story", vec!["The Call", "The Journey", "The Return"]),
        ("mystery", "Mystery Novel", vec!["The Crime", "The Investigation"]),
        ("romance", "Love Story", vec!["Meeting", "Falling", "Commitment", "Resolution"])
    ];
    
    for (story_name, story_title, episode_titles) in test_data.iter() {
        // Create story
        world.create_test_story(story_name, story_title);
        
        // Create episodes
        for (i, episode_title) in episode_titles.iter().enumerate() {
            world.run_command_success(&[
                "episode", "create",
                "--story", story_name,
                "--set", &format!("title={}", episode_title),
                "--set", &format!("chapter={}", i + 1),
                "--set", &format!("word_count={}", (i + 1) * 1000)
            ]);
        }
    }
    
    // Verify database exists and contains all episodes
    assert!(world.database_exists());
    
    // Verify each story has correct episodes
    for (story_name, _, expected_episodes) in test_data.iter() {
        let list_output = world.run_command_success(&["episode", "list", "--story", story_name]);
        
        for (i, expected_title) in expected_episodes.iter().enumerate() {
            let episode_number = i + 1;
            assert!(list_output.contains(&format!("{:03}. {}", episode_number, expected_title)));
        }
    }
    
    // Test individual episode retrieval
    for (story_name, _, episode_titles) in test_data.iter() {
        for (i, expected_title) in episode_titles.iter().enumerate() {
            let episode_number = i + 1;
            let info_output = world.run_command_success(&[
                "episode", "info", 
                "--story", story_name, 
                "--number", &episode_number.to_string()
            ]);
            assert!(info_output.contains(&format!("Title: {}", expected_title)));
        }
    }
}