use super::{MemoryEntry, MemoryType};
use crate::Result;
use std::collections::HashMap;

pub struct MemoryStore {
    entries: Vec<MemoryEntry>,
    store_path: String,
}

impl MemoryStore {
    pub fn new(store_path: &str) -> Self {
        Self {
            entries: Vec::new(),
            store_path: store_path.to_string(),
        }
    }

    pub fn store(&mut self, entry: MemoryEntry) -> Result<()> {
        self.entries.push(entry);
        Ok(())
    }

    pub fn search_relevant(&self, _session_id: &str, query: &str, limit: usize) -> Result<Vec<String>> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<String> = self
            .entries
            .iter()
            .filter(|e| e.content.to_lowercase().contains(&query_lower))
            .map(|e| e.content.clone())
            .collect();
        results.truncate(limit);
        Ok(results)
    }

    pub fn get_all(&self) -> &[MemoryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl MemoryEntry {
    pub fn new(session_id: String, content: String, entry_type: MemoryType) -> Self {
        use uuid::Uuid;
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            content,
            entry_type,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }
}
