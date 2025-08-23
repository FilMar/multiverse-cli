use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};

// WorldMeta is now defined in config.rs - import it
pub use super::config::WorldMeta;

// These structs are now defined in config.rs - import them
pub use super::config::{VisualIdentity, GlobalConfig};

// Default implementations are now in config.rs

/// Main World struct for managing multiverse projects
#[derive(Debug)]
pub struct World {
    pub path: std::path::PathBuf,
    pub meta: WorldMeta,
}

impl WorldMeta {
    // new() method is now in config.rs
    
    /// Load world metadata from config.toml file (DEPRECATED - use WorldConfig::load instead)
    pub fn load(world_path: &Path) -> Result<Self> {
        let config = super::config::WorldConfig::load()?;
        Ok(config.world)
    }
    
    /// Save world metadata to config.toml file (DEPRECATED - use WorldConfig::save instead)
    pub fn save(&self, world_path: &Path) -> Result<()> {
        let mut config = super::config::WorldConfig::load()
            .unwrap_or_else(|_| super::config::WorldConfig::new(self.name.clone(), self.description.clone()));
        
        config.world = self.clone();
        config.save(world_path)
    }
}

// Core interface
impl World {
    /// Initialize a new world project
    pub fn init(name: String, description: Option<String>, aesthetic: Option<String>, from_git: Option<String>, merge: bool) -> Result<Self> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        
        if current_dir.join(".multiverse").exists() {
            anyhow::bail!("Already in a multiverse project. Use 'multiverse info' to see details.");
        }
        
        if !merge && Self::is_directory_not_empty(&current_dir)? {
            anyhow::bail!("Directory is not empty. Use --merge to initialize in existing directory, or run in an empty directory.");
        }
        
        if let Some(repo_url) = from_git {
            Self::init_from_git(repo_url, current_dir)
        } else {
            Self::init_local(name, description, aesthetic, merge, current_dir)
        }
    }

    /// Get current world info
    pub fn info() -> Result<Self> {
        let world_root = super::config::WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")?;
        
        let config = super::config::WorldConfig::load()
            .context("Failed to load world configuration")?;
        let meta = config.world;
        
        Ok(World {
            path: world_root,
            meta,
        })
    }

    /// Pull updates from Git
    pub fn pull(&self) -> Result<()> {
        let repo = super::git::WorldGitRepo::new(&self.path)?;
        repo.pull()
    }

    /// Push changes to Git
    pub fn push(&self) -> Result<()> {
        let repo = super::git::WorldGitRepo::new(&self.path)?;
        repo.push()
    }

    /// Show Git status
    pub fn status(&self) -> Result<super::git::WorldGitStatus> {
        let repo = super::git::WorldGitRepo::new(&self.path)?;
        repo.status()
    }

    /// Import SQL scripts
    pub fn import_sql(file: Option<String>, all: bool) -> Result<()> {
        let world_root = super::config::WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")?;
        
        if all {
            Self::import_all_sql(&world_root)
        } else if let Some(file_path) = file {
            Self::import_sql_file_or_dir(&world_root, file_path)
        } else {
            Self::show_import_help()
        }
    }
}

// Private utility functions
impl World {
    fn init_from_git(repo_url: String, current_dir: std::path::PathBuf) -> Result<Self> {
        super::git::WorldGitRepo::clone_from(&repo_url, &current_dir)?;
        
        // Check if cloned repository has .multiverse setup
        let multiverse_dir = current_dir.join(".multiverse");
        let needs_multiverse_setup = !multiverse_dir.exists();
        
        if needs_multiverse_setup {
            // Repository doesn't have .multiverse setup - we need to initialize it
            std::fs::create_dir_all(&multiverse_dir)
                .context("Failed to create .multiverse directory")?;
                
            // Create default config based on repository name
            let repo_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown World")
                .to_string();
                
            let config = super::config::WorldConfig::new(repo_name, None);
            config.save(&current_dir)
                .context("Failed to save configuration")?;
        }
        
        // Verify database exists or initialize it
        if let Ok(db_path) = super::config::WorldConfig::get_database_path() {
            if !db_path.exists() {
                super::database::init_world_database(&db_path)
                    .context("Failed to initialize world database")?;
            }
        }
        
        // Generate LLM extraction guide if this appears to be existing content without CLI setup
        if needs_multiverse_setup && Self::should_generate_extraction_guide(&current_dir)? {
            Self::generate_extraction_guide(&current_dir)
                .context("Failed to generate LLM extraction guide")?;
        }
        
        let config = super::config::WorldConfig::load()
            .context("Failed to load world configuration")?;
        let meta = config.world;
        
        Ok(World {
            path: current_dir,
            meta,
        })
    }

    fn init_local(name: String, description: Option<String>, aesthetic: Option<String>, merge: bool, current_dir: std::path::PathBuf) -> Result<Self> {
        let multiverse_dir = current_dir.join(".multiverse");
        std::fs::create_dir_all(&multiverse_dir)
            .context("Failed to create .multiverse directory")?;
        
        let stories_dir = current_dir.join("stories");
        if !stories_dir.exists() {
            std::fs::create_dir_all(&stories_dir)
                .context("Failed to create stories directory")?;
        }
        
        let config = super::config::WorldConfig::new(name.clone(), description.clone());
        config.save(&current_dir)
            .context("Failed to save configuration")?;
        
        Self::create_fundamental_files(&current_dir, &name, description.as_deref(), merge)
            .context("Failed to create fundamental files")?;
        
        // Generate LLM extraction guide if we're merging with existing content
        if merge && Self::should_generate_extraction_guide(&current_dir)? {
            Self::generate_extraction_guide(&current_dir)
                .context("Failed to generate LLM extraction guide")?;
        }
        
        let db_path = multiverse_dir.join("world.db");
        super::database::init_world_database(&db_path)
            .context("Failed to initialize world database")?;
        
        let world_repo = super::git::WorldGitRepo::new(&current_dir)?;
        world_repo.init()?;
        
        let mut meta = WorldMeta::new(name, description);
        if let Some(aesthetic_value) = aesthetic {
            meta.visual_identity = VisualIdentity {
                estetica: aesthetic_value,
                descrizione: "Custom aesthetic".to_string(),
            };
        }
        
        Ok(World {
            path: current_dir,
            meta,
        })
    }

    fn is_directory_not_empty(dir: &Path) -> Result<bool> {
        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            if !name_str.starts_with('.') {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn create_fundamental_files(world_path: &Path, _name: &str, _description: Option<&str>, merge: bool) -> Result<()> {
        let files = ["00_ESSENTIAL.md", "01_HISTORY.md", "README.md"];
        for file_name in &files {
            let file_path = world_path.join(file_name);
            if merge && file_path.exists() {
                continue;
            }
            std::fs::write(&file_path, "").with_context(|| format!("Failed to create {}", file_path.display()))?;
        }
        Ok(())
    }

    fn import_all_sql(world_root: &Path) -> Result<()> {
        use rusqlite::Connection;
        
        let sql_dir = world_root.join("sql");
        if !sql_dir.exists() {
            anyhow::bail!("No sql/ directory found. Create it and add .sql files to import data");
        }

        let db_path = super::config::WorldConfig::get_database_path()?;
        let conn = Connection::open(&db_path).context("Failed to open database")?;
        
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&sql_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                files.push(path);
            }
        }
        
        files.sort();
        
        if files.is_empty() {
            anyhow::bail!("No .sql files found in sql/");
        }

        let file_count = files.len();
        
        for sql_file in files {
            Self::execute_sql_file(&conn, &sql_file)?;
        }
        
        println!("✅ Imported {} SQL files", file_count);
        Ok(())
    }

    fn import_sql_file_or_dir(world_root: &Path, file_path: String) -> Result<()> {
        use rusqlite::Connection;
        
        let sql_path = std::path::PathBuf::from(file_path);
        if !sql_path.exists() {
            anyhow::bail!("SQL file not found: {}", sql_path.display());
        }
        
        let db_path = super::config::WorldConfig::get_database_path()?;
        let conn = Connection::open(&db_path).context("Failed to open database")?;
        
        if sql_path.is_dir() {
            let mut files = Vec::new();
            for entry in std::fs::read_dir(&sql_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                    files.push(path);
                }
            }
            
            files.sort();
            
            for sql_file in files {
                Self::execute_sql_file(&conn, &sql_file)?;
            }
        } else {
            Self::execute_sql_file(&conn, &sql_path)?;
        }
        
        Ok(())
    }

    fn execute_sql_file(conn: &rusqlite::Connection, file_path: &Path) -> Result<()> {
        let sql_content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read SQL file: {}", file_path.display()))?;
        
        println!("   📄 Executing {}...", file_path.file_name().unwrap_or_default().to_string_lossy());
        
        conn.execute_batch(&sql_content)
            .with_context(|| format!("Failed to execute SQL from: {}", file_path.display()))?;
        
        Ok(())
    }

    fn show_import_help() -> Result<()> {
        println!("📥 Import data from SQL scripts");
        println!("   --file <path>  : Import from specific file or directory");
        println!("   --all          : Import all files from sql/");
        Ok(())
    }
    
    /// Check if we should generate an extraction guide (has content that needs processing)
    fn should_generate_extraction_guide(world_path: &Path) -> Result<bool> {
        // Look for signs of existing narrative content that would benefit from extraction
        let stories_dir = world_path.join("stories");
        let has_stories_dir = stories_dir.exists() && stories_dir.is_dir();
        
        if has_stories_dir {
            // Check if stories directory has any .md files
            if let Ok(entries) = std::fs::read_dir(&stories_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Check subdirectories for .md files
                        if let Ok(sub_entries) = std::fs::read_dir(&path) {
                            for sub_entry in sub_entries.flatten() {
                                if sub_entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                                    return Ok(true);
                                }
                            }
                        }
                    } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        return Ok(true);
                    }
                }
            }
        }
        
        // Also check for any top-level .md files that might be narrative content
        if let Ok(entries) = std::fs::read_dir(world_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    // Skip known non-narrative files
                    if !["README.md", "CLAUDE.md", "EXTRACTION_GUIDE.md"].contains(&filename) {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    /// Generate the LLM extraction guide
    fn generate_extraction_guide(world_path: &Path) -> Result<()> {
        use crate::templates::EXTRACTION_GUIDE;
        
        let guide_path = world_path.join("EXTRACTION_GUIDE.md");
        std::fs::write(&guide_path, EXTRACTION_GUIDE)
            .with_context(|| format!("Failed to write extraction guide to {}", guide_path.display()))?;
        
        // Create sql directory if it doesn't exist
        let sql_dir = world_path.join("sql");
        if !sql_dir.exists() {
            std::fs::create_dir_all(&sql_dir)
                .with_context(|| format!("Failed to create sql directory at {}", sql_dir.display()))?;
        }
        
        println!("📋 Generated EXTRACTION_GUIDE.md");
        println!("   This file contains instructions for Claude/LLM to analyze existing content");
        println!("   and generate SQL files for database import.");
        println!("   📁 Also created sql/ directory for generated SQL files");
        
        Ok(())
    }
}
