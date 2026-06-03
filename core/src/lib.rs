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

use serde::{Deserialize, Serialize};

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
        Self {
            workspace: String::from("~/.nexus"),
            default_provider: String::from("openai"),
            default_model: String::from("gpt-4o"),
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
