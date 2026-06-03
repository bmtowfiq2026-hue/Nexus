use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub type TrajectoryId = String;
pub type StepId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub id: TrajectoryId,
    pub session_id: String,
    pub user_message: String,
    pub system_prompt: String,
    pub steps: Vec<TrajectoryStep>,
    pub final_response: String,
    pub success: bool,
    pub total_duration_ms: u64,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStep {
    pub id: StepId,
    pub kind: StepKind,
    pub input: Value,
    pub output: Value,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepKind {
    ProviderCall,
    ToolCall { tool_name: String },
    SkillLookup,
    MemorySearch,
    MemoryStore,
    Thought,
}

pub struct TrajectoryRecorder {
    pub current: Option<Trajectory>,
}

impl TrajectoryRecorder {
    pub fn new() -> Self {
        Self { current: None }
    }

    pub fn begin_turn(&mut self, session_id: &str, user_message: &str, system_prompt: &str) {
        self.current = Some(Trajectory {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_message: user_message.to_string(),
            system_prompt: system_prompt.to_string(),
            steps: Vec::new(),
            final_response: String::new(),
            success: false,
            total_duration_ms: 0,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        });
    }

    pub fn record_step(&mut self, kind: StepKind, input: Value, output: Value, duration_ms: u64, error: Option<String>) {
        if let Some(ref mut t) = self.current {
            t.steps.push(TrajectoryStep {
                id: Uuid::new_v4().to_string(),
                kind,
                input,
                output,
                duration_ms,
                timestamp: Utc::now(),
                error,
            });
        }
    }

    pub fn end_turn(&mut self, response: &str, success: bool) -> Option<Trajectory> {
        if let Some(ref mut t) = self.current {
            t.final_response = response.to_string();
            t.success = success;
            t.total_duration_ms = (Utc::now() - t.created_at).num_milliseconds() as u64;
        }
        self.current.take()
    }

    pub fn get_current_steps(&self) -> Vec<TrajectoryStep> {
        self.current.as_ref().map(|t| t.steps.clone()).unwrap_or_default()
    }
}
