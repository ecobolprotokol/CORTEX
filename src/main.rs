use clap::Parser;

#[derive(Parser)]
#[command(name = "cortex", version, about = "CORTEX - A persistent, state-based, continually learning AI model")]
struct Cli {
    #[command(subcommand)]
    command: cortex::cli::Commands,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let config_path = cortex::config::CortexConfig::find_config().unwrap_or_else(|| "cortex.toml".into());

    match cli.command {
        cortex::cli::Commands::Version => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
        }
        cortex::cli::Commands::Init { force } => {
            if let Err(e) = cortex::cli::commands::execute_init(&config_path, force) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Status => {
            if let Err(e) = cortex::cli::commands::execute_status(&config_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Inspect { section } => {
            if let Err(e) = cortex::cli::commands::execute_inspect(&config_path, section.as_deref()) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Run { config: custom_config, json: _, quiet: _ } => {
            let path = custom_config.unwrap_or(config_path);
            match cortex::config::CortexConfig::load(&path) {
                Ok(config) => match cortex::cortex::CortexRuntime::boot(config) {
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
        cortex::cli::Commands::Serve { config: custom_config, bind } => {
            let path = custom_config.unwrap_or(config_path);
            match cortex::config::CortexConfig::load(&path) {
                Ok(mut config) => {
                    if let Some(b) = bind {
                        config.api.bind = b;
                    }
                    let api_key = std::env::var(&config.api.api_key_env).ok();
                    let bind_addr = config.api.bind.clone();
                    match cortex::cortex::CortexRuntime::boot(config) {
                        Ok(runtime) => {
                            let runtime = std::sync::Arc::new(std::sync::Mutex::new(runtime));
                            let server = cortex::api::ApiServer::new(
                                &bind_addr,
                                api_key,
                            ).with_runtime(runtime);
                            if let Err(e) = server.start().await {
                                eprintln!("Error: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error booting runtime: {}", e),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        cortex::cli::Commands::Observe { text, source: _, importance } => {
            if let Err(e) = cortex::cli::commands::execute_observe(&config_path, &text, importance) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Experience { json_data } => {
            let path = config_path.clone();
            match cortex::config::CortexConfig::load(&path) {
                Ok(config) => match cortex::cortex::CortexRuntime::boot(config) {
                    Ok(mut runtime) => {
                        let input = json_data.trim();
                        match runtime.learn(input) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                std::process::exit(1);
                            }
                        }
                        if let Err(e) = runtime.save() {
                            eprintln!("Error saving state: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error booting runtime: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cortex::cli::Commands::Learn => {
            let path = config_path.clone();
            match cortex::config::CortexConfig::load(&path) {
                Ok(config) => match cortex::cortex::CortexRuntime::boot(config) {
                    Ok(mut runtime) => {
                        let stdin = std::io::stdin();
                        println!("Enter text to learn (Ctrl+D to finish):");
                        let mut input = String::new();
                        match stdin.read_line(&mut input) {
                            Ok(0) => println!("No input provided."),
                            Ok(_) => {
                                let text = input.trim().to_string();
                                if !text.is_empty() {
                                    match runtime.learn(&text) {
                                        Ok(msg) => println!("{}", msg),
                                        Err(e) => {
                                            eprintln!("Error: {}", e);
                                            std::process::exit(1);
                                        }
                                    }
                                } else {
                                    println!("Empty input. No learning applied.");
                                }
                            }
                            Err(e) => {
                                eprintln!("Read error: {}", e);
                                std::process::exit(1);
                            }
                        }
                        if let Err(e) = runtime.save() {
                            eprintln!("Error saving state: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error booting runtime: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cortex::cli::Commands::Query { text, target, max_results } => {
            if let Err(e) = cortex::cli::commands::execute_query(&config_path, &text, &target, max_results) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Verify { claim } => {
            if let Err(e) = cortex::cli::commands::execute_verify(&config_path, &claim) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Checkpoint => {
            if let Err(e) = cortex::cli::commands::execute_checkpoint(&config_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        cortex::cli::Commands::Migrate { dry_run } => {
            if let Err(e) = cortex::cli::commands::execute_migrate(dry_run) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
