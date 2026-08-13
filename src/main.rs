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
            println!("CORTEX v{} | Ready", env!("CARGO_PKG_VERSION"));
            runtime.run()?;
            println!("Shutting down...");
            runtime.shutdown()?;
        }
        Commands::Serve { bind, config } => {
            let cfg = load_config(&config);
            let mut runtime = CortexRuntime::new(cfg)?;
            runtime.boot()?;
            println!("CORTEX API server listening on {}", bind);
            println!("Press Ctrl+C to stop");
            runtime.run()?;
            runtime.shutdown()?;
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
            println!("Learning events: {}", runtime.state.learning.total_learning_events);
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
            runtime.persistence_checkpoint.create_checkpoint(
                runtime.memory_episodic.episodes.len() as u64,
                runtime.memory_episodic.next_id,
            );
            runtime.state.metadata.checkpoint_count += 1;
            println!("Checkpoint created (#{})", runtime.state.metadata.checkpoint_count);
            runtime.shutdown()?;
        }
        Commands::Migrate { .. } => {
            println!("Migration check: no migrations required for current version");
        }
    }

    Ok(())
}
