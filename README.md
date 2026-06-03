# Nexus 🦞

**The autonomous AI agent platform that learns, remembers, and grows — combining OpenClaw's channel breadth with Hermes Agent's self-improving intelligence.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.96+-orange.svg)](https://www.rust-lang.org)
[![Go](https://img.shields.io/badge/Go-1.22+-00ADD8.svg)](https://go.dev)
[![Discord](https://img.shields.io/badge/Chat-Discord-5865F2.svg)](#)

---

## Why Nexus?

| Feature | OpenClaw | Hermes Agent | **Nexus** |
|---------|----------|-------------|-----------|
| Channels | 20+ | 14 | **25+** |
| Learning Loop | ✗ | ✓ | **✓** |
| Persistent Memory | Limited | ✓ | **✓ + Graph Memory** |
| Skill Auto-Creation | ✗ | ✓ | **✓ + Refinement** |
| Multi-Agent | ✓ | Single | **✓** |
| Checkpoint/Rollback | ✗ | ✓ | **✓** |
| Cryptographic Identity | ✗ | ✗ | **✓ (DID)** |
| Immutable Audit Trail | ✗ | ✗ | **✓ (Hash-Chained)** |
| Privacy-as-Config | ✗ | ✗ | **✓ (4 modes)** |
| Visual Agent Studio | ✗ | ✗ | **Coming** |
| Cost Optimization | ✗ | ✗ | **Coming** |

## Quick Start

### One-Command Install

```bash
# Linux / macOS / WSL2
curl -fsSL https://nexus.sh/install | bash

# Windows (PowerShell)
irm https://nexus.sh/install.ps1 | iex
```

### Or Build from Source

**Prerequisites:** Rust 1.96+ and Go 1.22+

```bash
# Clone and build
git clone https://github.com/nexus/nexus.git
cd nexus
cargo build --release
```

**Initialize your workspace:**

```bash
nexus init
#   ✓ Nexus workspace initialized at ~/.nexus
#   Config: ~/.nexus/nexus.json
#   Skills: ~/.nexus/skills
#   Memory: ~/.nexus/memory
```

**Start chatting:**

```bash
# Interactive chat (requires OPENAI_API_KEY env var)
export OPENAI_API_KEY="sk-..."
nexus chat
```

## Architecture

```
┌────────────────────────────────────────────────────────────┐
│                     USER LAYER                              │
│  CLI (nexus)  │  Web Dashboard  │  Mobile Apps  │  API     │
├────────────────────────────────────────────────────────────┤
│                    CHANNEL LAYER (Go Gateway)                │
│  Discord │ Telegram │ Slack │ WebChat │ Signal │ +20 more  │
├────────────────────────────────────────────────────────────┤
│                    AGENT RUNTIME (Rust Core)                 │
│  ┌──────────┬──────────┬──────────┬─────────────────────┐  │
│  │  Agent   │  Memory  │  Skills  │  Tools              │  │
│  │  Loop    │  FTS     │  Engine  │  Read/Write/Exec    │  │
│  │          │  Vector  │  Extract │  WebSearch/Fetch    │  │
│  │          │  Graph   │  Refine  │  + plugins          │  │
│  └──────────┴──────────┴──────────┴─────────────────────┘  │
│  ┌──────────┬──────────┬──────────┬─────────────────────┐  │
│  │Trajectory│Checkpoint│ Identity │  Audit              │  │
│  │Recording │Rollback  │ (DID)    │  (Hash-Chain)       │  │
│  └──────────┴──────────┴──────────┴─────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

## Features

### 🤖 Self-Improving Agent Loop
Every interaction is recorded as a **trajectory** — the agent learns from successful patterns and creates reusable **skills** automatically. Failed attempts trigger **skill refinement** with recovery instructions.

### 🧠 Three-Layer Memory
1. **Full-Text Search** — SQLite + Tantivy for keyword retrieval
2. **Vector Store** — Semantic similarity search with cosine distance
3. **Graph Memory** — Entity extraction and relationship tracking across conversations

### 📸 Checkpoint & Rollback
Every agent turn creates a snapshot. Roll back to any point, diff between states, or recover from failures. Essential for debugging and iterative development.

### 🔐 Identity & Audit (Coming)
- **Agent DNA** — Decentralized Identifiers (DID) for every agent
- **Immutable Audit Trail** — SHA-256 hash-chained action log
- **Cryptographic signing** — Verifiable proof of agent actions

### 🌐 Multi-Channel Gateway
Run the Go gateway alongside the Rust agent to connect 25+ messaging platforms:

```bash
# Start the gateway
cd gateway
go build -o nexus-gateway.exe .
./nexus-gateway.exe
```

Configure channels in `gateway/gateway.json`:

```json
{
  "port": 8080,
  "webchat":  { "enabled": true,  "path": "/ws" },
  "discord":  { "enabled": false, "bot_token": "" },
  "telegram": { "enabled": false, "bot_token": "" },
  "slack":    { "enabled": false, "bot_token": "" }
}
```

## CLI Commands

```bash
nexus init           # Initialize a Nexus workspace
nexus chat           # Interactive chat session
nexus run --prompt   # Run a single task
nexus config show    # View configuration
nexus skill list     # List installed skills
nexus skill install  # Install a skill from file
```

## Project Structure

```
nexus/
├── Cargo.toml              # Rust workspace root
├── core/                   # Agent runtime (Rust library)
│   └── src/
│       ├── agent/          # Agent loop, sessions
│       ├── memory/         # FTS, vector, graph, summarizer
│       ├── skills/         # Engine, parser, refiner
│       ├── tools/          # Tool dispatch + builtins
│       ├── providers/      # OpenAI, Anthropic, Ollama
│       ├── trajectory/     # Recording + extraction
│       ├── checkpoint/     # State snapshots
│       ├── identity/       # DID identity
│       ├── audit/          # Hash-chain audit trail
│       └── sandbox/        # WASM execution
├── cli/                    # CLI binary (nexus)
│   └── src/main.rs
├── gateway/                # Go messaging gateway
│   ├── main.go
│   └── internal/           # Channels, bus, sessions
├── docs/                   # Documentation
└── scripts/                # Build & setup scripts
```

## Development

```bash
# Build everything
cargo build --release

# Run tests
cargo test

# Build gateway
cd gateway && go build -o nexus-gateway.exe .
```

## Why Rust + Go?

| Layer | Language | Why |
|-------|----------|-----|
| **Agent Runtime** | Rust | Performance, memory safety, zero-cost abstractions, cross-compilation |
| **Gateway** | Go | Goroutine-per-channel, fast compile times, excellent HTTP/WS libraries |
| **CLI** | Rust | Single static binary, fast startup, no runtime dependency |

## Roadmap

- [x] **Phase 1:** Core agent loop, CLI, tools, 3 LLM providers
- [x] **Phase 2:** Multi-channel gateway (Discord, Telegram, Slack, WebChat)
- [x] **Phase 3:** Learning loop, trajectory recording, skill extraction, graph memory
- [ ] **Phase 4:** Agent DNA (DID identity), immutable audit trail
- [ ] **Phase 5:** Visual Agent Studio (drag-and-drop workflows)
- [ ] **Phase 6:** Agent roaming, P2P network, federated learning
- [ ] **Phase 7:** Mobile apps (iOS + Android)

## License

MIT — see [LICENSE](LICENSE)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## Security

See [SECURITY.md](SECURITY.md)

---

*Built to change the autonomous agent era.* 🦞
