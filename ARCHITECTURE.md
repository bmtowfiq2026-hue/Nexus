# Architecture

## Overview

Nexus has three layers:

```
┌─────────────────────────────────────────┐
│            USER LAYER                    │
│  CLI  │  Terminal  │  Web  │  Mobile    │
├─────────────────────────────────────────┤
│          CHANNEL LAYER (Go Gateway)      │
│  HTTP/WebSocket/Unix Socket bridge       │
├─────────────────────────────────────────┤
│          RUNTIME LAYER (Rust Core)       │
│  Agent Loop  │  Memory  │  Skills       │
│  Tools       │  Trajectory  │  Audit    │
└─────────────────────────────────────────┘
```

## Layer 1: Runtime (Rust Core)

The core library (`libnexus_core`) contains all agent intelligence:

### Agent Loop (`core/src/agent/`)
- `AgentLoop` — orchestrates provider calls, tool dispatch, trajectory recording, and checkpointing
- `Session` — manages conversation state, turn history, session metadata

### Memory (`core/src/memory/`)
- `MemoryStore` — flat key-value storage with read/write/delete
- `FullTextSearch` (WIP) — Tantivy-based keyword search
- `VectorStore` — in-memory embeddings with cosine similarity
- `GraphMemory` — typed entity nodes with weighted relationships
- `MemorySummarizer` — LLM-driven conversation compression

### Skills (`core/src/skills/`)
- `SkillEngine` — loads, parses, and executes SKILL.md files
- `SkillParser` — YAML frontmatter + markdown body parsing
- `SkillRefiner` — auto-improves skills from trajectory failures

### Trajectory (`core/src/trajectory/`)
- `TrajectoryRecorder` — captures every step of an agent turn
- `SkillExtractor` — mines successful trajectories for skill candidates

### Providers (`core/src/providers/`)
- **Primary:** `OpenAI` (GPT-4o), `Anthropic` (Claude), `Demo` (no-key mode)
- **Local:** `Ollama`, `LM Studio`, `LocalAI`, `oobabooga`
- **Compatible (OpenAI API):** `Google Gemini`, `DeepSeek`, `Groq`, `Together AI`, `Fireworks AI`, `OpenRouter`, `Perplexity`, `Mistral AI`, `Cohere`, `AI21 Labs`, `Replicate`, `HuggingFace`, `Cerebras`, `xAI (Grok)`, `DeepInfra`, `SambaNova`, `Anyscale`
- Set via `NEXUS_PROVIDER` env var or `--provider` flag

### Checkpoint (`core/src/checkpoint/`)
- `CheckpointManager` — snapshot/restore agent state
- DAG-based history with named snapshots
- `diff()` between any two states

### Identity (`core/src/identity/`)
- DID (Decentralized Identifier) generation with Ed25519 keypairs

### Tools (`core/src/tools/`)
- `ToolRegistry` — register and dispatch tool calls
- Built-in tools: `read`, `write`, `search`, `fetch`, `exec`

## Layer 2: Gateway (Go)

The gateway (`gateway/`) handles multi-protocol messaging:

### Channels (13 supported)
- **WebChat** — WebSocket-based HTML chat UI + HTTP REST API (built-in)
- **Discord** — bot with message events, slash commands
- **Telegram** — bot with message handler, inline keyboards
- **Slack** — RTM socket mode + Web API posting
- **Matrix** — Client-Server API via webhook wrapper
- **WhatsApp** — Webhook-based (Meta API)
- **Signal** — Webhook-based (Signal Messenger REST API)
- **IRC** — Native TCP implementation (NICK/USER/JOIN/PING-PONG)
- **Google Chat** — Incoming webhook
- **MSTeams** — Incoming webhook
- **LINE** — Webhook-based (LINE Messaging API)
- **Messenger** — Webhook-based (Facebook Messenger API)
- **Twilio** — Webhook-based (Twilio SMS/WhatsApp API)

### Message Bus
In-process publish/subscribe with named topics. Pending: NATS JetStream for distributed mode.

### Session Manager
Thread-safe `sync.Map`-based sessions with turn history ring buffer.

## Layer 3: Communication

The CLI and gateway communicate via either:
1. **Unix domain socket** (Linux/macOS) — fastest, local only
2. **TCP localhost** (Windows) — IP-based, configurable port
3. **Embedded mode** (planned) — CLI spawns gateway as a child process

## Data Flow

```
User Message
    │
    ▼
Channel (Discord/Telegram/Slack/WebChat)
    │  (JSON message envelope)
    ▼
Message Bus
    │  (routed to gateway → agent bridge)
    ▼
AgentLoop.run_turn()
    │
    ├── CheckpointManager.snapshot()
    ├── TrajectoryRecorder.record_turn_start()
    ├── Provider.chat() ──► LLM API
    ├── Tool dispatch loop:
    │       ├── Parse tool calls from response
    │       ├── Execute tools (read/write/search/fetch/exec)
    │       └── Feed results back to LLM
    ├── MemoryStore.save_conversation()
    ├── VectorStore.insert()
    ├── GraphMemory.extract_entities()
    ├── TrajectoryRecorder.record_turn_end()
    └── SkillExtractor (if trajectory completed successfully)
```

## Directory Layout

```
nexus/
├── Cargo.toml                 # Workspace root
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── agent/     (loop, session)
│       ├── memory/    (store, fts, vector, graph, summarizer)
│       ├── skills/    (engine, parser, refiner)
│       ├── tools/     (registry, builtin)
│       ├── providers/ (openai, anthropic, ollama, demo, openai_compat)
│       ├── trajectory/(recorder, extractor)
│       ├── checkpoint/(snapshot, rollback, diff)
│       ├── identity/  (did, keys)
│       └── audit/     (logging)
├── cli/
│   ├── Cargo.toml
│   └── src/main.rs
├── gateway/
│   ├── main.go
│   ├── gateway.json
│   └── internal/
│       ├── channel/    (13 channels: webchat, discord, telegram, slack, matrix, whatsapp, signal, irc, googlechat, msteams, line, messenger, twilio)
│       ├── bus/        (pub/sub)
│       └── session/    (turn history)
├── docs/
├── scripts/
├── README.md
├── LICENSE
└── INSTALL.md
```

## Extending

### Adding a Channel

1. Create `gateway/internal/channel/<name>/<name>.go`
2. Implement the `Channel` interface (see `channel.go`)
3. Register in `main.go`

### Adding a Provider

1. Create `core/src/providers/<name>.rs`
2. Implement the `Provider` trait
3. Add variant to `ProviderKind` enum in `mod.rs`
4. Register in `agent/loop_.rs`

### Adding a Built-in Tool

1. Add function in `core/src/tools/builtin.rs`
2. Register in `ToolRegistry::new()` in `mod.rs`
