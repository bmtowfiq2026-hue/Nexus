use super::recorder::{StepKind, Trajectory, TrajectoryStep};
use crate::skills::Skill;
use serde_json::Value;

pub struct SkillExtractor;

impl SkillExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_from_trajectory(trajectory: &Trajectory) -> Option<Skill> {
        if !trajectory.success {
            return None;
        }

        let tool_steps: Vec<&TrajectoryStep> = trajectory
            .steps
            .iter()
            .filter(|s| matches!(s.kind, StepKind::ToolCall { .. }))
            .collect();

        if tool_steps.is_empty() {
            return None;
        }

        let tool_sequence: Vec<String> = tool_steps
            .iter()
            .map(|s| {
                if let StepKind::ToolCall { ref tool_name } = s.kind {
                    tool_name.clone()
                } else {
                    String::new()
                }
            })
            .filter(|n| !n.is_empty())
            .collect();

        let name = generate_skill_name(&trajectory.user_message, &tool_sequence);
        let description = generate_description(&trajectory.user_message, &tool_sequence);
        let instructions = Self::build_instructions(trajectory, &tool_steps);

        Some(Skill {
            name,
            description,
            instructions,
            version: "1.0.0".to_string(),
            author: Some("nexus-auto".to_string()),
        })
    }

    fn build_instructions(trajectory: &Trajectory, tool_steps: &[&TrajectoryStep]) -> String {
        let mut instructions = String::from("## Workflow\n\n");

        for (i, step) in tool_steps.iter().enumerate() {
            if let StepKind::ToolCall { ref tool_name } = step.kind {
                instructions.push_str(&format!("### Step {}: {}\n", i + 1, tool_name));

                if let Value::Object(ref map) = step.input {
                    for (key, val) in map {
                        if key != "api_key" && key != "token" && key != "secret" {
                            instructions.push_str(&format!("- `{}`: {}\n", key, val));
                        }
                    }
                }

                if let Some(ref err) = step.error {
                    instructions.push_str(&format!("  - ⚠️ Common error: {}\n", err));
                }
                instructions.push('\n');
            }
        }

        instructions.push_str("## Example\n\n");
        instructions.push_str(&format!("User request: \"{}\"\n", trajectory.user_message));
        instructions.push('\n');
        instructions.push_str("## Notes\n\n- This skill was auto-generated from a successful trajectory.\n");
        instructions
    }

    pub fn extract_multiple(trajectories: &[Trajectory]) -> Vec<Skill> {
        trajectories
            .iter()
            .filter_map(|t| Self::extract_from_trajectory(t))
            .collect()
    }

    pub fn extract_patterns(trajectories: &[Trajectory]) -> Vec<Skill> {
        let mut skills = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

        for traj in trajectories {
            if !traj.success {
                continue;
            }
            let tool_names: Vec<String> = traj
                .steps
                .iter()
                .filter_map(|s| {
                    if let StepKind::ToolCall { ref tool_name } = s.kind {
                        Some(tool_name.clone())
                    } else {
                        None
                    }
                })
                .collect();

            let key = tool_names.join("->");
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if let Some(skill) = Self::extract_from_trajectory(traj) {
                skills.push(skill);
            }
        }

        skills
    }
}

fn generate_skill_name(user_message: &str, tool_sequence: &[String]) -> String {
    let words: Vec<&str> = user_message
        .split_whitespace()
        .filter(|w| w.len() > 3)
        .take(3)
        .collect();

    if words.is_empty() {
        let tools_joined = tool_sequence.join("_");
        return format!("auto_{}", tools_joined);
    }

    let name = words.join("_");
    format!("auto_{}", name.to_lowercase())
}

fn generate_description(user_message: &str, tool_sequence: &[String]) -> String {
    let tools = tool_sequence.join(", ");
    format!(
        "Auto-generated skill for: \"{:.60}\". Uses tools: {}.",
        user_message, tools
    )
}
