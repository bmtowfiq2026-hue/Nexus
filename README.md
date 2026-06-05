<p align="center">
  <img src="https://raw.githubusercontent.com/bmtowfiq2026-hue/Nexus/main/logo.svg" alt="Nexus" width="400">
</p>

<p align="center">
  <b>Autonomous AI Agent Platform — learns, remembers, and grows.</b>
</p>

<p align="center">
  <a href="https://github.com/bmtowfiq2026-hue/Nexus/actions"><img src="https://img.shields.io/github/actions/workflow/status/bmtowfiq2026-hue/Nexus/ci.yml?branch=main&style=flat-square" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.86+-orange.svg?style=flat-square" alt="Rust"></a>
  <a href="https://go.dev"><img src="https://img.shields.io/badge/go-1.22+-00ADD8.svg?style=flat-square" alt="Go"></a>
  <a href="https://github.com/bmtowfiq2026-hue/Nexus/releases"><img src="https://img.shields.io/github/v/release/bmtowfiq2026-hue/Nexus?style=flat-square" alt="Release"></a>
  <a href="https://github.com/bmtowfiq2026-hue/Nexus/stargazers"><img src="https://img.shields.io/github/stars/bmtowfiq2026-hue/Nexus?style=flat-square" alt="Stars"></a>
  <a href="https://github.com/bmtowfiq2026-hue/Nexus/discussions"><img src="https://img.shields.io/badge/discussions-181717.svg?style=flat-square" alt="Discussions"></a>
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

### One-Liner Install

Install Nexus with a single command — no manual cloning needed:

| Platform | Command |
|----------|---------|
| **macOS / Linux** | `curl -fsSL https://raw.githubusercontent.com/bmtowfiq2026-hue/Nexus/main/scripts/install.sh \| sh` |
| **Windows PowerShell** | `iwr -Uri https://raw.githubusercontent.com/bmtowfiq2026-hue/Nexus/main/scripts/install.ps1 \| iex` |

The script installs Rust (if missing), clones the repo, builds the binary, adds it to PATH, and runs `nexus init`. Then just run:

```bash
nexus chat              # CLI chat (demo mode, no API keys)
nexus start             # WebChat UI (auto-opens http://localhost:8080)
```

**First time?** Both work immediately — no API keys, no accounts, no cloud signup.

```
✦ Nexus Agent ready (demo mode). Type '/quit' to exit.
You: hello
Nexus: Hello! I'm Nexus, your autonomous AI agent.
```

**Zero setup. No API keys. No accounts. Works on Windows, macOS, and Linux.**

### Option 1: Try it now — zero setup, no API keys

**Demo mode** works immediately with no accounts or keys. The agent simulates realistic responses and demonstrates the full CLI, memory, and skill system.

#### Windows PowerShell

```powershell
# 1. Install Rust (one command, one-time)
winget install Rustlang.Rustup

# 2. Build Nexus
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release

# 3. Initialize workspace
.\target\release\nexus init

# 4. Start chatting!
.\target\release\nexus chat
```

#### Windows Terminal (cmd.exe)

```cmd
REM 1. Install Rust from https://rustup.rs
REM 2. Build Nexus
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release

REM 3. Initialize workspace
.\target\release\nexus init

REM 4. Start chatting!
.\target\release\nexus chat
.\target\release\nexus start    (opens WebChat UI at http://localhost:8080)
```

#### macOS / Linux

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Build Nexus
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release

# 3. Initialize workspace
./target/release/nexus init

# 4. Start chatting!
./target/release/nexus chat
./target/release/nexus start    (opens WebChat UI at http://localhost:8080)
```

### Option 2: Docker (no Rust needed)

All platforms:

```bash
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
docker compose up nexus
```

### Option 3: Install via Cargo (binary only — coming soon)

```bash
cargo install nexus
nexus init
nexus chat
```

> **Note:** If `nexus` is not found after install, add `~/.cargo/bin` to your PATH:
> - **Windows PowerShell:** `$env:Path += ";$env:USERPROFILE\.cargo\bin"`
> - **macOS/Linux:** `export PATH="$PATH:$HOME/.cargo/bin"` (add to `~/.bashrc` or `~/.zshrc`)

### Connect a Real Provider

Set the matching env var, then specify the provider:

| Provider | Env Var | Default Model | Command |
|----------|---------|--------------|---------|
| OpenAI | `OPENAI_API_KEY` | `gpt-4o` | `nexus chat --provider openai` |
| Anthropic | `ANTHROPIC_API_KEY` | `claude-3-opus-20240229` | `nexus chat --provider anthropic` |
| Google Gemini | `GEMINI_API_KEY` | `gemini-2.0-flash-exp` | `nexus chat --provider gemini` |
| DeepSeek | `DEEPSEEK_API_KEY` | `deepseek-chat` | `nexus chat --provider deepseek` |
| Groq | `GROQ_API_KEY` | `llama3-70b-8192` | `nexus chat --provider groq` |
| Together AI | `TOGETHER_API_KEY` | `mistralai/Mixtral-8x22B-Instruct-v0.1` | `nexus chat --provider together` |
| xAI (Grok) | `XAI_API_KEY` | `grok-beta` | `nexus chat --provider xai` |
| Perplexity | `PERPLEXITY_API_KEY` | `llama-3-sonar-large-32k-online` | `nexus chat --provider perplexity` |
| OpenRouter | `OPENROUTER_API_KEY` | `openai/gpt-4o` | `nexus chat --provider openrouter` |

#### Example (Windows PowerShell)
```powershell
$env:GEMINI_API_KEY="AIza..."
.\target\release\nexus chat --provider gemini
```

#### Example (macOS / Linux)
```bash
export DEEPSEEK_API_KEY="sk-..."
./target/release/nexus chat --provider deepseek
```

#### Or set it permanently:
```bash
nexus config set api_keys.gemini "AIza..."
nexus chat --provider gemini
```

#### Free alternative — Ollama (local, no API key):
```bash
# Install Ollama from https://ollama.ai
ollama pull llama3
nexus chat --provider ollama
```

#### Use the guided setup wizard:
```bash
nexus onboard
```

### Run `nexus doctor` to check your setup

```bash
nexus doctor
```

Output:
```
✦ Nexus System Check
──────────────────────────────────────────────────

  ✓ Workspace at /home/user/.nexus (4 files)
  ✓ Config file found

  → Providers:
    ✓ OpenAI API key configured
    ✓ Gemini (Google) via GEMINI_API_KEY env var
    ✓ Ollama running at http://localhost:11434
    ✓ Demo mode always available (no setup needed)
    ℹ Anthropic — set ANTHROPIC_API_KEY for Claude
    ℹ DeepSeek — set DEEPSEEK_API_KEY
    ℹ Groq — set GROQ_API_KEY
    ℹ 15+ more providers available via env vars

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
  "webchat":   { "enabled": true,  "path": "/ws" },
  "discord":   { "enabled": false, "bot_token": "" },
  "telegram":  { "enabled": false, "bot_token": "" },
  "slack":     { "enabled": false, "bot_token": "" },
  "matrix":    { "enabled": false, "homeserver_url": "", "access_token": "" },
  "whatsapp":  { "enabled": false, "webhook_secret": "", "verify_token": "" },
  "signal":    { "enabled": false, "phone_number": "" },
  "irc":       { "enabled": false, "server": "irc.libera.chat", "nick": "nexus-bot" },
  "googlechat": { "enabled": false, "webhook_url": "" },
  "msteams":   { "enabled": false, "webhook_url": "" },
  "line":      { "enabled": false, "channel_secret": "", "access_token": "" },
  "messenger": { "enabled": false, "page_access_token": "", "verify_token": "" },
  "twilio":    { "enabled": false, "account_sid": "", "auth_token": "" }
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
| `nexus start` | Launch agent API server + WebChat UI |
| `nexus serve` | Start agent API service (no UI) |
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

| Channel | Type | Status | Setup |
|---------|------|--------|-------|
| WebChat | WebSocket | ✅ Ready | [Guide](docs/CHANNELS.md#webchat) |
| Discord | Gateway API | ✅ Ready | [Guide](docs/CHANNELS.md#discord) |
| Telegram | Bot API | ✅ Ready | [Guide](docs/CHANNELS.md#telegram) |
| Slack | Events API | ✅ Ready | [Guide](docs/CHANNELS.md#slack) |
| Matrix | Client-Server API | ✅ Ready | [Guide](docs/CHANNELS.md#matrix) |
| IRC | Native TCP | ✅ Ready | [Guide](docs/CHANNELS.md#irc) |
| WhatsApp | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#whatsapp) |
| Signal | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#signal) |
| Google Chat | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#googlechat) |
| MSTeams | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#msteams) |
| LINE | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#line) |
| Messenger | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#messenger) |
| Twilio | Webhook | ✅ Ready | [Guide](docs/CHANNELS.md#twilio) |

## Providers

Nexus supports **20+ LLM providers** through a generic OpenAI-compatible adapter. Set the matching env var or use `nexus config set` to configure.

| Provider | Base URL | Default Model | Env Var |
|----------|----------|--------------|---------|
| **OpenAI** | `https://api.openai.com/v1` | `gpt-4o` | `OPENAI_API_KEY` |
| **Anthropic** | `https://api.anthropic.com/v1` | `claude-3-opus-20240229` | `ANTHROPIC_API_KEY` |
| **Google Gemini** | `https://generativelanguage.googleapis.com/v1beta` | `gemini-2.0-flash-exp` | `GEMINI_API_KEY` |
| **DeepSeek** | `https://api.deepseek.com/v1` | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| **Groq** | `https://api.groq.com/openai/v1` | `llama3-70b-8192` | `GROQ_API_KEY` |
| **Together AI** | `https://api.together.xyz/v1` | `mistralai/Mixtral-8x22B-Instruct-v0.1` | `TOGETHER_API_KEY` |
| **Fireworks AI** | `https://api.fireworks.ai/inference/v1` | `accounts/fireworks/models/llama-v3-70b-instruct` | `FIREWORKS_API_KEY` |
| **OpenRouter** | `https://openrouter.ai/api/v1` | `openai/gpt-4o` | `OPENROUTER_API_KEY` |
| **Perplexity** | `https://api.perplexity.ai` | `llama-3-sonar-large-32k-online` | `PERPLEXITY_API_KEY` |
| **Mistral AI** | `https://api.mistral.ai/v1` | `mistral-large-latest` | `MISTRAL_API_KEY` |
| **Cohere** | `https://api.cohere.com/v1` | `command-r-plus` | `COHERE_API_KEY` |
| **AI21 Labs** | `https://api.ai21.com/studio/v1` | `jamba-instruct` | `AI21_API_KEY` |
| **Replicate** | `https://api.replicate.com/v1` | `meta/llama-2-70b-chat` | `REPLICATE_API_KEY` |
| **HuggingFace** | `https://api-inference.huggingface.co/v1` | `meta-llama/Llama-3.1-70B-Instruct` | `HUGGINGFACE_API_KEY` |
| **Cerebras** | `https://api.cerebras.ai/v1` | `llama3.1-8b` | `CEREBRAS_API_KEY` |
| **xAI (Grok)** | `https://api.x.ai/v1` | `grok-beta` | `XAI_API_KEY` |
| **LM Studio** | `http://localhost:1234/v1` | `local-model` | `LM_STUDIO_API_KEY` |
| **LocalAI** | `http://localhost:8080/v1` | `gpt-4` | `LOCALAI_API_KEY` |
| **oobabooga** | `http://localhost:5000/v1` | `local-model` | `OOBABOOGA_API_KEY` |
| **DeepInfra** | `https://api.deepinfra.com/v1/openai` | `meta-llama/Meta-Llama-3.1-70B-Instruct` | `DEEPINFRA_API_KEY` |
| **SambaNova** | `https://api.sambanova.ai/v1` | `Meta-Llama-3.1-70B-Instruct` | `SAMBANOVA_API_KEY` |
| **Anyscale** | `https://api.endpoints.anyscale.com/v1` | `meta-llama/Meta-Llama-3-70B-Instruct` | `ANYSCALE_API_KEY` |
| **Ollama** ⚡ | `http://localhost:11434/v1` | `llama3` | — |
| **Demo** 🎯 | — | — | — |

> ⚡ **Ollama** runs locally with no API key required. Install from [ollama.ai](https://ollama.ai).
> 🎯 **Demo mode** works immediately with no setup — perfect for testing the CLI and tool system.

You can also configure any **custom OpenAI-compatible** endpoint:
```bash
nexus config set api_keys.base_url "https://my-custom-endpoint/v1"
nexus chat --provider openai_compat
```

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
│       ├── providers/      # 20+ providers (OpenAI, Anthropic, Ollama, Demo, Gemini, DeepSeek, Groq, etc.)
│       ├── trajectory/     # Recording + skill extraction
│       ├── checkpoint/     # State snapshots + rollback
│       └── identity/       # DID-based cryptographic identity
├── cli/                    # CLI binary (nexus)
│   └── src/main.rs
├── gateway/                # Go messaging gateway
│   ├── main.go
│   └── internal/channel/   # 12 channel implementations + webhook base
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

#### Windows (PowerShell)
```powershell
# Install Rust
# Option A: Download from https://rustup.rs
# Option B:
winget install Rustlang.Rustup

# Install Git
winget install Git.Git

# Build
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release

# If you get linker errors, switch to GNU toolchain:
rustup default stable-x86_64-pc-windows-gnu
```

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Xcode CLI tools (includes Git)
xcode-select --install

# Build
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release
```

#### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt update
sudo apt install curl pkg-config libssl-dev build-essential git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install pkg-config openssl-devel gcc git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/bmtowfiq2026-hue/Nexus.git
cd Nexus
cargo build --release
```

## Roadmap

- [x] **Phase 1:** Core agent loop, CLI, 20+ LLM providers, tool system
- [x] **Phase 2:** 13-channel gateway (Discord, Telegram, Slack, WebChat, Matrix, WhatsApp, Signal, IRC, Google Chat, MSTeams, LINE, Messenger, Twilio)
- [x] **Phase 3:** Learning loop, trajectory recording, skill extraction, graph/vector memory, checkpointing
- [ ] **Phase 4:** Webchat UI polish, session persistence, multi-user support
- [ ] **Phase 5:** Agent DNA (DID identity), immutable audit trail
- [ ] **Phase 6:** Visual Agent Studio (drag-and-drop workflow builder)
- [ ] **Phase 7:** Agent roaming (P2P network, federated learning)
- [ ] **Phase 8:** Mobile apps (iOS + Android)

## Comparison

| Feature | OpenClaw | Hermes Agent | **Nexus** |
|---------|----------|-------------|-----------|
| Channels | 20+ | 14 | **13** |
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
  <a href="https://github.com/bmtowfiq2026-hue/Nexus">GitHub</a> •
  <a href="docs/CHANNELS.md">Channels</a> •
  <a href="docs/SECURITY.md">Security</a> •
  <a href="docs/TROUBLESHOOTING.md">Help</a>
</p>
