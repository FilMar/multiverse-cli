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
    pub template_default: String,       // "diary"
    pub story_types: std::collections::HashMap<String, StoryTypeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTypeConfig {
    pub display_name: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub defaults: std::collections::HashMap<String, serde_json::Value>,
    pub numbering_format: String,
}

impl StoryTypeConfig {
    /// Validate and build metadata from user input
    pub fn build_metadata(&self, set_args: Vec<(String, String)>) -> anyhow::Result<std::collections::HashMap<String, serde_json::Value>> {
        let mut metadata = std::collections::HashMap::new();
        
        // Apply defaults first
        for (key, value) in &self.defaults {
            metadata.insert(key.clone(), value.clone());
        }
        
        // Apply user-provided values
        for (key, value) in set_args {
            // Validate field is allowed for this story type
            self.validate_field(&key)?;
            metadata.insert(key, serde_json::Value::String(value));
        }
        
        // Validate all required fields are present
        let missing_fields = self.missing_required_fields(&metadata);
        if !missing_fields.is_empty() {
            return Err(anyhow::anyhow!(
                "Missing required fields: {}",
                missing_fields.into_iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
        
        Ok(metadata)
    }
    
    /// Check if a field is valid for this story type
    pub fn validate_field(&self, field_name: &str) -> anyhow::Result<()> {
        if !self.required_fields.contains(&field_name.to_string()) && !self.optional_fields.contains(&field_name.to_string()) {
            return Err(anyhow::anyhow!(
                "Field '{}' is not valid for this story type. Valid fields: {}",
                field_name,
                [self.required_fields.clone(), self.optional_fields.clone()].concat().join(", ")
            ));
        }
        Ok(())
    }
    
    /// Get missing required fields from metadata
    pub fn missing_required_fields(&self, metadata: &std::collections::HashMap<String, serde_json::Value>) -> Vec<&String> {
        self.required_fields
            .iter()
            .filter(|field| !metadata.contains_key(*field))
            .collect()
    }
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
        let mut story_types = std::collections::HashMap::new();
        
        // Diary story type
        let mut diary_defaults = std::collections::HashMap::new();
        diary_defaults.insert("signature".to_string(), serde_json::Value::String("F.M.".to_string()));
        
        story_types.insert("diary".to_string(), StoryTypeConfig {
            display_name: "Personal Diary".to_string(),
            required_fields: vec!["narrator".to_string()],
            optional_fields: vec!["signature".to_string(), "perspective".to_string()],
            defaults: diary_defaults,
            numbering_format: "001".to_string(),
        });
        
        // Book story type
        story_types.insert("book".to_string(), StoryTypeConfig {
            display_name: "Book/Novel".to_string(),
            required_fields: vec!["author".to_string()],
            optional_fields: vec!["genre".to_string(), "series_name".to_string(), "volume".to_string()],
            defaults: std::collections::HashMap::new(),
            numbering_format: "Chapter %d".to_string(),
        });
        
        Self {
            formato_numerazione: "001".to_string(),
            template_default: "diary".to_string(),
            story_types,
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
    
    /// Get a specific story type configuration
    pub fn get_story_type(&self, type_name: &str) -> Result<&StoryTypeConfig> {
        self.world.global_config.story_types.get(type_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown story type '{}'. Use 'multiverse story types' to see available types", type_name))
    }
    
    /// List all available story types
    pub fn list_story_types(&self) -> &std::collections::HashMap<String, StoryTypeConfig> {
        &self.world.global_config.story_types
    }
}
