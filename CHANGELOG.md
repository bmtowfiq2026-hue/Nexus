# Changelog

## v0.5.0 (2026-06-04)

### 🎉 New: 12 Gateway Channels
Added messaging channel support to the Go gateway — connect Nexus to any platform:

- **Matrix** — Client-Server API via webhook wrapper
- **WhatsApp** — Webhook-based (Meta API)
- **Signal** — Webhook-based (Signal Messenger REST API)
- **IRC** — Native TCP implementation (NICK/USER/JOIN/PING-PONG)
- **Google Chat** — Incoming webhook
- **MSTeams** — Incoming webhook
- **LINE** — Webhook-based (LINE Messaging API)
- **Messenger** — Webhook-based (Facebook Messenger API)
- **Twilio** — Webhook-based (Twilio SMS/WhatsApp API)
- Plus existing: **WebChat**, **Discord**, **Telegram**, **Slack**

All channels are disabled by default in `gateway.json` — set `"enabled": true` and add credentials to activate.

### 🎉 New: 20+ LLM Providers
Added generic OpenAI-compatible provider adapter supporting 22 providers:

- **Google Gemini**, **DeepSeek**, **Groq**, **Together AI**, **Fireworks AI**
- **OpenRouter**, **Perplexity**, **Mistral AI**, **Cohere**, **AI21 Labs**
- **Replicate**, **HuggingFace**, **Cerebras**, **xAI (Grok)**
- **LM Studio**, **LocalAI**, **oobabooga** (local/self-hosted)
- **DeepInfra**, **SambaNova**, **Anyscale**
- Plus existing: **OpenAI**, **Anthropic**, **Ollama**, **Demo**

Set `<PROVIDER>_API_KEY` env var or use `nexus config set api_keys.<name> <key>`.

### 🎉 New: Webchat UI
Embedded HTML/CSS/JS chat interface served by the gateway at `/`:
- WebSocket real-time messaging, auto-reconnect
- Dark purple theme, session management
- Markdown message rendering

### 🎉 New: Onboard Setup Wizard
Interactive `nexus onboard` command — guides through provider selection and API key setup.

### 🎉 New: Logo & Website
- Professional SVG logos (neural network hexagon + "N") at `docs/assets/`
- Pure static marketing site at `https://bmtowfiq2026-hue.github.io/nexus-website/`
- No build step, no framework — single HTML file

### 🔧 Improvements
- Binary renamed from `nexus-cli` to `nexus` for simpler invocation
- `nexus doctor` checks all configured providers
- Gateway JSON config restructured with per-channel config objects
- Install scripts updated for new binary name

### 🐛 Fixes
- Vercel 404 resolved by replacing Next.js with pure static HTML

### 📦 Package
- Windows x86_64 release package: `nexus.exe` + `nexus-gateway.exe` + `gateway.json` + install script

## v0.4.0 (2026-05-??)

### Core Agent
- Agent loop with tool calling, session management
- Tool system: read, write, exec, web_search, web_fetch
- Memory: SQLite full-text, vector store (cosine similarity), graph memory (entity/relation)

### CLI
- `nexus init`, `nexus chat`, `nexus run`, `nexus config`, `nexus skill`, `nexus doctor`
- Demo mode (no API keys needed)
- `--provider` flag for OpenAI/Anthropic/Ollama/Demo

### Gateway
- Discord, Telegram, Slack, WebSocket channels
- Message bus routing architecture

### Skills & Learning
- Trajectory recording, skill extraction, skill refinement
- Checkpoint/rollback system
