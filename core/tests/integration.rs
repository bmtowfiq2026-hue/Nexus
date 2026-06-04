use nexus_core::agent::session::{Turn, TurnRole};
use nexus_core::memory::graph::{EntityType, GraphMemory};
use nexus_core::memory::store::MemoryStore;
use nexus_core::memory::vector::VectorStore;
use nexus_core::memory::MemoryEntry;
use nexus_core::memory::MemoryType;
use nexus_core::providers::demo::DemoProvider;
use nexus_core::providers::{Provider, ProviderConfig, ProviderRequest, TokenUsage};
use nexus_core::tools::ToolDispatcher;
use nexus_core::NexusConfig;
use std::collections::HashMap;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

fn make_turn(role: TurnRole, content: &str) -> Turn {
    Turn {
        id: uuid::Uuid::new_v4().to_string(),
        role,
        content: content.to_string(),
        tool_calls: vec![],
        timestamp: chrono::Utc::now(),
    }
}

#[test]
fn test_demo_provider_greeting() {
    let provider = DemoProvider::new();
    let request = ProviderRequest {
        system_prompt: String::new(),
        messages: vec![make_turn(TurnRole::User, "hello")],
        user_message: "hello".to_string(),
        tools: vec![],
    };
    let response = rt().block_on(provider.complete(request)).unwrap();
    assert!(response.content.contains("Hello"), "Response should contain greeting");
    assert_eq!(response.tool_calls.len(), 0);
}

#[test]
fn test_demo_provider_name() {
    let provider = DemoProvider::new();
    assert_eq!(provider.name(), "demo");
}

#[test]
fn test_demo_provider_capabilities_query() {
    let provider = DemoProvider::new();
    let request = ProviderRequest {
        system_prompt: String::new(),
        messages: vec![make_turn(TurnRole::User, "what can you do")],
        user_message: "what can you do".to_string(),
        tools: vec![],
    };
    let response = rt().block_on(provider.complete(request)).unwrap();
    assert!(response.content.contains("Memory"), "Should mention Memory");
    assert!(response.content.contains("Tools"), "Should mention tools");
}

#[test]
fn test_config_default_values() {
    let config = NexusConfig::default();
    assert_eq!(config.default_provider, "demo");
    assert_eq!(config.default_model, "demo");
    assert_eq!(config.workspace, "~/.nexus");
    assert!(config.api_keys.contains_key("openai"));
    assert!(config.api_keys.contains_key("anthropic"));
}

#[test]
fn test_config_serialize_roundtrip() {
    let config = NexusConfig::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    let deserialized: NexusConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.default_provider, config.default_provider);
    assert_eq!(deserialized.default_model, config.default_model);
}

#[test]
fn test_config_api_key_fallback() {
    let config = NexusConfig::default();
    let key = config.get_api_key("nonexistent");
    assert!(key.is_none(), "Missing provider should return None");
}

#[test]
fn test_provider_config_defaults() {
    let config = ProviderConfig::default();
    assert_eq!(config.model, "gpt-4o");
    assert_eq!(config.max_tokens, 4096);
    assert_eq!(config.temperature, 0.7);
    assert!(config.api_key.is_none());
}

#[test]
fn test_memory_store_store_and_count() {
    let mut store = MemoryStore::new(":memory:");
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);

    let entry = MemoryEntry::new(
        "sess-1".to_string(),
        "Hello Nexus".to_string(),
        MemoryType::Conversation,
    );
    store.store(entry).unwrap();

    assert!(!store.is_empty());
    assert_eq!(store.len(), 1);
}

#[test]
fn test_memory_store_search_relevant() {
    let mut store = MemoryStore::new(":memory:");
    let entry = MemoryEntry::new(
        "sess-1".to_string(),
        "The capital of France is Paris".to_string(),
        MemoryType::Fact,
    );
    store.store(entry).unwrap();

    let results = store.search_relevant("sess-1", "France", 10).unwrap();
    assert!(!results.is_empty(), "Should find results for France");
    assert!(results[0].contains("Paris"));
}

#[test]
fn test_memory_store_search_empty() {
    let store = MemoryStore::new(":memory:");
    let results = store.search_relevant("sess-1", "nonexistent", 10).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_vector_store_store_and_count() {
    let store = VectorStore::new(":memory:", 128);
    assert_eq!(store.count(), 0);

    let emb = nexus_core::memory::vector::compute_embedding("hello world");
    let mut meta = HashMap::new();
    meta.insert("source".to_string(), "test".to_string());
    store.store_embedding("id-1", emb, "hello world", meta).unwrap();
    assert_eq!(store.count(), 1);
}

#[test]
fn test_vector_store_search_similar() {
    let store = VectorStore::new(":memory:", 128);

    let emb = nexus_core::memory::vector::compute_embedding("test data for search");
    let emb_diff = nexus_core::memory::vector::compute_embedding("completely different content here");

    store.store_embedding("id-1", emb, "test data for search", HashMap::new()).unwrap();
    store.store_embedding("id-2", emb_diff, "completely different", HashMap::new()).unwrap();

    // Search with exact same text as id-1
    let query = nexus_core::memory::vector::compute_embedding("test data for search");
    let results = store.search_similar(&query, 3).unwrap();
    assert!(!results.is_empty(), "Should return at least one result");
    assert_eq!(results[0].0, "id-1", "Exact match should be most similar");
}

#[test]
fn test_vector_store_clear() {
    let store = VectorStore::new(":memory:", 128);
    let emb = nexus_core::memory::vector::compute_embedding("test");
    store.store_embedding("id-1", emb, "test", HashMap::new()).unwrap();
    assert_eq!(store.count(), 1);
    store.clear();
    assert_eq!(store.count(), 0);
}

#[test]
fn test_vector_store_get_content() {
    let store = VectorStore::new(":memory:", 128);
    let emb = nexus_core::memory::vector::compute_embedding("hello");
    store.store_embedding("id-1", emb, "hello content", HashMap::new()).unwrap();
    assert_eq!(store.get_content("id-1"), Some("hello content".to_string()));
    assert_eq!(store.get_content("nonexistent"), None);
}

#[test]
fn test_graph_memory_add_entity() {
    let mut graph = GraphMemory::new();
    assert_eq!(graph.entity_count(), 0);

    let id = graph.add_entity(
        "Nexus",
        EntityType::Project,
        HashMap::new(),
    );
    assert_eq!(graph.entity_count(), 1);

    let entity = graph.find_entity("Nexus");
    assert!(entity.is_some());
    assert_eq!(entity.unwrap().name, "Nexus");

    let by_id = graph.get_entity(&id);
    assert!(by_id.is_some());
}



#[test]
fn test_graph_memory_get_related_entities() {
    let mut graph = GraphMemory::new();
    let nexus = graph.add_entity("Nexus", EntityType::Project, HashMap::new());
    let rust = graph.add_entity("Rust", EntityType::Technology, HashMap::new());

    graph.add_relation(&nexus, &rust, "built_with", "core runtime");
    let related = graph.get_related_entities(&nexus);
    assert!(!related.is_empty());
    assert_eq!(related[0].0.name, "Rust");
}

#[test]
fn test_graph_memory_extract_entities() {
    let mut graph = GraphMemory::new();
    graph.extract_entities_from_text(
        "Alice Smith and Bob Jones work at Microsoft Corp on Project Nexus.",
        "sess-1",
    );
    assert!(graph.entity_count() > 0, "Should extract some entities");
}

#[test]
fn test_tool_dispatcher_register_builtins() {
    let mut dispatcher = ToolDispatcher::new();
    dispatcher.register_builtins();
    let defs = dispatcher.get_definitions();
    let names: Vec<&str> = defs.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"read"), "Should have read tool, got: {:?}", names);
    assert!(names.contains(&"write"), "Should have write tool");
    assert!(names.contains(&"web_search"), "Should have web_search tool");
    assert!(names.contains(&"web_fetch"), "Should have web_fetch tool");
    assert!(names.contains(&"exec"), "Should have exec tool");
}

#[test]
fn test_tool_dispatcher_execute_unknown() {
    let dispatcher = ToolDispatcher::new();
    let result = dispatcher.execute("nonexistent", serde_json::json!({}));
    assert!(result.is_err(), "Unknown tool should return error");
}

#[test]
fn test_provider_list_integrity() {
    use nexus_core::providers::OPENAI_COMPAT_PROVIDERS;
    assert!(!OPENAI_COMPAT_PROVIDERS.is_empty());
    for (name, _display, base_url, _model) in OPENAI_COMPAT_PROVIDERS {
        assert!(!name.is_empty(), "Provider name should not be empty");
        assert!(!base_url.is_empty(), "Base URL should not be empty");
    }
    let names: Vec<&str> = OPENAI_COMPAT_PROVIDERS.iter().map(|(n, _, _, _)| *n).collect();
    let mut unique = names.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), names.len(), "Provider names must be unique");
}

#[test]
fn test_tool_dispatcher_empty() {
    let dispatcher = ToolDispatcher::new();
    let defs = dispatcher.get_definitions();
    assert!(defs.is_empty(), "New dispatcher should have no tools");
}

#[test]
fn test_token_usage_default() {
    let usage = TokenUsage { input_tokens: 0, output_tokens: 0, total_tokens: 0 };
    assert_eq!(usage.total_tokens, 0);
}

#[test]
fn test_turn_creation() {
    let turn = make_turn(TurnRole::User, "hello");
    assert!(matches!(turn.role, TurnRole::User));
    assert_eq!(turn.content, "hello");
    assert!(turn.tool_calls.is_empty());

    let assistant = make_turn(TurnRole::Assistant, "hi there");
    assert!(matches!(assistant.role, TurnRole::Assistant));
}
