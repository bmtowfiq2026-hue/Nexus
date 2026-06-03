pub mod loop_;
pub mod session;

pub use loop_::AgentLoop;
pub use session::{AgentState, Session};

pub type AgentId = String;
pub type SessionId = String;
pub type TurnId = String;

#[derive(Debug, Clone)]
pub enum AgentStatus {
    Idle,
    Thinking,
    ExecutingTool,
    WaitingForInput,
    Error(String),
}
