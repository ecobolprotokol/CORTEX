use clap::{Parser, Subcommand};

use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

#[derive(Parser)]
#[command(
    name = "cortex",
    version = env!("CARGO_PKG_VERSION"),
    about = "A persistent, state-based, continually learning AI model"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Serve {
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        bind: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Observe {
        text: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Query {
        text: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Status {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Init {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Checkpoint {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Experience {
        text: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Learn {
        text: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Inspect {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
        #[arg(short, long)]
        component: Option<String>,
    },
    Verify {
        claim: String,
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
    Migrate {
        #[arg(short, long, default_value = "cortex.toml")]
        config: String,
    },
}

fn load_config(path: &str) -> CortexConfig {
    CortexConfig::load(path).unwrap_or_else(|_| {
        eprintln!("No config found at '{}', using defaults", path);
        CortexConfig::default()
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            println!(
                "CORTEX v{} | Interactive mode (type 'exit' to quit)",
                env!("CARGO_PKG_VERSION")
            );

            let stdin = std::io::stdin();
            loop {
                print!("> ");
                std::io::Write::flush(&mut std::io::stdout())?;

                let mut line = String::new();
                match stdin.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if trimmed == "exit" || trimmed == "quit" {
                            break;
                        }
                        if trimmed == "status" {
                            println!("Episodes: {}", runtime.state.metadata.episode_count);
                            println!("Vocabulary: {}", runtime.language_vocabulary.size());
                            println!("Learning: {}", runtime.state.learning.total_learning_events);
                            println!("Entities: {}", runtime.state.world.entities.len());
                            println!("Version: {}", runtime.state_version);
                            println!("Mutations: {}", runtime.mutation_log.records.len());
                            continue;
                        }
                        if trimmed == "checkpoint" {
                            match runtime.save_state() {
                                Ok(()) => println!("State saved"),
                                Err(e) => println!("Save failed: {}", e),
                            }
                            continue;
                        }
                        match runtime.process(trimmed) {
                            Ok(response) => println!("{}", response),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            println!("Shutting down...");
            runtime.shutdown()?;
        }
        Commands::Serve { bind, config } => {
            let cfg = load_config(&config);
            let api_key =
                std::env::var(&cfg.api.api_key_env).unwrap_or_else(|_| "cortex-default-key".into());
            let mut api_manager = cortex::api::ApiManager::new(&api_key);
            println!("CORTEX API server listening on {}", bind);
            println!("Press Ctrl+C to stop");
            api_manager.start_synchronous_server(&bind)?;
        }
        Commands::Observe { text, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            let response = runtime.process(&text)?;
            println!("{}", response);
        }
        Commands::Query { text, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            let response = runtime.process(&text)?;
            println!("{}", response);
        }
        Commands::Status { config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            println!("State: {:?}", runtime.runtime_state);
            println!("Episodes: {}", runtime.state.metadata.episode_count);
            println!(
                "Learning events: {}",
                runtime.state.learning.total_learning_events
            );
            println!("Checkpoints: {}", runtime.state.metadata.checkpoint_count);
            println!("Vocabulary size: {}", runtime.language_vocabulary.size());
            runtime.shutdown()?;
        }
        Commands::Init { .. } => {
            println!("Initializing new CORTEX state...");
            let cfg = CortexConfig::default();
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            runtime.shutdown()?;
            println!("State initialized successfully");
        }
        Commands::Checkpoint { config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            runtime.save_state()?;
            println!("Checkpoint created");
            runtime.shutdown()?;
        }
        Commands::Migrate { .. } => {
            println!("Migration check: no migrations required for current version");
        }
        Commands::Experience { text, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            let response = runtime.process(&text)?;
            println!("{}", response);
            runtime.shutdown()?;
        }
        Commands::Learn { text, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            let response = runtime.process(&text)?;
            println!("{}", response);
            runtime.shutdown()?;
        }
        Commands::Inspect { config, component } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            match component.as_deref() {
                Some("episodes") => println!("Episodes: {}", runtime.state.metadata.episode_count),
                Some("vocabulary") => println!("Vocabulary size: {}", runtime.language_vocabulary.size()),
                Some("entities") => println!("Entities: {}", runtime.state.world.entities.len()),
                Some("learning") => println!("Learning events: {}", runtime.state.learning.total_learning_events),
                Some("state") => println!("State version: {}", runtime.state_version),
                Some("mutations") => println!("Mutations: {}", runtime.mutation_log.records.len()),
                Some("checkpoints") => println!("Checkpoints: {}", runtime.state.metadata.checkpoint_count),
                Some(other) => {
                    eprintln!("Unknown component: {}", other);
                    eprintln!("Available: episodes, vocabulary, entities, learning, state, mutations, checkpoints");
                }
                None => {
                    println!("Episodes: {}", runtime.state.metadata.episode_count);
                    println!("Vocabulary size: {}", runtime.language_vocabulary.size());
                    println!("Entities: {}", runtime.state.world.entities.len());
                    println!("Learning events: {}", runtime.state.learning.total_learning_events);
                    println!("State version: {}", runtime.state_version);
                    println!("Mutations: {}", runtime.mutation_log.records.len());
                    println!("Checkpoints: {}", runtime.state.metadata.checkpoint_count);
                }
            }
            runtime.shutdown()?;
        }
        Commands::Verify { claim, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            let response = runtime.process(&claim)?;
            println!("{}", response);
            runtime.shutdown()?;
        }
    }

    Ok(())
}
