//! Integration tests for the query command

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

        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        let output = Command::new(&multiverse_bin)
            .args(&["world", "init", "test-world"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to run world init");

        if !output.status.success() {
            panic!(
                "World init failed: {} Stdout: {}",
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }

        Self {
            temp_dir,
            world_path,
        }
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
            panic!(
                "Command failed: {:?}\nStderr: {}\nStdout: {}",
                args,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    #[allow(dead_code)]
    fn run_command_expect_failure(&self, args: &[&str]) -> String {
        let multiverse_bin = std::env::current_dir()
            .expect("Failed to get current dir")
            .join("target/debug/multiverse");

        let output = Command::new(&multiverse_bin)
            .args(args)
            .current_dir(&self.world_path)
            .output()
            .expect("Failed to run command");

        assert!(
            !output.status.success(),
            "Command should have failed but succeeded"
        );

        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

#[test]
fn test_query_list_tables() {
    let world = TestWorld::new();
    let output = world.run_command_success(&[
        "query",
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name",
    ]);
    assert!(output.contains("characters"));
    assert!(output.contains("locations"));
    assert!(output.contains("factions"));
    assert!(output.contains("events"));
    assert!(output.contains("races"));
    assert!(output.contains("systems"));
}

#[test]
fn test_query_empty_result() {
    let world = TestWorld::new();
    world.run_command_success(&["story", "create", "test-story", "--set", "type=diary"]);
    world.run_command_success(&["episode", "create", "--story", "test-story"]);
    // Query existing table with no matching results
    let output = world.run_command_success(&[
        "query",
        "SELECT * FROM episodes WHERE title = 'nonexistent'",
    ]);
    assert!(output.contains("No results found"));
}

#[test]
fn test_query_with_relations() {
    let world = TestWorld::new();
    world.run_command_success(&["story", "create", "test-story", "--set", "type=diary"]);
    world.run_command_success(&[
        "episode",
        "create",
        "--story",
        "test-story",
        "--set",
        "title=The First Episode",
    ]);
    world.run_command_success(&[
        "character",
        "create",
        "john",
        "--set",
        "display_name=John Doe",
        "--set",
        "episode=test-story:1*protagonist",
    ]);

    let output = world.run_command_success(&[
        "query",
        "SELECT from_id, to_id, role FROM character_episode_relations",
    ]);
    assert!(output.contains("role"));
    assert!(output.contains("protagonist"));
}

#[test]
fn test_query_join_relations() {
    let world = TestWorld::new();
    world.run_command_success(&["story", "create", "test-story", "--set", "type=diary"]);
    world.run_command_success(&[
        "episode",
        "create",
        "--story",
        "test-story",
        "--set",
        "title=The First Episode",
    ]);
    world.run_command_success(&[
        "character",
        "create",
        "hero",
        "--set",
        "display_name=The Hero",
        "--set",
        "episode=test-story:1*main",
    ]);

    let output = world.run_command_success(&["query", "SELECT c.display_name, e.title FROM characters c JOIN character_episode_relations r ON c.id = r.from_id JOIN episodes e ON e.id = r.to_id"]);
    assert!(output.contains("The Hero"));
    assert!(output.contains("The First Episode"));
}

#[test]
fn test_query_security_validation() {
    let world = TestWorld::new();
    let error_output = world.run_command_expect_failure(&["query", "DROP TABLE characters"]);
    assert!(error_output.contains("Only SELECT queries are allowed"));

    let error_output_insert = world
        .run_command_expect_failure(&["query", "INSERT INTO characters (name) VALUES ('mallory')"]);
    assert!(error_output_insert.contains("Only SELECT queries are allowed"));
}
