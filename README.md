<h1 align="center">Nexus</h1>

<p align="center">
  <b>Autonomous AI Agent Platform — learns, remembers, and grows.</b>
</p>

<p align="center">
  <a href="https://github.com/nexus/nexus/actions"><img src="https://img.shields.io/github/actions/workflow/status/nexus/nexus/ci.yml?branch=main&style=flat-square" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.86+-orange.svg?style=flat-square" alt="Rust"></a>
  <a href="https://go.dev"><img src="https://img.shields.io/badge/go-1.22+-00ADD8.svg?style=flat-square" alt="Go"></a>
  <a href="https://github.com/nexus/nexus/releases"><img src="https://img.shields.io/github/v/release/nexus/nexus?style=flat-square" alt="Release"></a>
  <a href="https://github.com/nexus/nexus/stargazers"><img src="https://img.shields.io/github/stars/nexus/nexus?style=flat-square" alt="Stars"></a>
  <a href="https://discord.gg/nexus"><img src="https://img.shields.io/badge/chat-discord-5865F2.svg?style=flat-square" alt="Discord"></a>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#cli-commands">CLI Reference</a> •
  <a href="#channels">Channels</a> •
  <a href="#development">Development</a> •
  <a href="docs/SECURITY.md">Security</a>
</p>

<hr>

Nexus is an **end-to-end autonomous AI agent platform** built with Rust and Go. It combines the channel breadth of OpenClaw with the self-improving learning loop of Hermes Agent, adding unique innovations like graph memory, DID-based identity, and privacy-as-config.

**New here?** Start with the [Quick Start](#quick-start) — no API keys needed.

## Quick Start

### Option 1: Try it now (zero setup)

```bash
# Prerequisites: Rust 1.86+ (https://rustup.rs)
git clone https://github.com/nexus/nexus.git
cd nexus

# Build (takes ~1-2 min first time)
cargo build --release

# Initialize workspace
./target/release/nexus init

# Start chatting immediately — no API keys!
./target/release/nexus chat
```

```
✦ Nexus Agent ready (demo mode). Type '/quit' to exit.

  ℹ Run with a real provider:
    • nexus chat --provider openai  (set OPENAI_API_KEY)
    • nexus chat --provider ollama   (run Ollama locally)

You: hello
Nexus: Hello! I'm Nexus, your autonomous AI agent.
```

**Demo mode** works immediately with no accounts or keys. The agent simulates realistic responses and demonstrates the full CLI, memory, and skill system.

### Option 2: Docker (no Rust needed)

```bash
docker compose up nexus
```

### Option 3: Build from source

```bash
# Prerequisites
# - Rust 1.86+: https://rustup.rs
# - Go 1.22+:  https://go.dev/dl

git clone https://github.com/nexus/nexus.git
cd nexus
cargo build --release

# (Optional) Build the gateway
cd gateway && go build -o nexus-gateway && cd ..
```

### Connect a Real Provider

```bash
# OpenAI (set API key)
export OPENAI_API_KEY="sk-..."
nexus chat --provider openai

# Anthropic Claude
export ANTHROPIC_API_KEY="sk-ant-..."
nexus chat --provider anthropic

# Ollama (free, local — just install Ollama)
ollama pull llama3
nexus chat --provider ollama

# Or use the onboarding wizard
nexus onboard
```

### Run `nexus doctor` to check your setup

```bash
nexus doctor
```

Output:
```
🦞 Nexus System Check
──────────────────────────────────────────────────

  ✓ Workspace at /home/user/.nexus (4 files)
  ✓ Config file found

  → Providers:
    ✓ OpenAI API key configured
    ✗ Anthropic — run 'nexus config set api_keys.anthropic <key>'
    ✓ Ollama running at http://localhost:11434
    ✓ Demo mode always available (no setup needed)

  ℹ CLI version: 0.5.0
  ℹ Default provider: demo
```

## Features

### 🧠 Self-Improving Learning Loop

Every interaction is recorded as a **trajectory** — a detailed step-by-step log. When a task succeeds, Nexus analyzes the trajectory and automatically generates a reusable **skill**. When a skill fails, it's **refined** with recovery instructions.

```
User request → Agent processes → Tool calls → Response
                                                     ↓
                                          Trajectory recorded
                                                     ↓
                                          Success? → Create SKILL.md
                                          Failure? → Refine existing skill
```

### 💾 Three-Layer Memory

| Layer | Storage | Purpose |
|-------|---------|---------|
| **Full-Text Search** | SQLite + Tantivy | Keyword retrieval across all conversations |
| **Vector Store** | In-memory embeddings | Semantic similarity with cosine distance |
| **Graph Memory** | Entity + Relation graph | Knowledge tracking across sessions |

Memory is automatically summarized when conversations exceed 50 turns to preserve context.

### 📸 Checkpoint & Rollback

Every agent turn creates a snapshot. Roll back to any point, diff between states, or recover from failures.

### 🔧 Tool System

Built-in tools the agent can invoke autonomously:

| Tool | Description |
|------|-------------|
| `read` | Read files from filesystem |
| `write` | Write content to files |
| `search` | Web search via DuckDuckGo |
| `fetch` | Fetch and parse web pages |
| `exec` | Execute commands in sandbox |

### 🌐 Multi-Channel Gateway (Go)

Connect Nexus to messaging platforms via the Go gateway. See [CHANNELS.md](docs/CHANNELS.md) for full setup guides.

```bash
cd gateway && go build -o nexus-gateway .
./nexus-gateway
```

```json
{
  "port": 8080,
  "webchat":  { "enabled": true,  "path": "/ws" },
  "discord":  { "enabled": false, "bot_token": "" },
  "telegram": { "enabled": false, "bot_token": "" },
  "slack":    { "enabled": false, "bot_token": "" }
}
```

### 🔐 Security & Privacy

- **Sandboxed execution** — commands run with configurable resource limits (CPU, memory, network)
- **Local-first** — all memory and data stored on your machine by default
- **API key protection** — keys stored in config file (masked in output) or environment variables
- **4 privacy modes** (coming): Air-gapped, Local-first, Selective, Full

See [SECURITY.md](docs/SECURITY.md) for details.

## CLI Commands

| Command | Description |
|---------|-------------|
| `nexus init [--path]` | Initialize a Nexus workspace |
| `nexus onboard` | Interactive setup wizard |
| `nexus chat [--provider] [--model]` | Start interactive chat (default: demo) |
| `nexus run --prompt <text>` | Execute a single task |
| `nexus config show` | View configuration (keys masked) |
| `nexus config set <key> <value>` | Set configuration (e.g. `api_keys.openai sk-...`) |
| `nexus config delete <key>` | Clear a configuration key |
| `nexus skill list` | List installed skills |
| `nexus skill install <path>` | Install a skill |
| `nexus doctor` | System health check |

### Chat Commands

| Command | Description |
|---------|-------------|
| `/quit` or `/exit` | Exit chat |
| `/help` | Show chat help |
| `/doctor` | Run health check during chat |

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                     USER LAYER                            │
│   CLI    │   Terminal   │   Web Chat   │    API          │
├──────────────────────────────────────────────────────────┤
│                  CHANNEL LAYER (Go Gateway)               │
│  Discord │ Telegram │ Slack │ WebSocket  │  Message Bus  │
├──────────────────────────────────────────────────────────┤
│                  RUNTIME LAYER (Rust Core)                │
│  ┌──────────┬──────────┬──────────┬────────────────┐    │
│  │  Agent   │  Memory  │  Skills  │  Tools         │    │
│  │  Loop    │  FTS     │  Engine  │  read/write    │    │
│  │          │  Vector  │  Extract │  search/fetch  │    │
│  │          │  Graph   │  Refine  │  exec/sandbox  │    │
│  └──────────┴──────────┴──────────┴────────────────┘    │
│  ┌──────────┬──────────┬──────────┬────────────────┐    │
│  │Trajectory│Checkpoint│ Identity │  Audit         │    │
│  │Recording │Rollback  │ (DID)    │  (hash-chain)  │    │
│  └──────────┴──────────┴──────────┴────────────────┘    │
└──────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Message
  │
  ▼
Channel (Discord/Telegram/Slack/WebChat)
  │
  ▼
Message Bus → Gateway routes to agent
  │
  ▼
AgentLoop.run_turn()
  ├── CheckpointManager.snapshot()
  ├── TrajectoryRecorder.record_turn_start()
  ├── Provider.chat() ───► LLM API
  ├── Tool dispatch (read/write/search/fetch/exec)
  ├── MemoryStore.save_conversation()
  ├── VectorStore.insert()
  ├── GraphMemory.extract_entities()
  ├── TrajectoryRecorder.record_turn_end()
  └── SkillExtractor (auto-generate skill if successful)
```

## Channels

| Channel | Status | Setup |
|---------|--------|-------|
| WebChat | ✅ Ready | [Guide](docs/CHANNELS.md#webchat) |
| Discord | ✅ Ready | [Guide](docs/CHANNELS.md#discord) |
| Telegram | ✅ Ready | [Guide](docs/CHANNELS.md#telegram) |
| Slack | ✅ Ready | [Guide](docs/CHANNELS.md#slack) |
| Signal | 🔜 Planned | — |
| WhatsApp | 🔜 Planned | — |
| Matrix | 🔜 Planned | — |

## Project Structure

```
nexus/
├── Cargo.toml              # Rust workspace root
├── core/                   # Agent runtime library (Rust)
│   └── src/
│       ├── agent/          # Agent loop, session management
│       ├── memory/         # FTS, vector store, graph, summarizer
│       ├── skills/         # Engine, parser, refiner
│       ├── tools/          # Tool registry + built-in tools
│       ├── providers/      # OpenAI, Anthropic, Ollama, Demo
│       ├── trajectory/     # Recording + skill extraction
│       ├── checkpoint/     # State snapshots + rollback
│       └── identity/       # DID-based cryptographic identity
├── cli/                    # CLI binary (nexus)
│   └── src/main.rs
├── gateway/                # Go messaging gateway
│   ├── main.go
│   └── internal/           # Channels, message bus, sessions
├── docs/
│   ├── SECURITY.md         # Security model
│   ├── CHANNELS.md         # Channel setup guides
│   └── TROUBLESHOOTING.md  # Common issues
├── scripts/                # Setup scripts
├── Dockerfile              # Container build
└── docker-compose.yml      # Multi-service deployment
```

## Why Rust + Go?

| Layer | Language | Rationale |
|-------|----------|-----------|
| **Agent Runtime** | Rust | Performance, memory safety, zero-cost abstractions, single static binary |
| **Gateway** | Go | Goroutine-per-channel concurrency, fast compilation, excellent HTTP/WS libraries |
| **CLI** | Rust | Fast startup, no runtime dependency, cross-compilation |

## Development

```bash
# Build everything
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Build gateway
cd gateway && go build -o nexus-gateway .
```

### Prerequisites by Platform

| Platform | Requirements |
|----------|-------------|
| **Linux** | `sudo apt install pkg-config libssl-dev build-essential` |
| **macOS** | `xcode-select --install` |
| **Windows** | Rust via rustup.rs, Go via go.dev, MinGW-w64 for GNU linker |

## Roadmap

- [x] **Phase 1:** Core agent loop, CLI, 3 LLM providers, tool system
- [x] **Phase 2:** Multi-channel gateway (Discord, Telegram, Slack, WebChat)
- [x] **Phase 3:** Learning loop, trajectory recording, skill extraction, graph/vector memory, checkpointing
- [ ] **Phase 4:** Agent DNA (DID identity), immutable audit trail
- [ ] **Phase 5:** Visual Agent Studio (drag-and-drop workflow builder)
- [ ] **Phase 6:** Agent roaming (P2P network, federated learning)
- [ ] **Phase 7:** Mobile apps (iOS + Android)

## Comparison

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
| Tech Stack | TypeScript | TypeScript | **Rust + Go** |

## License

MIT — see [LICENSE](LICENSE)

## Contributing

We welcome contributions of all sizes. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

Report vulnerabilities per our [security policy](docs/SECURITY.md).

---

<p align="center">
  <a href="https://github.com/nexus/nexus">GitHub</a> •
  <a href="docs/CHANNELS.md">Channels</a> •
  <a href="docs/SECURITY.md">Security</a> •
  <a href="docs/TROUBLESHOOTING.md">Help</a>
</p>
