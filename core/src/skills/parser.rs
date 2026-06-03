use super::Skill;
use crate::Result;

pub fn parse_skill_md(content: &str) -> Result<Skill> {
    let mut name = String::from("unnamed");
    let mut description = String::new();
    let mut instructions = String::new();
    let mut in_instructions = false;

    for line in content.lines() {
        if line.starts_with("# ") {
            name = line[2..].trim().to_string();
        } else if line.starts_with("> ") {
            description = line[2..].trim().to_string();
        } else if line.starts_with("## ") {
            in_instructions = true;
        } else if in_instructions {
            instructions.push_str(line);
            instructions.push('\n');
        }
    }

    Ok(Skill {
        name,
        description,
        instructions: instructions.trim().to_string(),
        version: "1.0.0".to_string(),
        author: None,
    })
}

pub fn generate_skill_md(skill: &Skill) -> String {
    format!(
        "# {}\n\n> {}\n\n## Instructions\n\n{}\n\n## Metadata\n\n- Version: {}\n- Author: {}\n",
        skill.name,
        skill.description,
        skill.instructions,
        skill.version,
        skill.author.as_deref().unwrap_or("unknown"),
    )
}
