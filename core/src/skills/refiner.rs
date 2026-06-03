use crate::skills::Skill;
use crate::Result;

pub struct SkillRefiner;

impl SkillRefiner {
    pub fn new() -> Self {
        Self
    }

    pub fn refine_from_failure(
        skill: &Skill,
        user_message: &str,
        error_message: &str,
    ) -> Result<Skill> {
        let mut instructions = skill.instructions.clone();

        let error_note = format!(
            "\n\n## Failure Recovery\n\nWhen handling \"{:.100}\", this skill failed with:\n```\n{}\n```\n\n### Recovery Steps\n1. Verify inputs are valid before proceeding\n2. If tool returns an error, retry once with corrected parameters\n3. If persistent error, report the issue and suggest alternatives\n",
            user_message, error_message
        );

        instructions.push_str(&error_note);

        let version_parts: Vec<&str> = skill.version.split('.').collect();
        let patch: u32 = version_parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0) + 1;
        let new_version = format!("1.0.{}", patch);

        Ok(Skill {
            name: skill.name.clone(),
            description: skill.description.clone(),
            instructions,
            version: new_version,
            author: skill.author.clone(),
        })
    }

    pub fn merge_skills(existing: &Skill, new: &Skill) -> Skill {
        let mut merged_instructions = existing.instructions.clone();

        merged_instructions.push_str("\n\n---\n\n## Merged Improvements\n\n");
        merged_instructions.push_str(&new.instructions);

        let merged_name = existing.name.clone();
        let merged_desc = if existing.description.len() < new.description.len() {
            new.description.clone()
        } else {
            existing.description.clone()
        };

        Skill {
            name: merged_name,
            description: merged_desc,
            instructions: merged_instructions,
            version: existing.version.clone(),
            author: existing.author.clone(),
        }
    }
}
