use crate::agent::session::Session;
use crate::agent::AgentStatus;
use crate::checkpoint::CheckpointManager;
use crate::memory::graph::GraphMemory;
use crate::memory::summarizer::MemorySummarizer;
use crate::memory::vector::{compute_embedding, VectorStore};
use crate::memory::{MemoryEntry, MemoryStore, MemoryType};
use crate::providers::{Provider, ProviderRequest};
use crate::skills::refiner::SkillRefiner;
use crate::skills::SkillEngine;
use crate::tools::ToolDispatcher;
use crate::trajectory::extractor::SkillExtractor;
use crate::trajectory::recorder::{StepKind, TrajectoryRecorder};
use crate::NexusConfig;
use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

pub struct AgentLoop {
    pub config: NexusConfig,
    pub provider: Arc<dyn Provider + Send + Sync>,
    pub tools: Arc<ToolDispatcher>,
    pub skills: Arc<SkillEngine>,
    pub memory: Arc<std::sync::Mutex<MemoryStore>>,
    pub vector_store: Arc<VectorStore>,
    pub graph_memory: Arc<std::sync::Mutex<GraphMemory>>,
    pub trajectory_recorder: Arc<std::sync::Mutex<TrajectoryRecorder>>,
    pub checkpoint_manager: Arc<std::sync::Mutex<CheckpointManager>>,
    pub status: AgentStatus,
}

impl AgentLoop {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: NexusConfig,
        provider: Arc<dyn Provider + Send + Sync>,
        tools: Arc<ToolDispatcher>,
        skills: Arc<SkillEngine>,
        memory: Arc<std::sync::Mutex<MemoryStore>>,
        vector_store: Arc<VectorStore>,
        graph_memory: Arc<std::sync::Mutex<GraphMemory>>,
    ) -> Self {
        Self {
            config,
            provider,
            tools,
            skills,
            memory,
            vector_store,
            graph_memory,
            trajectory_recorder: Arc::new(std::sync::Mutex::new(TrajectoryRecorder::new())),
            checkpoint_manager: Arc::new(std::sync::Mutex::new(CheckpointManager::new(100))),
            status: AgentStatus::Idle,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub async fn run_turn(
        &mut self,
        session: &mut Session,
        user_message: &str,
    ) -> Result<String> {
        self.status = AgentStatus::Thinking;
        info!("Agent thinking for session {}", session.id);

        self.checkpoint_manager
            .lock().unwrap()
            .create_snapshot("before_turn", session, None);

        let skill_context = self.skills.get_active_context(session).await?;

        let recent_memory = self.memory.lock().unwrap().search_relevant(&session.id, user_message, 5)?;

        let embedding = compute_embedding(user_message);
        let vector_results = self.vector_store.search_similar(&embedding, 3)?;

        let vector_context: Vec<String> = vector_results
            .iter()
            .filter_map(|(id, _)| self.vector_store.get_content(id))
            .collect();

        let system_prompt = self.build_system_prompt(&skill_context, &recent_memory, &vector_context);

        {
            let mut recorder = self.trajectory_recorder.lock().unwrap();
            recorder.begin_turn(&session.id, user_message, &system_prompt);
        }

        self.status = AgentStatus::ExecutingTool;

        let request = ProviderRequest {
            system_prompt,
            messages: session.history.clone(),
            user_message: user_message.to_string(),
            tools: self.tools.get_definitions(),
        };

        let provider_start = std::time::Instant::now();
        let response = self.provider.complete(request).await?;
        let provider_duration = provider_start.elapsed().as_millis() as u64;

        {
            let mut recorder = self.trajectory_recorder.lock().unwrap();
            recorder.record_step(
                StepKind::ProviderCall,
                serde_json::json!({"model": self.config.default_model, "turns": session.history.len()}),
                serde_json::json!({"length": response.content.len(), "usage": response.usage.total_tokens}),
                provider_duration,
                None,
            );
        }

        session.add_turn(user_message, &response.content);

        self.memory.lock().unwrap().store(MemoryEntry::new(
            session.id.clone(),
            format!("User: {} | Assistant: {}", user_message, response.content),
            MemoryType::Conversation,
        ))?;

        let emb = compute_embedding(&format!("{} {}", user_message, response.content));
        let mut meta = HashMap::new();
        meta.insert("session".to_string(), session.id.clone());
        self.vector_store.store_embedding(
            &uuid::Uuid::new_v4().to_string(),
            emb,
            &response.content,
            meta,
        )?;

        {
            let mut graph = self.graph_memory.lock().unwrap();
            graph.extract_entities_from_text(user_message, &session.id);
            graph.extract_entities_from_text(&response.content, &session.id);
        }

        let trajectory = {
            let mut recorder = self.trajectory_recorder.lock().unwrap();
            recorder.end_turn(&response.content, true)
        };

        if let Some(ref traj) = trajectory {
            if let Some(skill) = SkillExtractor::extract_from_trajectory(traj) {
                self.memory.lock().unwrap().store(MemoryEntry::new(
                    session.id.clone(),
                    format!("Auto-generated skill: {} - {}", skill.name, skill.description),
                    MemoryType::Skill,
                ))?;
                info!("Extracted skill '{}' from trajectory {}", skill.name, traj.id);
            }

            let elapsed = provider_duration;
            self.memory.lock().unwrap().store(MemoryEntry::new(
                session.id.clone(),
                format!("Trajectory: {} ({} steps, {}ms, success={})",
                    traj.id, traj.steps.len(), elapsed, traj.success),
                MemoryType::Trajectory,
            ))?;
        }

        {
            let all_entries = self.memory.lock().unwrap().get_all().to_vec();
            if MemorySummarizer::should_summarize(&all_entries, 50) {
                let conv_entries: Vec<MemoryEntry> = all_entries
                    .iter()
                    .filter(|e| matches!(e.entry_type, MemoryType::Conversation))
                    .cloned()
                    .collect();

                if conv_entries.len() > 50 {
                    let summary = MemorySummarizer::summarize_conversation(
                        &conv_entries[..conv_entries.len().min(100)],
                        2000,
                    )?;
                    self.memory.lock().unwrap().store(summary)?;
                    info!("Memory summarization triggered for session {}", session.id);
                }
            }
        }

        let _snapshot_id = self.checkpoint_manager
            .lock().unwrap()
            .create_snapshot("after_turn", session, trajectory);

        self.status = AgentStatus::Idle;

        Ok(response.content)
    }

    pub fn handle_failure(&self, skill_name: &str, user_message: &str, error: &str) -> Result<()> {
        let skills = self.skills.list_skills();
        if let Some(existing) = skills.iter().find(|s| s.name == skill_name) {
            let refined = SkillRefiner::refine_from_failure(existing, user_message, error)?;
            info!("Refined skill '{}' to v{} based on error", skill_name, refined.version);
        }
        Ok(())
    }

    fn build_system_prompt(&self, skill_context: &str, memory: &[String], vector_memory: &[String]) -> String {
        let mut prompt = String::from(
            "You are Nexus, an autonomous AI agent with persistent learning capabilities.\n\n"
        );

        if !skill_context.is_empty() {
            prompt.push_str("=== ACTIVE SKILLS ===\n");
            prompt.push_str(skill_context);
            prompt.push('\n');
        }

        if !memory.is_empty() || !vector_memory.is_empty() {
            prompt.push_str("=== RELEVANT MEMORY ===\n");
            for m in memory {
                prompt.push_str(&format!("- {}\n", m));
            }
            for m in vector_memory {
                prompt.push_str(&format!("- [semantic] {}\n", m));
            }
            prompt.push('\n');
        }

        prompt.push_str(
            "You learn from every interaction. Each conversation helps you understand user preferences better.\n"
        );
        prompt.push_str("You have tools available. Use them when appropriate.\n");
        prompt.push_str("When a tool fails, learn from the error and try alternative approaches.\n");

        prompt
    }
}
