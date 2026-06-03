<h1 align="center">Nexus</h1>

<p align="center">
  <b>The autonomous AI agent platform that learns, remembers, and grows.</b>
</p>

<p align="center">
  <a href="https://github.com/nexus/nexus/actions"><img src="https://img.shields.io/github/actions/workflow/status/nexus/nexus/ci.yml?branch=main&style=flat-square" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.96+-orange.svg?style=flat-square" alt="Rust"></a>
  <a href="https://go.dev"><img src="https://img.shields.io/badge/go-1.22+-00ADD8.svg?style=flat-square" alt="Go"></a>
  <a href="https://github.com/nexus/nexus/releases"><img src="https://img.shields.io/github/v/release/nexus/nexus?style=flat-square" alt="Release"></a>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#cli-commands">CLI</a> •
  <a href="#contributing">Contributing</a>
</p>

<hr>

## Quick Start

**No API key needed.** Try Nexus immediately in demo mode:

```bash
# Build from source
git clone https://github.com/nexus/nexus.git
cd nexus
cargo build --release

# Initialize your workspace
./target/release/nexus init

# Start chatting right now (zero configuration)
./target/release/nexus chat
```

```
✦ Nexus Agent ready (demo mode). Type '/quit' to exit.

  ℹ Run with a real provider:
    • nexus chat --provider openai  (set OPENAI_API_KEY)
    • nexus chat --provider ollama   (run Ollama locally)

You: hello
Nexus: Hello! I'm Nexus, your autonomous AI agent. I'm currently running in demo mode.
...
```

That's it. No accounts, no API keys, no setup.

> **Demo mode** simulates realistic responses so you can explore the full interface — memory, skills, tools, trajectory recording. When you're ready, connect a real LLM provider for actual AI-powered conversations.

### Connect a real provider

```bash
# OpenAI (recommended)
export OPENAI_API_KEY="sk-..."
nexus chat --provider openai

# Anthropic Claude
export ANTHROPIC_API_KEY="sk-ant-..."
nexus chat --provider anthropic

# Local Ollama (free, no API key)
ollama pull llama3     # Install Ollama first: https://ollama.ai
nexus chat --provider ollama
```

## What is Nexus?

Nexus is an **end-to-end autonomous agent platform** built from scratch in Rust and Go. Unlike existing agent frameworks that are either single-channel or lack self-improvement, Nexus combines:

- **OpenClaw's channel breadth** — Discord, Telegram, Slack, WebSocket, and 20+ more through the Go gateway
- **Hermes Agent's learning loop** — trajectory recording, skill extraction, checkpoint/rollback
- **Unique innovations** — graph memory, DID identity, immutable audit trail, privacy-as-config

### Why not just use OpenClaw or Hermes Agent?

| Feature | OpenClaw | Hermes Agent | **Nexus** |
|---------|----------|-------------|-----------|
| Channels | 20+ | 14 | **25+** |
| Demo mode | ✗ | ✗ | **✓** |
| Learning Loop | ✗ | ✓ | **✓** |
| Skill Auto-Creation | ✗ | ✓ | **✓ + Refinement** |
| Graph Memory | ✗ | ✗ | **✓** |
| Checkpoint/Rollback | ✗ | ✓ | **✓** |
| Multi-Agent | ✓ | Single | **✓** |
| Cryptographic Identity | ✗ | ✗ | **✓ (DID)** |
| Privacy-as-Config | ✗ | ✗ | **✓ (4 modes)** |
| Written in | TS | TS | **Rust + Go** |

## Features

### 🧠 Self-Improving Learning Loop
Every interaction is recorded as a **trajectory** — a detailed, step-by-step log of everything the agent did, thought, and tried. When a task succeeds, Nexus analyzes the trajectory and automatically generates a reusable **skill** (SKILL.md). When a skill fails, it's **refined** with recovery instructions.

### 💾 Three-Layer Memory
| Layer | Storage | Purpose |
|-------|---------|---------|
| **Full-Text Search** | SQLite + Tantivy | Keyword retrieval across all conversations |
| **Vector Store** | In-memory embeddings | Semantic similarity with cosine distance |
| **Graph Memory** | Entity + Relation graph | Knowledge tracking across sessions |

Memory is automatically summarized when conversations grow long, preserving key facts without overflowing context windows.

### 📸 Checkpoint & Rollback
Every agent turn is snapshotted. Roll back to any point in history, diff between states, or recover from failures — essential for debugging, auditing, and iterative agent development.

### 🔧 Tool System
Built-in tools that the agent can invoke autonomously:

- **read** — Read files from the filesystem
- **write** — Write content to files
- **search** — Web search via DuckDuckGo
- **fetch** — Fetch and parse web pages
- **exec** — Execute commands in a sandboxed environment

Tools are dispatched, tracked, and recorded in trajectories for learning.

### 🌐 Multi-Channel Gateway
The Go gateway connects Nexus to 25+ messaging platforms:

```bash
cd gateway
go build -o nexus-gateway .
./nexus-gateway
```

```json
{
  "port": 8080,
  "discord":  { "enabled": false, "bot_token": "" },
  "telegram": { "enabled": false, "bot_token": "" },
  "slack":    { "enabled": false, "bot_token": "" },
  "webchat":  { "enabled": true,  "path": "/ws" }
}
```

### 🔐 Security & Privacy
- **Sandboxed execution** — commands run with configurable resource limits (CPU, memory, network)
- **Privacy modes** — 4 levels from full-sharing to fully air-gapped
- **Local-first** — all memory and data stored locally by default
- **Audit trail** (coming) — SHA-256 hash-chained action log with cryptographic signing

## CLI Commands

| Command | Description |
|---------|-------------|
| `nexus init` | Initialize a Nexus workspace |
| `nexus chat` | Interactive chat session (starts in demo mode by default) |
| `nexus run --prompt <text>` | Run a single task and exit |
| `nexus config show` | View current configuration |
| `nexus config set <key> <value>` | Set a configuration value |
| `nexus skill list` | List installed skills |
| `nexus skill install <path>` | Install a skill from file |
| `nexus skill activate <name>` | Activate a skill |
| `nexus skill deactivate <name>` | Deactivate a skill |

```
nexus --help
nexus --version    # → nexus 0.5.0
```

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                     USER LAYER                            │
│    CLI     │   Terminal   │   Dashboard   │    API       │
├──────────────────────────────────────────────────────────┤
│                  CHANNEL LAYER (Go Gateway)               │
│  Discord │ Telegram │ Slack │ WebChat │ Signal │ +20    │
│                Message Bus (pub/sub)                     │
├──────────────────────────────────────────────────────────┤
│                  RUNTIME LAYER (Rust Core)                │
│  ┌──────────┬──────────┬──────────┬────────────────┐    │
│  │  Agent   │  Memory  │  Skills  │  Tools         │    │
│  │  Loop    │  FTS     │  Engine  │  read/write    │    │
│  │          │  Vector  │  Extract │  search/fetch  │    │
│  │          │  Graph   │  Refine  │  exec          │    │
│  └──────────┴──────────┴──────────┴────────────────┘    │
│  ┌──────────┬──────────┬──────────┬────────────────┐    │
│  │Trajectory│Checkpoint│ Identity │  Audit         │    │
│  │Recording │Rollback  │ (DID)    │  (hash-chain)  │    │
│  └──────────┴──────────┴──────────┴────────────────┘    │
└──────────────────────────────────────────────────────────┘
```

## Project Structure

```
nexus/
├── Cargo.toml              # Rust workspace root
├── core/                   # Agent runtime library
│   ├── src/
│   │   ├── agent/          # Agent loop, sessions
│   │   ├── memory/         # FTS, vector, graph, summarizer
│   │   ├── skills/         # Engine, parser, refiner
│   │   ├── tools/          # Tool registry + builtins
│   │   ├── providers/      # OpenAI, Anthropic, Ollama, Demo
│   │   ├── trajectory/     # Recording + skill extraction
│   │   ├── checkpoint/     # State snapshots + rollback
│   │   ├── identity/       # DID-based identity
│   │   └── audit/          # Hash-chain audit logging
│   └── Cargo.toml
├── cli/                    # CLI binary
│   ├── src/main.rs
│   └── Cargo.toml
├── gateway/                # Go messaging gateway
│   ├── main.go
│   ├── gateway.json
│   └── internal/           # Channels, bus, sessions
├── scripts/                # Build & setup scripts
├── docs/                   # Documentation
└── .github/                # CI, issue templates, funding
```

## Development

```bash
# Build everything
cargo build --release

# Run tests
cargo test

# Run linting
cargo clippy

# Build the Go gateway
cd gateway && go build -o nexus-gateway .
```

## Why Rust + Go?

| Layer | Language | Rationale |
|-------|----------|-----------|
| **Agent Runtime** | Rust | Performance, memory safety, zero-cost abstractions, single static binary |
| **Gateway** | Go | Goroutine-per-channel concurrency, fast compilation, excellent HTTP/WS libraries |
| **CLI** | Rust | Fast startup, no runtime dependency, cross-compilation to all targets |

## Roadmap

- [x] **Phase 1:** Core agent loop, CLI, 3 LLM providers, tool system
- [x] **Phase 2:** Multi-channel gateway (Discord, Telegram, Slack, WebChat)
- [x] **Phase 3:** Learning loop, trajectory recording, skill extraction, graph/vector memory, checkpointing
- [ ] **Phase 4:** Agent DNA (DID identity), immutable audit trail
- [ ] **Phase 5:** Visual Agent Studio (drag-and-drop workflow builder)
- [ ] **Phase 6:** Agent roaming (P2P network, federated learning)
- [ ] **Phase 7:** Mobile apps (iOS + Android)

## License

MIT — see [LICENSE](LICENSE)

## Contributing

We welcome contributions. See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## Security

See [SECURITY.md](SECURITY.md) for reporting vulnerabilities.

---

<p align="center">
  <a href="https://github.com/nexus/nexus">GitHub</a> •
  <a href="#">Discord</a> •
  <a href="#">Documentation</a> •
  <a href="https://github.com/nexus/nexus/issues">Issues</a>
</p>
