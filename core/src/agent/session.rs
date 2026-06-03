use crate::agent::{SessionId, TurnId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub agent_id: String,
    pub channel: String,
    pub user_id: String,
    pub history: Vec<Turn>,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub id: TurnId,
    pub role: TurnRole,
    pub content: String,
    pub tool_calls: Vec<ToolCallRecord>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurnRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub result: serde_json::Value,
    pub duration_ms: u64,
}

impl Session {
    pub fn new(id: SessionId, channel: String, user_id: String) -> Self {
        Self {
            id,
            agent_id: String::from("default"),
            channel,
            user_id,
            history: Vec::new(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn add_turn(&mut self, user_msg: &str, assistant_msg: &str) {
        use uuid::Uuid;
        self.history.push(Turn {
            id: Uuid::new_v4().to_string(),
            role: TurnRole::User,
            content: user_msg.to_string(),
            tool_calls: Vec::new(),
            timestamp: chrono::Utc::now(),
        });
        self.history.push(Turn {
            id: Uuid::new_v4().to_string(),
            role: TurnRole::Assistant,
            content: assistant_msg.to_string(),
            tool_calls: Vec::new(),
            timestamp: chrono::Utc::now(),
        });
        self.updated_at = chrono::Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: String,
    pub name: String,
    pub persona: String,
    pub config: serde_json::Value,
}
