use super::{parser, Skill};
use crate::agent::session::Session;
use crate::Result;
use std::collections::HashMap;

pub struct SkillEngine {
    skills: HashMap<String, Skill>,
    active_skills: Vec<String>,
}

impl SkillEngine {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            active_skills: Vec::new(),
        }
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<()> {
        let dir = std::path::Path::new(path);
        if !dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let content = std::fs::read_to_string(&path)?;
                if let Ok(skill) = parser::parse_skill_md(&content) {
                    self.register(skill);
                }
            }
        }
        Ok(())
    }

    pub fn register(&mut self, skill: Skill) {
        self.skills.insert(skill.name.clone(), skill);
    }

    pub fn activate(&mut self, name: &str) -> Result<()> {
        if self.skills.contains_key(name) {
            self.active_skills.push(name.to_string());
            Ok(())
        } else {
            Err(crate::NexusError::Skill(format!("Skill '{}' not found", name)))
        }
    }

    pub fn deactivate(&mut self, name: &str) {
        self.active_skills.retain(|s| s != name);
    }

    pub async fn get_active_context(&self, _session: &Session) -> Result<String> {
        let mut context = String::new();
        for name in &self.active_skills {
            if let Some(skill) = self.skills.get(name) {
                context.push_str(&format!("# Skill: {}\n{}\n\n", skill.name, skill.instructions));
            }
        }
        Ok(context)
    }

    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
}
