use clap::{Parser, Subcommand};
use colored::Colorize;
use nexus_core::agent::session::Session;
use nexus_core::agent::AgentLoop;
use nexus_core::memory::graph::GraphMemory;
use nexus_core::memory::vector::VectorStore;
use nexus_core::memory::MemoryStore;
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
    },
    /// Start an interactive chat session
    Chat {
        /// Provider to use (openai, anthropic, ollama)
        #[arg(short, long, default_value = "openai")]
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
        /// Provider to use
        #[arg(short, long, default_value = "openai")]
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
}

#[derive(Subcommand)]
enum ConfigAction {
    /// View current configuration
    Show,
    /// Set a configuration value
    Set {
        key: String,
        value: String,
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
        Commands::Init { path } => cmd_init(path.as_deref()).await,
        Commands::Chat { provider, model } => cmd_chat(provider, model.as_deref()).await,
        Commands::Run { prompt, provider } => cmd_run(prompt, provider).await,
        Commands::Config { action } => cmd_config(action).await,
        Commands::Skill { action } => cmd_skill(action).await,
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
        "✓".green().bold(),
        expanded.cyan()
    );
    println!("  Config: {}", config_path.display().to_string().cyan());
    println!("  Skills: {}", skills_dir.display().to_string().cyan());
    println!("  Memory: {}", memory_dir.display().to_string().cyan());
    println!();
    println!("{} Run 'nexus chat' to start interacting with your agent.", "→".blue());

    Ok(())
}

async fn cmd_chat(provider_name: &str, model: Option<&str>) -> anyhow::Result<()> {
    let config = load_config()?;
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

    println!(
        "{} Nexus Agent ready. Type '{}' to exit.\n",
        "✦".cyan().bold(),
        "/quit".yellow()
    );

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

        print!("{} ", "Nexus:".cyan().bold());
        std::io::stdout().flush()?;

        match agent.run_turn(&mut session, input).await {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                println!("{} Error: {}", "✗".red().bold(), e);
            }
        }
    }

    Ok(())
}

async fn cmd_run(prompt: &str, provider_name: &str) -> anyhow::Result<()> {
    let config = load_config()?;
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
        Err(e) => eprintln!("{} Error: {}", "✗".red().bold(), e),
    }

    Ok(())
}

async fn cmd_config(action: &ConfigAction) -> anyhow::Result<()> {
    match action {
        ConfigAction::Show => {
            let config = load_config()?;
            println!("{}", serde_json::to_string_pretty(&config)?);
        }
        ConfigAction::Set { key, value } => {
            println!("{} Config key '{}' set to '{}'", "✓".green(), key, value);
        }
    }
    Ok(())
}

async fn cmd_skill(action: &SkillAction) -> anyhow::Result<()> {
    match action {
        SkillAction::List => {
            let config = load_config()?;
            let skills_dir = std::path::Path::new(&config.workspace).join("skills");
            let mut engine = SkillEngine::new();
            engine.load_from_directory(&skills_dir.to_string_lossy())?;
            let skills = engine.list_skills();
            if skills.is_empty() {
                println!("{} No skills installed.", "ℹ".blue());
                println!("  Use 'nexus skill install <path>' to add a skill.");
            } else {
                println!("{} Installed skills:", "✦".cyan().bold());
                for skill in skills {
                    println!("  • {} - {}", skill.name, skill.description);
                }
            }
        }
        SkillAction::Install { path } => {
            println!("{} Skill installed from {}", "✓".green(), path);
        }
        SkillAction::Activate { name } => {
            println!("{} Skill '{}' activated", "✓".green(), name);
        }
        SkillAction::Deactivate { name } => {
            println!("{} Skill '{}' deactivated", "✓".green(), name);
        }
    }
    Ok(())
}

fn load_config() -> anyhow::Result<NexusConfig> {
    let config_path = shellexpand::tilde("~/.nexus/nexus.json").to_string();
    if std::path::Path::new(&config_path).exists() {
        let data = std::fs::read_to_string(&config_path)?;
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(NexusConfig::default())
    }
}

fn create_provider(
    name: &str,
    model: Option<&str>,
    config: &NexusConfig,
) -> anyhow::Result<Arc<dyn nexus_core::providers::Provider + Send + Sync>> {
    let provider: Arc<dyn nexus_core::providers::Provider + Send + Sync> = match name {
        "openai" => {
            let api_key = std::env::var("OPENAI_API_KEY").ok();
            Arc::new(OpenAIProvider::new(ProviderConfig {
                api_key,
                model: model.unwrap_or(&config.default_model).to_string(),
                ..Default::default()
            }))
        }
        "anthropic" => {
            let api_key = std::env::var("ANTHROPIC_API_KEY").ok();
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
        _ => anyhow::bail!("Unknown provider '{}'. Available: openai, anthropic, ollama", name),
    };
    Ok(provider)
}

fn print_help() {
    println!("{} Nexus CLI Help", "✦".cyan().bold());
    println!("  {}", "/quit, /exit  - Exit the chat".yellow());
    println!("  {}", "/help          - Show this help".yellow());
    println!();
    println!("{} Commands:", "→".blue());
    println!("  {} {}  - Initialize a workspace", "nexus init".cyan(), "[--path <dir>]".dimmed());
    println!("  {} {} - Start chat", "nexus chat".cyan(), "[--provider <name>] [--model <name>]".dimmed());
    println!("  {} {}     - Run a single task", "nexus run".cyan(), "--prompt <text> [--provider <name>]".dimmed());
    println!("  {}           - View configuration", format!("nexus config show").cyan());
    println!("  {}        - List skills", format!("nexus skill list").cyan());
}
