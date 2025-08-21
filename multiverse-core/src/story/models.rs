use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};

/// Story metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub narrator: String,
    pub story_type: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: StoryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

/// Individual episode within a story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub story_name: String,
    pub episode_number: i32,
    pub title: Option<String>,
    pub status: EpisodeStatus,
    pub word_count: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpisodeStatus {
    Draft,
    InProgress,
    Review,
    Published,
}


impl Default for StoryStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for EpisodeStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl Story {
    pub fn new(name: String, narrator: String, story_type: Option<String>) -> Self {
        let story_type = story_type.unwrap_or_else(|| "diary".to_string());
        
        Self {
            name,
            narrator,
            story_type,
            description: None,
            created_at: chrono::Utc::now(),
            status: StoryStatus::Active,
        }
    }
    
    /// Get the story directory path within stories/ with pattern nome_tipo/
    pub fn get_story_path(&self, world_root: &Path) -> std::path::PathBuf {
        let dir_name = format!("{}_{}", self.name, self.story_type);
        world_root.join("stories").join(&dir_name)
    }
}