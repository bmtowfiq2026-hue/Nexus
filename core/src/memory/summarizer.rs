use crate::memory::{MemoryEntry, MemoryType};
use crate::Result;

pub struct MemorySummarizer;

impl MemorySummarizer {
    pub fn new() -> Self {
        Self
    }

    pub fn summarize_conversation(entries: &[MemoryEntry], max_length: usize) -> Result<MemoryEntry> {
        let mut summary = String::new();
        let mut key_points: Vec<String> = Vec::new();
        let mut topics: std::collections::HashSet<String> = std::collections::HashSet::new();

        for entry in entries {
            let content = &entry.content;

            if content.len() > 200 {
                let condensed = extract_key_phrases(content, 3);
                key_points.push(condensed);
            } else {
                key_points.push(content.clone());
            }

            if matches!(entry.entry_type, MemoryType::Fact | MemoryType::Preference) {
                topics.insert(extract_topic(content));
            }

            if summary.len() > max_length * 2 {
                break;
            }
        }

        let topic_list: Vec<&String> = topics.iter().take(5).collect();
        summary.push_str(&format!(
            "📋 Conversation Summary ({} entries, {} topics)\n",
            entries.len(),
            topic_list.len()
        ));

        if !topic_list.is_empty() {
            summary.push_str("Topics: ");
            let topics_str: Vec<String> = topic_list.iter().map(|s| s.to_string()).collect();
            summary.push_str(&topics_str.join(", "));
            summary.push('\n');
        }

        summary.push_str("\nKey points:\n");
        for (i, point) in key_points.iter().take(10).enumerate() {
            summary.push_str(&format!("{}. {}\n", i + 1, point));
        }

        if summary.len() > max_length {
            summary.truncate(max_length);
            summary.push_str("...");
        }

        let session_id = entries.first().map(|e| e.session_id.clone()).unwrap_or_default();
        Ok(MemoryEntry::new(session_id, summary, MemoryType::Observation))
    }

    pub fn should_summarize(entries: &[MemoryEntry], threshold: usize) -> bool {
        entries.len() > threshold
    }

    pub fn compress_facts(facts: &[MemoryEntry]) -> Result<MemoryEntry> {
        let mut compressed = String::new();
        compressed.push_str(&format!("📌 Stored Facts ({} total)\n", facts.len()));

        for fact in facts.iter().take(20) {
            compressed.push_str(&format!("- {}\n", fact.content));
        }

        if facts.len() > 20 {
            compressed.push_str(&format!("  ... and {} more facts\n", facts.len() - 20));
        }

        let session_id = facts.first().map(|e| e.session_id.clone()).unwrap_or_default();
        Ok(MemoryEntry::new(session_id, compressed, MemoryType::Observation))
    }
}

fn extract_key_phrases(text: &str, max_phrases: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let chunk_size = words.len() / max_phrases.max(1);

    if chunk_size == 0 {
        return text.to_string();
    }

    let mut phrases = Vec::new();
    for i in 0..max_phrases {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(words.len());
        if start < words.len() {
            phrases.push(words[start..end].join(" "));
        }
    }

    phrases.join(" ... ")
}

fn extract_topic(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return "general".to_string();
    }

    words
        .iter()
        .filter(|w| w.len() > 4)
        .take(3)
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
