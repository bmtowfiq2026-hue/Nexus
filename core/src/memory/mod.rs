pub mod store;
pub mod fts;
pub mod vector;
pub mod graph;
pub mod summarizer;

pub use store::MemoryStore;

pub type MemoryEntryId = String;

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub id: MemoryEntryId,
    pub session_id: String,
    pub content: String,
    pub entry_type: MemoryType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum MemoryType {
    Conversation,
    Fact,
    Preference,
    Skill,
    Error,
    Observation,
    Summary,
    Trajectory,
}
