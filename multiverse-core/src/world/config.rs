use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};

/// Configuration stored in .multiverse/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub world: WorldMeta,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub export: ExportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMeta {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub visual_identity: VisualIdentity,
    #[serde(default)]
    pub global_config: GlobalConfig,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitConfig {
    pub auto_commit: Option<bool>,
    pub commit_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExportConfig {
    pub default_format: Option<String>,
    pub output_dir: Option<String>,
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

impl WorldMeta {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            visual_identity: VisualIdentity::default(),
            global_config: GlobalConfig::default(),
        }
    }
}

impl WorldConfig {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            world: WorldMeta::new(name, description),
            git: GitConfig::default(),
            export: ExportConfig::default(),
        }
    }
    
    /// Find .multiverse directory by walking up the directory tree
    pub fn find_multiverse_dir() -> Result<std::path::PathBuf> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        
        let mut dir = current_dir.as_path();
        loop {
            let multiverse_dir = dir.join(".multiverse");
            if multiverse_dir.exists() && multiverse_dir.is_dir() {
                return Ok(multiverse_dir);
            }
            
            match dir.parent() {
                Some(parent) => dir = parent,
                None => break,
            }
        }
        
        anyhow::bail!("No .multiverse directory found in current directory or any parent directory. Run 'multiverse init' to create one.");
    }
    
    /// Load configuration from .multiverse/config.toml
    pub fn load() -> Result<Self> {
        let multiverse_dir = Self::find_multiverse_dir()?;
        let config_path = multiverse_dir.join("config.toml");
        
        if !config_path.exists() {
            anyhow::bail!("No config.toml found in .multiverse directory");
        }
        
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;
        
        let config: WorldConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", config_path.display()))?;
        
        Ok(config)
    }
    
    /// Save configuration to .multiverse/config.toml
    pub fn save(&self, world_root: &Path) -> Result<()> {
        let multiverse_dir = world_root.join(".multiverse");
        std::fs::create_dir_all(&multiverse_dir)
            .with_context(|| format!("Failed to create .multiverse directory at {}", multiverse_dir.display()))?;
        
        let config_path = multiverse_dir.join("config.toml");
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write {}", config_path.display()))?;
        
        Ok(())
    }
    
    /// Get the world root directory (parent of .multiverse)
    pub fn get_world_root() -> Result<std::path::PathBuf> {
        let multiverse_dir = Self::find_multiverse_dir()?;
        Ok(multiverse_dir.parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid .multiverse directory structure"))?
            .to_path_buf())
    }
    
    /// Get the database path (.multiverse/world.db)
    pub fn get_database_path() -> Result<std::path::PathBuf> {
        let multiverse_dir = Self::find_multiverse_dir()?;
        Ok(multiverse_dir.join("world.db"))
    }
}
