# Changelog

All notable changes to Nexus will be documented in this file.

## [0.5.0] - 2026-06-02

### Added
- Trajectory recording — every agent action captured with timing and IO
- Skill extraction engine — auto-generates SKILL.md from successful trajectories
- Skill refiner — learns from failures, patches skills with recovery steps
- Memory summarization — LLM-based conversation compression
- Graph memory — entity extraction with typed relationships
- Vector store — semantic similarity search with cosine distance
- Checkpoint/rollback — snapshot/restore agent state at any turn
- CLI `chat` mode with session support

### Changed
- AgentLoop fully integrated with memory, trajectory, and checkpoint subsystems
- Core library exposed via `lib.rs` re-exports

## [0.4.0] - 2026-05-01

### Added
- Discord channel (send messages, listen to events)
- Telegram channel (send messages, inline keyboard support)
- Slack channel (RTM + Web API)
- WebSocket WebChat channel
- In-process message bus
- Session manager with turn history

## [0.3.0] - 2026-04-15

### Added
- Go gateway scaffold with `main.go`
- Channel interface
- Message bus skeleton

## [0.2.0] - 2026-04-01

### Added
- Rust workspace with `core/` library and `cli/` binary
- OpenAI provider
- Anthropic provider
- Ollama provider
- Tool system with built-in tools (read, write, search, fetch, exec)
- Skill engine with parser
- CLI `init`, `config show`, `skill list`, `skill install`

## [0.1.0] - 2026-03-15

### Added
- Project architecture planning
- Competitive analysis (OpenClaw vs Hermes Agent)
- Tech stack decisions
- Repository scaffold
