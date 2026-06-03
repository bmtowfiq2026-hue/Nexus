use crate::agent::session::Session;
use crate::trajectory::Trajectory;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SnapshotId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub label: String,
    pub session_state: SessionState,
    pub trajectory: Option<Trajectory>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub parent_id: Option<SnapshotId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: String,
    pub history_count: usize,
    pub turn_count: usize,
    pub agent_status: String,
    pub data: HashMap<String, String>,
}

pub struct CheckpointManager {
    snapshots: Vec<Snapshot>,
    max_snapshots: usize,
}

impl CheckpointManager {
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: Vec::with_capacity(max_snapshots),
            max_snapshots,
        }
    }

    pub fn create_snapshot(&mut self, label: &str, session: &Session, trajectory: Option<Trajectory>) -> SnapshotId {
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let parent_id = self.snapshots.last().map(|s| s.id.clone());

        let snapshot = Snapshot {
            id: id.clone(),
            label: label.to_string(),
            session_state: SessionState {
                session_id: session.id.clone(),
                history_count: session.history.len(),
                turn_count: session.history.len() / 2,
                agent_status: "idle".to_string(),
                data: session.metadata.clone(),
            },
            trajectory,
            created_at: chrono::Utc::now(),
            parent_id,
        };

        self.snapshots.push(snapshot);

        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }

        id
    }

    pub fn rollback(&self, snapshot_id: &str) -> Result<Snapshot> {
        self.snapshots
            .iter()
            .find(|s| s.id == snapshot_id)
            .cloned()
            .ok_or_else(|| crate::NexusError::Other(format!("Snapshot '{}' not found", snapshot_id)))
    }

    pub fn rollback_to_index(&self, index: usize) -> Result<Snapshot> {
        if index >= self.snapshots.len() {
            return Err(crate::NexusError::Other(format!(
                "Snapshot index {} out of bounds (max {})",
                index,
                self.snapshots.len().saturating_sub(1)
            )));
        }
        Ok(self.snapshots[index].clone())
    }

    pub fn diff(&self, from_id: &str, to_id: &str) -> Result<String> {
        let from = self
            .snapshots
            .iter()
            .find(|s| s.id == from_id)
            .ok_or_else(|| crate::NexusError::Other("From snapshot not found".to_string()))?;
        let to = self
            .snapshots
            .iter()
            .find(|s| s.id == to_id)
            .ok_or_else(|| crate::NexusError::Other("To snapshot not found".to_string()))?;

        let mut diff = String::new();
        diff.push_str(&format!("--- Snapshot: {}\n", from.label));
        diff.push_str(&format!("+++ Snapshot: {}\n", to.label));
        diff.push_str(&format!(
            "@@ turns: {} -> {} @@\n",
            from.session_state.turn_count, to.session_state.turn_count
        ));
        diff.push_str(&format!(
            "  history entries: {} -> {}",
            from.session_state.history_count, to.session_state.history_count
        ));

        Ok(diff)
    }

    pub fn list_snapshots(&self) -> &[Snapshot] {
        &self.snapshots
    }

    pub fn latest(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }
}
