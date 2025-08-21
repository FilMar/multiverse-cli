use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};

/// World metadata stored in .world.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMeta {
    pub name: String,
    pub description: Option<String>,
    pub visual_identity: Option<VisualIdentity>,
    pub global_config: Option<GlobalConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualIdentity {
    pub estetica: String,        // "fantasy", "moderna", "storica", "cyberpunk"
    pub descrizione: String,     // "Quaderni anticati con inchiostro seppia"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub formato_numerazione: String,    // "001", "1", "I"
    pub template_default: String,       // "diario_personale"
    pub categorie: CategoryRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRules {
    pub diari: CategoryConfig,
    pub extra: CategoryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryConfig {
    pub firma_pubblica_default: Option<String>,  // "F.M." per diari, None per extra
    pub tipi_permessi: Vec<String>,
}

impl Default for VisualIdentity {
    fn default() -> Self {
        Self {
            estetica: "moderna".to_string(),
            descrizione: "Interfaccia pulita e minimalista".to_string(),
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            formato_numerazione: "001".to_string(),
            template_default: "diario_personale".to_string(),
            categorie: CategoryRules {
                diari: CategoryConfig {
                    firma_pubblica_default: Some("F.M.".to_string()),
                    tipi_permessi: vec![
                        "diario_personale".to_string(), 
                        "log_personale".to_string()
                    ],
                },
                extra: CategoryConfig {
                    firma_pubblica_default: None,
                    tipi_permessi: vec![
                        "lettera".to_string(),
                        "documento_ufficiale".to_string(),
                        "trascrizione".to_string(),
                        "rapporto".to_string(),
                    ],
                },
            },
        }
    }
}

/// Main World struct for managing multiverse projects
#[derive(Debug)]
pub struct World {
    pub path: std::path::PathBuf,
    pub meta: WorldMeta,
}

impl WorldMeta {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            visual_identity: Some(VisualIdentity::default()),
            global_config: Some(GlobalConfig::default()),
        }
    }
    
    /// Load world metadata from .world.json file
    pub fn load(world_path: &Path) -> Result<Self> {
        let meta_path = world_path.join(".multiverse/config.toml");
        let content = std::fs::read_to_string(&meta_path)
            .with_context(|| format!("Failed to read {}", meta_path.display()))?;
        
        let meta: WorldMeta = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", meta_path.display()))?;
        
        Ok(meta)
    }
    
    /// Save world metadata to .world.json file
    pub fn save(&self, world_path: &Path) -> Result<()> {
        let meta_path = world_path.join(".multiverse/config.toml");
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize world metadata")?;
        
        std::fs::write(&meta_path, content)
            .with_context(|| format!("Failed to write {}", meta_path.display()))?;
        
        Ok(())
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
        
        let meta = WorldMeta::load(&world_root)
            .context("Failed to load world metadata")?;
        
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
        
        // Verify database exists or initialize it
        if let Ok(db_path) = super::config::WorldConfig::get_database_path() {
            if !db_path.exists() {
                super::database::init_world_database(&db_path)
                    .context("Failed to initialize world database")?;
            }
        }
        
        let meta = WorldMeta::load(&current_dir)
            .context("Failed to load world metadata from cloned repository")?;
        
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
        
        let db_path = multiverse_dir.join("world.db");
        super::database::init_world_database(&db_path)
            .context("Failed to initialize world database")?;
        
        let world_repo = super::git::WorldGitRepo::new(&current_dir)?;
        world_repo.init()?;
        
        let mut meta = WorldMeta::new(name, description);
        if let Some(aesthetic_value) = aesthetic {
            meta.visual_identity = Some(VisualIdentity {
                estetica: aesthetic_value,
                descrizione: "Custom aesthetic".to_string(),
            });
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
}
