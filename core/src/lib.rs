use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod agent;
pub mod audit;
pub mod checkpoint;
pub mod identity;
pub mod memory;
pub mod network;
pub mod providers;
pub mod sandbox;
pub mod skills;
pub mod tools;
pub mod trajectory;

pub type Result<T> = std::result::Result<T, NexusError>;

#[derive(Debug, thiserror::Error)]
pub enum NexusError {
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Tool error: {0}")]
    Tool(String),
    #[error("Memory error: {0}")]
    Memory(String),
    #[error("Skill error: {0}")]
    Skill(String),
    #[error("Sandbox error: {0}")]
    Sandbox(String),
    #[error("Identity error: {0}")]
    Identity(String),
    #[error("Audit error: {0}")]
    Audit(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Trajectory error: {0}")]
    Trajectory(String),
    #[error("Checkpoint error: {0}")]
    Checkpoint(String),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    pub workspace: String,
    pub default_provider: String,
    pub default_model: String,
    pub api_keys: HashMap<String, String>,
    pub memory: MemoryConfig,
    pub sandbox: SandboxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub store_path: String,
    pub vector_dimensions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub max_memory_mb: usize,
    pub max_cpu_cores: usize,
    pub network_access: bool,
}

impl Default for NexusConfig {
    fn default() -> Self {
        let mut api_keys = HashMap::new();
        api_keys.insert("openai".to_string(), String::new());
        api_keys.insert("anthropic".to_string(), String::new());
        Self {
            workspace: String::from("~/.nexus"),
            default_provider: String::from("demo"),
            default_model: String::from("demo"),
            api_keys,
            memory: MemoryConfig {
                store_path: String::from("~/.nexus/memory"),
                vector_dimensions: 1536,
            },
            sandbox: SandboxConfig {
                enabled: true,
                max_memory_mb: 512,
                max_cpu_cores: 2,
                network_access: false,
            },
        }
    }
}

fn get_config_path() -> String {
    if let Ok(path) = std::env::var("NEXUS_CONFIG_PATH") {
        return path;
    }
    if let Some(data_dir) = dirs::config_dir() {
        let path = data_dir.join("nexus").join("nexus.json");
        if path.exists() {
            return path.to_string_lossy().to_string();
        }
    }
    shellexpand::tilde("~/.nexus/nexus.json").to_string()
}

impl NexusConfig {
    pub fn load() -> Self {
        let config_path = get_config_path();
        if std::path::Path::new(&config_path).exists() {
            if let Ok(data) = std::fs::read_to_string(&config_path) {
                if let Ok(cfg) = serde_json::from_str::<NexusConfig>(&data) {
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path();
        if let Some(parent) = std::path::Path::new(&config_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, data)?;
        Ok(())
    }

    pub fn get_api_key(&self, provider: &str) -> Option<String> {
        self.api_keys.get(provider).and_then(|k| {
            if k.is_empty() {
                std::env::var(format!("{}_API_KEY", provider.to_uppercase())).ok()
            } else {
                Some(k.clone())
            }
        })
    }

    pub fn workspace_dir(&self) -> String {
        shellexpand::tilde(&self.workspace).to_string()
    }
}
