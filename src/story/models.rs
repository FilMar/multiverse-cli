//! Story entity using the new modular macro system

use crate::define_complete_entity;
use serde::{Deserialize, Serialize};

// Generate complete Story entity
define_complete_entity!(
    Story,
    StoryStatus,
    StoryDb,
    table: "stories",
    key_fields: { 
        name: String 
    },
    fields: { 
        display_name: String,
        story_type: String,
        word_count: i32
    },
    status_variants: [ Draft, InProgress, Review, Published, Archived ],
    create_sql: "CREATE TABLE IF NOT EXISTS stories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        display_name TEXT NOT NULL,
        story_type TEXT NOT NULL DEFAULT 'Fantasy',
        word_count INTEGER NOT NULL DEFAULT 0,
        metadata TEXT NOT NULL DEFAULT '{}',
        created_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'Draft'
    )"
);

// Custom implementations for Story
impl Story {
    /// Display name for UI
    pub fn display_name(&self) -> &str {
        if !self.display_name.is_empty() {
            &self.display_name
        } else {
            &self.name
        }
    }

    /// Resolve story name to database ID
    pub fn resolve_id(name: &str) -> anyhow::Result<String> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT id FROM stories WHERE name = ?")?;
        let id: i32 = stmt.query_row([name], |row| {
            row.get(0)
        }).map_err(|_| anyhow::anyhow!("Story not found: '{}'", name))?;
        Ok(id.to_string())
    }

    /// Get story directory path within world
    pub fn get_story_path(&self, world_root: &std::path::Path) -> std::path::PathBuf {
        world_root.join("stories").join(&self.name)
    }

    /// Create story directory and initial files
    pub fn create_with_directory(&mut self) -> anyhow::Result<()> {
        let world_root = Self::ensure_world_context()?;
        
        // Create the story in database first
        self.create()?;
        
        // Create story directory structure
        let story_path = self.get_story_path(&world_root);
        std::fs::create_dir_all(&story_path)?;
        
        // Create initial README.md file
        let readme_path = story_path.join("README.md");
        let readme_content = format!(
            "# {}\n\n**Type:** {}\n**Status:** {:?}\n\n## Synopsis\n\n[Story synopsis goes here]\n\n## Episodes\n\n[Episodes will be listed here]\n",
            self.display_name(),
            self.story_type,
            self.status
        );
        std::fs::write(&readme_path, readme_content)?;
        
        println!("📁 Created story directory: {}", story_path.display());
        Ok(())
    }

    /// Delete story with directory from filesystem
    pub fn delete_with_directory(&self, force: bool) -> anyhow::Result<()> {
        if !force {
            anyhow::bail!("Use --force to confirm deletion");
        }
        
        let world_root = Self::ensure_world_context()?;
        let story_path = self.get_story_path(&world_root);
        
        // Delete from database first
        self.delete(force)?;
        
        // Delete story directory if it exists
        if story_path.exists() {
            std::fs::remove_dir_all(&story_path)?;
            println!("🗑️ Deleted story directory: {}", story_path.display());
        }
        
        Ok(())
    }

    /// Get total word count including all episodes
    pub fn calculate_total_word_count(&self) -> anyhow::Result<i32> {
        // TODO: This will sum episode word counts when Episode entity exists
        Ok(self.word_count)
    }

    /// Count total stories
    pub fn count_total() -> anyhow::Result<i32> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM stories")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    /// Count stories by status
    pub fn count_by_status() -> anyhow::Result<Vec<(String, i32)>> {
        let conn = Self::get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) FROM stories GROUP BY status ORDER BY status"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }
}