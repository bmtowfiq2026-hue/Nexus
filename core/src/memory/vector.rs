use crate::Result;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct VectorRecord {
    pub id: String,
    pub embedding: Vec<f32>,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

pub struct VectorStore {
    records: Mutex<Vec<VectorRecord>>,
    dimensions: usize,
}

impl VectorStore {
    pub fn new(_path: &str, dimensions: usize) -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            dimensions,
        }
    }

    pub fn store_embedding(&self, id: &str, embedding: Vec<f32>, content: &str, metadata: HashMap<String, String>) -> Result<()> {
        let mut records = self.records.lock().unwrap();
        records.push(VectorRecord {
            id: id.to_string(),
            embedding,
            content: content.to_string(),
            metadata,
        });
        Ok(())
    }

    pub fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<(String, f32)>> {
        let records = self.records.lock().unwrap();

        if records.is_empty() || query_embedding.is_empty() {
            return Ok(Vec::new());
        }

        let mut scored: Vec<(usize, f32)> = records
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let sim = cosine_similarity(query_embedding, &r.embedding);
                (i, sim)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let results: Vec<(String, f32)> = scored
            .iter()
            .take(limit)
            .map(|(i, score)| (records[*i].id.clone(), *score))
            .collect();

        Ok(results)
    }

    pub fn get_content(&self, id: &str) -> Option<String> {
        let records = self.records.lock().unwrap();
        records.iter().find(|r| r.id == id).map(|r| r.content.clone())
    }

    pub fn count(&self) -> usize {
        self.records.lock().unwrap().len()
    }

    pub fn clear(&self) {
        self.records.lock().unwrap().clear();
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

pub fn compute_embedding(text: &str) -> Vec<f32> {
    let mut vec = Vec::with_capacity(128);
    let bytes = text.as_bytes();

    for i in 0..128 {
        let mut val = 0.0f32;
        let start = (i * bytes.len()) / 128;
        let end = if bytes.len() == 0 {
            0
        } else {
            (((i + 1) * bytes.len()) / 128).min(bytes.len())
        };
        let count = if end > start { end - start } else { 1 };

        for j in start..end {
            val += bytes[j] as f32;
        }

        vec.push(val / count as f32);
    }

    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut vec {
            *v /= norm;
        }
    }

    vec
}
