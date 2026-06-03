
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type EntityId = String;
pub type RelationId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub entity_type: EntityType,
    pub attributes: HashMap<String, String>,
    pub mention_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Concept,
    Technology,
    Project,
    Topic,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: RelationId,
    pub source: EntityId,
    pub target: EntityId,
    pub relation_type: String,
    pub weight: f64,
    pub context: String,
}

pub struct GraphMemory {
    entities: HashMap<EntityId, Entity>,
    relations: Vec<Relation>,
    name_index: HashMap<String, EntityId>,
}

impl GraphMemory {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            relations: Vec::new(),
            name_index: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, name: &str, entity_type: EntityType, attributes: HashMap<String, String>) -> EntityId {
        if let Some(existing_id) = self.name_index.get(name) {
            if let Some(entity) = self.entities.get_mut(existing_id) {
                entity.mention_count += 1;
                for (k, v) in attributes {
                    entity.attributes.entry(k).or_insert(v);
                }
            }
            return existing_id.clone();
        }

        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let entity = Entity {
            id: id.clone(),
            name: name.to_string(),
            entity_type,
            attributes,
            mention_count: 1,
        };

        self.name_index.insert(name.to_string(), id.clone());
        self.entities.insert(id.clone(), entity);
        id
    }

    pub fn add_relation(&mut self, source_id: &str, target_id: &str, relation_type: &str, context: &str) {
        use uuid::Uuid;
        let relation = Relation {
            id: Uuid::new_v4().to_string(),
            source: source_id.to_string(),
            target: target_id.to_string(),
            relation_type: relation_type.to_string(),
            weight: 1.0,
            context: context.to_string(),
        };
        self.relations.push(relation);
    }

    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn find_entity(&self, name: &str) -> Option<&Entity> {
        self.name_index.get(name).and_then(|id| self.entities.get(id))
    }

    pub fn find_relations(&self, entity_id: &str) -> Vec<&Relation> {
        self.relations
            .iter()
            .filter(|r| r.source == entity_id || r.target == entity_id)
            .collect()
    }

    pub fn get_related_entities(&self, entity_id: &str) -> Vec<(&Entity, &Relation)> {
        let mut related = Vec::new();
        for rel in &self.relations {
            if rel.source == entity_id {
                if let Some(entity) = self.entities.get(&rel.target) {
                    related.push((entity, rel));
                }
            } else if rel.target == entity_id {
                if let Some(entity) = self.entities.get(&rel.source) {
                    related.push((entity, rel));
                }
            }
        }
        related
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    pub fn export_json(&self) -> serde_json::Value {
        let entities: Vec<&Entity> = self.entities.values().collect();
        let relations: Vec<&Relation> = self.relations.iter().collect();
        serde_json::json!({
            "entities": entities,
            "relations": relations,
        })
    }

    pub fn extract_entities_from_text(&mut self, text: &str, session_id: &str) {
        let candidates = extract_candidates(text);
        for (name, etype) in candidates {
            let mut attrs = HashMap::new();
            attrs.insert("source".to_string(), "conversation".to_string());
            attrs.insert("session".to_string(), session_id.to_string());
            self.add_entity(&name, etype, attrs);
        }
    }
}

fn extract_candidates(text: &str) -> Vec<(String, EntityType)> {
    let mut candidates = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for window in words.windows(2) {
        let first = window[0].trim_matches(|c: char| !c.is_alphanumeric());
        let second = window[1].trim_matches(|c: char| !c.is_alphanumeric());

        if first.chars().next().map_or(false, |c| c.is_uppercase())
            && second.chars().next().map_or(false, |c| c.is_uppercase())
        {
            let name = format!("{} {}", first, second);
            if name.len() > 2 && !candidates.iter().any(|(n, _)| n == &name) {
                candidates.push((name, EntityType::Person));
            }
        }
    }

    for word in words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.len() > 6 && clean.chars().next().map_or(false, |c| c.is_uppercase()) {
            let lower = clean.to_lowercase();
            if lower.contains("corp") || lower.contains("inc") || lower.contains("ltd") || lower.contains("llc") {
                if !candidates.iter().any(|(n, _)| n == clean) {
                    candidates.push((clean.to_string(), EntityType::Organization));
                }
            }
        }
    }

    candidates
}
