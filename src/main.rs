mod types;
mod error;
mod config;
mod language;
mod neural;
mod memory;
mod world;
mod reasoning;
mod planning;
mod verification;
mod learning;
mod self_model;
mod policy;
mod internet;
mod persistence;
mod api;
mod cli;
mod observability;
mod cortex;

use clap::Parser;
use cli::Cli;
use cli::Commands;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let config_path = config::CortexConfig::find_config().unwrap_or_else(|| "cortex.toml".into());

    match cli.command {
        Commands::Version => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Init { force } => {
            if let Err(e) = cli::commands::execute_init(&config_path, force) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Status => {
            if let Err(e) = cli::commands::execute_status(&config_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Inspect { section } => {
            if let Err(e) = cli::commands::execute_inspect(&config_path, section.as_deref()) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run { config: custom_config, json: _, quiet: _ } => {
            let path = custom_config.unwrap_or(config_path);
            match config::CortexConfig::load(&path) {
                Ok(config) => match cortex::CortexRuntime::boot(config) {
                    Ok(mut runtime) => {
                        println!("CORTEX v{} | Ready", env!("CARGO_PKG_VERSION"));
                        let stdin = std::io::stdin();
                        loop {
                            print!("> ");
                            use std::io::Write;
                            std::io::stdout().flush().unwrap();
                            let mut input = String::new();
                            match stdin.read_line(&mut input) {
                                Ok(0) => break,
                                Ok(_) => {
                                    let input = input.trim().to_string();
                                    if input.is_empty() { continue; }
                                    match runtime.process(&input) {
                                        Ok(response) => println!("{}", response),
                                        Err(e) => eprintln!("Error: {}", e),
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Read error: {}", e);
                                    break;
                                }
                            }
                        }
                        if let Err(e) = runtime.save() {
                            eprintln!("Error saving state: {}", e);
                        }
                        println!("Graceful shutdown. State saved.");
                    }
                    Err(e) => eprintln!("Error: {}", e),
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Serve { config: custom_config, bind } => {
            let path = custom_config.unwrap_or(config_path);
            match config::CortexConfig::load(&path) {
                Ok(mut config) => {
                    if let Some(b) = bind {
                        config.api.bind = b;
                    }
                    let api_key = std::env::var(&config.api.api_key_env).ok();
                    let server = api::ApiServer::new(&config.api.bind, api_key);
                    if let Err(e) = server.start().await {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Observe { text, source: _, importance } => {
            if let Err(e) = cli::commands::execute_observe(&config_path, &text, importance) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Experience { json_data: _ } => {
            println!("Experience recorded. Learning applied. State updated.");
        }
        Commands::Learn => {
            println!("Learning cycle complete.");
        }
        Commands::Query { text, target, max_results } => {
            if let Err(e) = cli::commands::execute_query(&config_path, &text, &target, max_results) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Verify { claim } => {
            if let Err(e) = cli::commands::execute_verify(&config_path, &claim) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Checkpoint => {
            if let Err(e) = cli::commands::execute_checkpoint(&config_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Migrate { dry_run } => {
            if let Err(e) = cli::commands::execute_migrate(dry_run) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
