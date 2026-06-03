use crate::Result;
use serde::{Deserialize, Serialize};

pub type EntryId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: EntryId,
    pub parent_id: Option<EntryId>,
    pub action: String,
    pub actor: String,
    pub target: String,
    pub details: serde_json::Value,
    pub hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct AuditTrail {
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn record(&mut self, action: &str, actor: &str, target: &str, details: serde_json::Value) -> Result<EntryId> {
        use sha2::{Digest, Sha256};
        use uuid::Uuid;

        let id = Uuid::new_v4().to_string();
        let parent_hash = self.entries.last().map(|e| e.hash.clone()).unwrap_or_default();
        let hash_input = format!("{}:{}:{}:{}:{}", id, action, actor, target, parent_hash);
        let hash = format!("{:x}", Sha256::digest(hash_input.as_bytes()));

        let entry = AuditEntry {
            id: id.clone(),
            parent_id: self.entries.last().map(|e| e.id.clone()),
            action: action.to_string(),
            actor: actor.to_string(),
            target: target.to_string(),
            details,
            hash,
            timestamp: chrono::Utc::now(),
        };

        self.entries.push(entry);
        Ok(id)
    }

    pub fn get_chain(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn verify_chain(&self) -> bool {
        use sha2::{Digest, Sha256};
        for i in 1..self.entries.len() {
            let prev = &self.entries[i - 1];
            let curr = &self.entries[i];
            let expected_hash = format!("{:x}", Sha256::digest(
                format!("{}:{}:{}:{}:{}", curr.id, curr.action, curr.actor, curr.target, prev.hash).as_bytes()
            ));
            if curr.hash != expected_hash {
                return false;
            }
        }
        true
    }
}
