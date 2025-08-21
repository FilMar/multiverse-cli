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
        let meta_path = world_path.join(".world.json");
        let content = std::fs::read_to_string(&meta_path)
            .with_context(|| format!("Failed to read {}", meta_path.display()))?;
        
        let meta: WorldMeta = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", meta_path.display()))?;
        
        Ok(meta)
    }
    
    /// Save world metadata to .world.json file
    pub fn save(&self, world_path: &Path) -> Result<()> {
        let meta_path = world_path.join(".world.json");
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize world metadata")?;
        
        std::fs::write(&meta_path, content)
            .with_context(|| format!("Failed to write {}", meta_path.display()))?;
        
        Ok(())
    }
}