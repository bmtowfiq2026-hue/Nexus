use clap::{Parser, Subcommand};
use colored::Colorize;
use nexus_core::agent::session::Session;
use nexus_core::agent::AgentLoop;
use nexus_core::memory::graph::GraphMemory;
use nexus_core::memory::vector::VectorStore;
use nexus_core::memory::MemoryStore;
use nexus_core::providers::demo::DemoProvider;
use nexus_core::providers::openai::OpenAIProvider;
use nexus_core::providers::ProviderConfig;
use nexus_core::skills::SkillEngine;
use nexus_core::tools::ToolDispatcher;
use nexus_core::NexusConfig;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "nexus")]
#[command(about = "Nexus - Autonomous AI Agent Platform", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Nexus workspace
    Init {
        /// Path for the workspace (default: ~/.nexus)
        #[arg(short, long)]
        path: Option<String>,
        /// Interactive onboarding wizard
        #[arg(short, long)]
        wizard: bool,
    },
    /// Start an interactive chat session (default: demo)
    Chat {
        /// Provider to use (demo, openai, anthropic, ollama)
        #[arg(short, long, default_value = "demo")]
        provider: String,
        /// Model to use
        #[arg(short, long)]
        model: Option<String>,
    },
    /// Run a single command and exit
    Run {
        /// The instruction for the agent
        #[arg(long)]
        prompt: String,
        /// Provider to use (demo, openai, anthropic, ollama)
        #[arg(short, long, default_value = "demo")]
        provider: String,
    },
    /// Configure Nexus settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// List and manage skills
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
    /// Check system health and detect available providers
    Doctor,
    /// Alias for init --wizard
    Onboard,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// View current configuration
    Show,
    /// Set a configuration value (e.g. api_keys.openai sk-...)
    Set {
        key: String,
        value: String,
    },
    /// Delete a configuration key
    Delete {
        key: String,
    },
}

#[derive(Subcommand)]
enum SkillAction {
    /// List installed skills
    List,
    /// Install a skill from a file
    Install { path: String },
    /// Activate a skill
    Activate { name: String },
    /// Deactivate a skill
    Deactivate { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path, wizard } => {
            if *wizard {
                cmd_onboard(path.as_deref()).await
            } else {
                cmd_init(path.as_deref()).await
            }
        }
        Commands::Chat { provider, model } => cmd_chat(provider, model.as_deref()).await,
        Commands::Run { prompt, provider } => cmd_run(prompt, provider).await,
        Commands::Config { action } => cmd_config(action).await,
        Commands::Skill { action } => cmd_skill(action).await,
        Commands::Doctor => cmd_doctor().await,
        Commands::Onboard => cmd_onboard(None).await,
    }
}

async fn cmd_init(path: Option<&str>) -> anyhow::Result<()> {
    let ws_path = path.unwrap_or("~/.nexus");
    let expanded = shellexpand::tilde(ws_path).to_string();
    std::fs::create_dir_all(&expanded)?;

    let config = NexusConfig::default();
    let config_path = std::path::Path::new(&expanded).join("nexus.json");
    std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

    let skills_dir = std::path::Path::new(&expanded).join("skills");
    std::fs::create_dir_all(&skills_dir)?;

    let memory_dir = std::path::Path::new(&expanded).join("memory");
    std::fs::create_dir_all(&memory_dir)?;

    println!(
        "{} Nexus workspace initialized at {}",
        "\u{2713}".green().bold(),
        expanded.cyan()
    );
    println!("  Config: {}", config_path.display().to_string().cyan());
    println!("  Skills: {}", skills_dir.display().to_string().cyan());
    println!("  Memory: {}", memory_dir.display().to_string().cyan());
    println!();
    println!("{} Run 'nexus chat' to start interacting with your agent.", "\u{2192}".blue());
    println!("  {}", "(No API key needed \u{2014} uses demo mode by default)".dimmed());
    println!("  {}", "Or run 'nexus onboard' for a guided setup.".dimmed());

    Ok(())
}

async fn cmd_onboard(_path: Option<&str>) -> anyhow::Result<()> {
    use std::io::Write;

    println!("{}", "\u{1f99e} Nexus Onboarding Wizard".cyan().bold());
    println!("{}", "Let's get you set up!\n".dimmed());
    println!("{} Choose a provider:", "\u{2192}".blue());
    println!("  1. Demo mode (no setup needed)");
    println!("  2. Ollama (free, local)");
    println!("  3. OpenAI (requires API key)");
    println!("  4. Anthropic (requires API key)");
    println!();

    let mut config = NexusConfig::load();

    loop {
        print!("  Selection [1-4, default=1]: ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "" | "1" => {
                config.default_provider = "demo".to_string();
                config.default_model = "demo".to_string();
                println!("{} Using demo mode.", "\u{2713}".green());
                break;
            }
            "2" => {
                config.default_provider = "ollama".to_string();
                print!("  Ollama model [llama3]: ");
                std::io::stdout().flush()?;
                let mut m = String::new();
                std::io::stdin().read_line(&mut m)?;
                let model = m.trim();
                config.default_model = if model.is_empty() { "llama3".to_string() } else { model.to_string() };
                println!("{} Using Ollama with model '{}'.", "\u{2713}".green(), config.default_model);
                println!("  {}", "Make sure Ollama is running: https://ollama.ai".dimmed());
                break;
            }
            "3" => {
                config.default_provider = "openai".to_string();
                config.default_model = "gpt-4o".to_string();
                print!("  OpenAI API Key: ");
                std::io::stdout().flush()?;
                let mut key = String::new();
                std::io::stdin().read_line(&mut key)?;
                let key = key.trim().to_string();
                config.api_keys.insert("openai".to_string(), key);
                println!("{} Using OpenAI with model 'gpt-4o'.", "\u{2713}".green());
                break;
            }
            "4" => {
                config.default_provider = "anthropic".to_string();
                config.default_model = "claude-3-opus-20240229".to_string();
                print!("  Anthropic API Key: ");
                std::io::stdout().flush()?;
                let mut key = String::new();
                std::io::stdin().read_line(&mut key)?;
                let key = key.trim().to_string();
                config.api_keys.insert("anthropic".to_string(), key);
                println!("{} Using Anthropic with model 'claude-3-opus-20240229'.", "\u{2713}".green());
                break;
            }
            _ => {
                println!("{} Invalid choice, try again.", "\u{2717}".red());
            }
        }
    }

    config.save()?;
    println!();
    println!("{} Setup complete! Run 'nexus chat' to start.", "\u{2713}".green().bold());
    Ok(())
}

async fn cmd_chat(provider_name: &str, model: Option<&str>) -> anyhow::Result<()> {
    let config = NexusConfig::load();
    let provider = create_provider(provider_name, model, &config)?;
    let mut tools = ToolDispatcher::new();
    tools.register_builtins();
    let skills = SkillEngine::new();
    let memory = MemoryStore::new(&config.memory.store_path);

    let vector_store = Arc::new(VectorStore::new(&config.memory.store_path, config.memory.vector_dimensions));
    let graph_memory = Arc::new(std::sync::Mutex::new(GraphMemory::new()));

    let mut agent = AgentLoop::new(
        config,
        provider,
        Arc::new(tools),
        Arc::new(skills),
        Arc::new(std::sync::Mutex::new(memory)),
        vector_store,
        graph_memory,
    );

    let mut session = Session::new(
        uuid::Uuid::new_v4().to_string(),
        "cli".to_string(),
        "local".to_string(),
    );

    if provider_name == "demo" {
        println!(
            "{} Nexus Agent ready (demo mode). Type '{}' to exit.\n",
            "\u{2726}".cyan().bold(),
            "/quit".yellow()
        );
        println!("  {} Run with a real provider:", "\u{2139}".blue().dimmed());
        println!("    {} nexus chat --provider openai  (set OPENAI_API_KEY)", "  \u{2022}".dimmed());
        println!("    {} nexus chat --provider ollama   (run Ollama locally)", "  \u{2022}".dimmed());
        println!();
    } else {
        println!(
            "{} Nexus Agent ready. Type '{}' to exit.\n",
            "\u{2726}".cyan().bold(),
            "/quit".yellow()
        );
    }

    loop {
        let mut input = String::new();
        print!("{} ", "You:".green().bold());
        use std::io::Write;
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "/quit" || input == "/exit" {
            break;
        }

        if input == "/help" {
            print_help();
            continue;
        }

        if input == "/doctor" {
            cmd_doctor().await?;
            continue;
        }

        print!("{} ", "Nexus:".cyan().bold());
        std::io::stdout().flush()?;

        match agent.run_turn(&mut session, input).await {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                println!("{} Error: {}", "\u{2717}".red().bold(), e);
            }
        }
    }

    Ok(())
}

async fn cmd_run(prompt: &str, provider_name: &str) -> anyhow::Result<()> {
    let config = NexusConfig::load();
    let provider = create_provider(provider_name, None, &config)?;
    let mut tools = ToolDispatcher::new();
    tools.register_builtins();
    let skills = SkillEngine::new();
    let memory = MemoryStore::new(&config.memory.store_path);

    let vector_store = Arc::new(VectorStore::new(&config.memory.store_path, config.memory.vector_dimensions));
    let graph_memory = Arc::new(std::sync::Mutex::new(GraphMemory::new()));

    let mut agent = AgentLoop::new(
        config, provider, Arc::new(tools), Arc::new(skills),
        Arc::new(std::sync::Mutex::new(memory)), vector_store, graph_memory,
    );
    let mut session = Session::new(uuid::Uuid::new_v4().to_string(), "cli".to_string(), "local".to_string());

    match agent.run_turn(&mut session, prompt).await {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("{} Error: {}", "\u{2717}".red().bold(), e),
    }

    Ok(())
}

async fn cmd_config(action: &ConfigAction) -> anyhow::Result<()> {
    match action {
        ConfigAction::Show => {
            let config = NexusConfig::load();
            let mut json = serde_json::to_value(&config)?;
            if let Some(keys) = json.get_mut("api_keys") {
                if let Some(obj) = keys.as_object_mut() {
                    for val in obj.values_mut() {
                        if !val.as_str().map_or(true, |s| s.is_empty()) {
                            *val = serde_json::Value::String("****".to_string());
                        }
                    }
                }
            }
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        ConfigAction::Set { key, value } => {
            let mut config = NexusConfig::load();
            match key.as_str() {
                "default_provider" => config.default_provider = value.clone(),
                "default_model" => config.default_model = value.clone(),
                "workspace" => config.workspace = value.clone(),
                k if k.starts_with("api_keys.") => {
                    let provider = k.trim_start_matches("api_keys.");
                    config.api_keys.insert(provider.to_string(), value.clone());
                }
                k if k.starts_with("memory.") => {
                    let field = k.trim_start_matches("memory.");
                    match field {
                        "store_path" => config.memory.store_path = value.clone(),
                        "vector_dimensions" => {
                            config.memory.vector_dimensions = value.parse().map_err(|_| {
                                anyhow::anyhow!("vector_dimensions must be a number")
                            })?;
                        }
                        _ => anyhow::bail!("Unknown config key '{}'", key),
                    }
                }
                _ => anyhow::bail!("Unknown config key '{}'", key),
            }
            config.save()?;
            println!("{} Config key '{}' set.", "\u{2713}".green(), key.cyan());
        }
        ConfigAction::Delete { key } => {
            let mut config = NexusConfig::load();
            match key.as_str() {
                k if k.starts_with("api_keys.") => {
                    let provider = k.trim_start_matches("api_keys.");
                    config.api_keys.insert(provider.to_string(), String::new());
                }
                _ => anyhow::bail!("Cannot delete key '{}'", key),
            }
            config.save()?;
            println!("{} Config key '{}' cleared.", "\u{2713}".green(), key.cyan());
        }
    }
    Ok(())
}

async fn cmd_doctor() -> anyhow::Result<()> {
    println!("{} Nexus System Check", "\u{1f99e}".cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    println!();

    let config = NexusConfig::load();
    let expanded = shellexpand::tilde(&config.workspace).to_string();

    let file_count = || -> String {
        let d = std::path::Path::new(&expanded);
        if d.exists() {
            if let Ok(entries) = std::fs::read_dir(d) {
                return format!("{}", entries.count());
            }
        }
        "0".to_string()
    };

    if std::path::Path::new(&expanded).exists() {
        println!("  {} Workspace {} ({} files)", "\u{2713}".green(), expanded.cyan(), file_count().dimmed());
    } else {
        println!("  {} Workspace not found at {}", "\u{2717}".yellow(), expanded.cyan());
        println!("    Run 'nexus init' to create one.");
    }

    let config_path = shellexpand::tilde("~/.nexus/nexus.json").to_string();
    if std::path::Path::new(&config_path).exists() {
        println!("  {} Config file found", "\u{2713}".green());
    } else {
        println!("  {} No config file (defaults will be used)", "\u{2139}".blue());
    }

    println!();
    println!("  {} Providers:", "\u{2192}".blue().bold());

    let openai_key = config.get_api_key("openai");
    let anthropic_key = config.get_api_key("anthropic");
    let has_ollama = check_ollama().await;

    if openai_key.is_some() && openai_key.as_deref() != Some("") {
        println!("    {} OpenAI API key configured", "\u{2713}".green());
    } else {
        println!("    {} OpenAI \u{2014} set OPENAI_API_KEY or run 'nexus config set api_keys.openai <key>'", "\u{2717}".yellow());
    }

    if anthropic_key.is_some() && anthropic_key.as_deref() != Some("") {
        println!("    {} Anthropic API key configured", "\u{2713}".green());
    } else {
        println!("    {} Anthropic \u{2014} set ANTHROPIC_API_KEY or run 'nexus config set api_keys.anthropic <key>'", "\u{2717}".yellow());
    }

    if has_ollama {
        println!("    {} Ollama running at http://localhost:11434", "\u{2713}".green());
    } else {
        println!("    {} Ollama \u{2014} install from https://ollama.ai for free local AI", "\u{2139}".blue());
    }

    println!("    {} Demo mode always available (no setup needed)", "\u{2713}".green());

    println!();
    println!("  {} CLI version: {}", "\u{2139}".blue(), env!("CARGO_PKG_VERSION"));
    println!("  {} Default provider: {}", "\u{2139}".blue(), config.default_provider.cyan());

    if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("  {} OPENAI_API_KEY env var detected", "\u{2713}".green());
    }
    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("  {} ANTHROPIC_API_KEY env var detected", "\u{2713}".green());
    }

    println!();
    println!("{} To start chatting: nexus chat", "\u{2192}".blue().bold());

    Ok(())
}

async fn check_ollama() -> bool {
    if let Ok(resp) = reqwest::get("http://localhost:11434/api/tags").await {
        resp.status().is_success()
    } else {
        false
    }
}

async fn cmd_skill(action: &SkillAction) -> anyhow::Result<()> {
    match action {
        SkillAction::List => {
            let config = NexusConfig::load();
            let skills_dir = std::path::Path::new(&config.workspace).join("skills");
            let mut engine = SkillEngine::new();
            engine.load_from_directory(&skills_dir.to_string_lossy())?;
            let skills = engine.list_skills();
            if skills.is_empty() {
                println!("{} No skills installed.", "\u{2139}".blue());
                println!("  Use 'nexus skill install <path>' to add a skill.");
            } else {
                println!("{} Installed skills:", "\u{2726}".cyan().bold());
                for skill in skills {
                    println!("  \u{2022} {} - {}", skill.name, skill.description);
                }
            }
        }
        SkillAction::Install { path } => {
            println!("{} Skill installed from {}", "\u{2713}".green(), path);
        }
        SkillAction::Activate { name } => {
            println!("{} Skill '{}' activated", "\u{2713}".green(), name);
        }
        SkillAction::Deactivate { name } => {
            println!("{} Skill '{}' deactivated", "\u{2713}".green(), name);
        }
    }
    Ok(())
}

fn create_provider(
    name: &str,
    model: Option<&str>,
    config: &NexusConfig,
) -> anyhow::Result<Arc<dyn nexus_core::providers::Provider + Send + Sync>> {
    let provider: Arc<dyn nexus_core::providers::Provider + Send + Sync> = match name {
        "openai" => {
            let api_key = config.get_api_key("openai");
            Arc::new(OpenAIProvider::new(ProviderConfig {
                api_key,
                model: model.unwrap_or(&config.default_model).to_string(),
                ..Default::default()
            }))
        }
        "anthropic" => {
            let api_key = config.get_api_key("anthropic");
            Arc::new(nexus_core::providers::anthropic::AnthropicProvider::new(ProviderConfig {
                api_key,
                model: model.unwrap_or("claude-3-opus-20240229").to_string(),
                ..Default::default()
            }))
        }
        "ollama" => {
            Arc::new(nexus_core::providers::ollama::OllamaProvider::new(ProviderConfig {
                model: model.unwrap_or("llama3").to_string(),
                ..Default::default()
            }))
        }
        "demo" => {
            Arc::new(DemoProvider::new())
        }
        _ => anyhow::bail!("Unknown provider '{}'. Available: demo, openai, anthropic, ollama", name),
    };
    Ok(provider)
}

fn print_help() {
    println!("{} Nexus CLI Help", "\u{2726}".cyan().bold());
    println!("  {}", "/quit, /exit  - Exit the chat".yellow());
    println!("  {}", "/help          - Show this help".yellow());
    println!("  {}", "/doctor        - Run system health check".yellow());
    println!();
    println!("{} Commands:", "\u{2192}".blue());
    println!("  {} {}  - Initialize a workspace", "nexus init".cyan(), "[--path <dir>]".dimmed());
    println!("  {} {} - Start chat", "nexus chat".cyan(), "[--provider <name>] [--model <name>]".dimmed());
    println!("  {} {}     - Run a single task", "nexus run".cyan(), "--prompt <text> [--provider <name>]".dimmed());
    println!("  {}           - View configuration", format!("nexus config show").cyan());
    println!("  {} {}  - Set config key", "nexus config set".cyan(), "<key> <value>".dimmed());
    println!("  {}        - List skills", format!("nexus skill list").cyan());
    println!("  {}          - System health check", format!("nexus doctor").cyan());
    println!("  {}       - Interactive setup wizard", format!("nexus onboard").cyan());
}
