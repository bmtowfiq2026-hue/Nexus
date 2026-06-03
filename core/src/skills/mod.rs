pub mod engine;
pub mod parser;
pub mod refiner;

pub use engine::SkillEngine;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub version: String,
    pub author: Option<String>,
}

impl Skill {
    pub fn new(name: &str, description: &str, instructions: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            instructions: instructions.to_string(),
            version: "1.0.0".to_string(),
            author: None,
        }
    }
}
