use super::{Provider, ProviderRequest, ProviderResponse, TokenUsage};
use crate::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct DemoProvider;

impl DemoProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Provider for DemoProvider {
    fn name(&self) -> &str {
        "demo"
    }

    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse> {
        let content = match request.user_message.to_lowercase().trim() {
            msg if msg.contains("hello") || msg.contains("hi") || msg.contains("hey") =>
                "Hello! I'm Nexus, your autonomous AI agent. I'm currently running in **demo mode**.\n\n\
                 To unlock my full potential with a real LLM provider, set one of these environment variables:\n\n\
                 ```\n\
                 export OPENAI_API_KEY=\"sk-...\"       # OpenAI GPT-4o\n\
                 export ANTHROPIC_API_KEY=\"sk-ant-...\" # Anthropic Claude\n\
                 ```\n\n\
                 Or run Ollama locally (free, no API key needed):\n\
                 ```\n\
                 # Install Ollama: https://ollama.ai\n\
                 ollama pull llama3\n\
                 nexus chat --provider ollama\n\
                 ```\n\n\
                 In demo mode, I can still show you my capabilities. Try asking me:\n\
                 - \"what can you do?\"\n\
                 - \"show me your memory\"\n\
                 - \"list your tools\"",

            msg if msg.contains("what can you do") || msg.contains("capabilities") =>
                "I'm Nexus — an autonomous AI agent platform. Here's what I can do:\n\n\
                 **🧠 Memory & Learning**\n\
                 - I remember our conversations across sessions\n\
                 - I create skills from successful interactions\n\
                 - I improve my skills when I make mistakes\n\
                 - I build a knowledge graph of entities and relationships\n\n\
                 **🔧 Tools**\n\
                 - `read` — Read files from your filesystem\n\
                 - `write` — Write content to files\n\
                 - `search` — Search the web for information\n\
                 - `fetch` — Fetch content from URLs\n\
                 - `exec` — Execute commands safely in a sandbox\n\n\
                 **🌐 Channels (via Go Gateway)**\n\
                 - Discord, Telegram, Slack, WebSocket\n\
                 - Message bus for event-driven architecture\n\n\
                 **🛡️ Security**\n\
                 - Sandboxed execution with resource limits\n\
                 - Privacy modes (local-only by default)\n\
                 - Immutable audit trail (coming soon)\n\n\
                 Run `nexus chat --provider openai` with a real API key for the full experience!",

            msg if msg.contains("memory") || msg.contains("remember") =>
                "I use a three-layer memory system:\n\n\
                 **1. Full-Text Search** — I store every conversation turn and can search\n\
                    by keyword across all sessions.\n\
                 **2. Vector Store** — I compute embeddings of our conversations and\n\
                    find semantically similar past interactions.\n\
                 **3. Graph Memory** — I extract entities (people, concepts, technologies)\n\
                    and track relationships between them.\n\n\
                 I also summarize long conversations automatically to preserve\n\
                 key information without overflowing context windows.\n\n\
                 Everything is stored locally at `~/.nexus/memory/`.",

            msg if msg.contains("skill") =>
                "Skills are reusable instruction sets that I learn from successful interactions.\n\n\
                 **How skills work:**\n\
                 1. I record every interaction as a **trajectory** (step-by-step log)\n\
                 2. When a trajectory completes successfully, I analyze the pattern\n\
                 3. I extract reusable steps into a **SKILL.md** file\n\
                 4. Next time a similar request comes in, I activate the skill\n\
                 5. If a skill fails, I refine it with recovery instructions\n\n\
                 Try running some real tasks with a connected LLM provider\n\
                 and watch me build my skill library automatically!\n\n\
                 ```bash\n\
                 nexus skill list    # See installed skills\n\
                 ```",

            msg if msg.contains("tool") =>
                "I have several built-in tools at my disposal:\n\n\
                 | Tool | Description |\n\
                 |------|-------------|\n\
                 | `read` | Read files from the filesystem |\n\
                 | `write` | Write content to files |\n\
                 | `search` | Search the web via DuckDuckGo |\n\
                 | `fetch` | Fetch and parse web pages |\n\
                 | `exec` | Execute commands in a sandboxed environment |\n\n\
                 Tools are dispatched automatically when I think they're needed.\n\
                 Each tool call is recorded in the trajectory for learning.\n\n\
                 More tools can be added via the `ToolRegistry` in code.",

            _ =>
                "I'm Nexus in **demo mode**. I can simulate responses so you can explore my interface,\n\
                 but for real AI-powered conversations, connect a provider.\n\n\
                 **Try asking me one of these:**\n\
                 - \"what can you do?\" — See my full capabilities\n\
                 - \"show me your memory\" — Learn about my memory system\n\
                 - \"tell me about skills\" — How I learn and improve\n\
                 - \"list your tools\" — Available tool integrations\n\n\
                 **Or set up a real provider:**\n\
                 ```bash\n\
                 export OPENAI_API_KEY=\"sk-...\"\n\
                 nexus chat --provider openai\n\
                 ```\n\n\
                 Demo mode features:\n\
                 - ✅ Full CLI experience\n\
                 - ✅ Memory & trajectory recording\n\
                 - ✅ Skill extraction & refinement\n\
                 - ✅ Checkpoint & rollback\n\
                 - ✅ Graph memory & vector store\n\
                 - ❌ Real AI responses (connect a provider for these)",
        };

        Ok(ProviderResponse {
            content: content.to_string(),
            tool_calls: Vec::new(),
            usage: TokenUsage {
                input_tokens: 0,
                output_tokens: 0,
                total_tokens: 0,
            },
        })
    }

    async fn stream_complete(
        &self,
        _request: ProviderRequest,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>> {
        Err(crate::NexusError::Provider("Streaming not available in demo mode".to_string()))
    }
}
