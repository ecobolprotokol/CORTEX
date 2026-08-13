use crate::error::CortexError;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Run { config: String },
    Serve { bind: String, config: String },
    Observe { text: String, config: String },
    Query { text: String, config: String },
    Status { config: String },
    Init { config: String },
    Checkpoint { config: String },
    Migrate { config: String },
    Help,
    Version,
}

impl CliCommand {
    pub fn parse(args: &[String]) -> Result<Self, CortexError> {
        if args.len() < 2 {
            return Ok(CliCommand::Help);
        }

        let config = args.iter().position(|a| a == "--config")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "cortex.toml".into());

        match args[1].as_str() {
            "run" => Ok(CliCommand::Run { config }),
            "serve" => {
                let bind = args.iter().position(|a| a == "--bind")
                    .and_then(|i| args.get(i + 1))
                    .cloned()
                    .unwrap_or_else(|| "127.0.0.1:8080".to_string());
                Ok(CliCommand::Serve { bind, config })
            }
            "observe" => {
                let text = args.get(2).cloned().unwrap_or_default();
                Ok(CliCommand::Observe { text, config })
            }
            "query" => {
                let text = args.get(2).cloned().unwrap_or_default();
                Ok(CliCommand::Query { text, config })
            }
            "status" => Ok(CliCommand::Status { config }),
            "init" => Ok(CliCommand::Init { config }),
            "checkpoint" => Ok(CliCommand::Checkpoint { config }),
            "migrate" => Ok(CliCommand::Migrate { config }),
            "help" | "--help" | "-h" => Ok(CliCommand::Help),
            "version" | "--version" | "-v" => Ok(CliCommand::Version),
            cmd => Err(CortexError::InputError(format!("Unknown command: {}", cmd))),
        }
    }
}

pub fn dispatch(args: &[String]) -> Result<(), CortexError> {
    let cmd = CliCommand::parse(args)?;
    match cmd {
        CliCommand::Help => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            println!("Usage: cortex <command> [options]");
            println!();
            println!("Commands:");
            println!("  run          Start the runtime");
            println!("  serve        Start the API server");
            println!("  observe      Process an observation");
            println!("  query        Query the system");
            println!("  status       Show system status");
            println!("  init         Initialize state");
            println!("  checkpoint   Create a checkpoint");
            println!("  migrate      Run migrations");
            println!("  help         Show this help");
            println!("  version      Show version");
            println!();
            println!("Options:");
            println!("  --config <path>   Config file (default: cortex.toml)");
            println!("  --bind <addr>     API bind address (serve only)");
            Ok(())
        }
        CliCommand::Version => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => {
            Err(CortexError::RuntimeError(
                "Programmatic dispatch not supported via lib; use cortex::cortex::CortexRuntime directly".into()
            ))
        }
    }
}
