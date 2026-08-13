use crate::error::CortexError;
use crate::runtime::Runtime;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Run { config: String },
    Serve { bind: String, config: String },
    Observe { text: String, config: String },
    Query { text: String, config: String },
    Status { config: String },
    Init { config: String },
    Checkpoint { config: String },
    Inspect { config: String, component: Option<String> },
    Migrate { config: String },
    Help,
    Version,
}

impl CliCommand {
    pub fn parse(args: &[String]) -> Result<Self, CortexError> {
        if args.len() < 2 {
            return Ok(CliCommand::Help);
        }

        let config = args
            .iter()
            .position(|a| a == "--config")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "cortex.toml".into());

        match args[1].as_str() {
            "run" => Ok(CliCommand::Run { config }),
            "serve" => {
                let bind = args
                    .iter()
                    .position(|a| a == "--bind")
                    .and_then(|i| args.get(i + 1))
                    .cloned()
                    .unwrap_or_else(|| "127.0.0.1:8080".into());
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
            "inspect" => {
                let component = args.get(2).cloned();
                Ok(CliCommand::Inspect { config, component })
            }
            "migrate" => Ok(CliCommand::Migrate { config }),
            "help" | "--help" | "-h" => Ok(CliCommand::Help),
            "version" | "--version" | "-v" => Ok(CliCommand::Version),
            cmd => Err(CortexError::InputError(format!("Unknown command: {}", cmd))),
        }
    }
}

pub fn dispatch(args: &[String]) -> Result<String, CortexError> {
    let cmd = CliCommand::parse(args)?;
    match cmd {
        CliCommand::Help => Ok(
            "CORTEX — A persistent, state-based, continually learning AI model\n\n\
                Usage: cortex <command> [options]\n\n\
                Commands:\n  \
                  run          Start the interactive runtime\n  \
                  serve        Start the API server\n  \
                  observe      Process an observation\n  \
                  query        Query the system\n  \
                  status       Show system status\n  \
                  init         Initialize state\n  \
                  checkpoint   Create a checkpoint\n  \
                  inspect      Inspect component state\n  \
                  migrate      Check migration status\n  \
                  help         Show this help\n  \
                  version      Show version\n\n\
                Options:\n  \
                  --config <path>   Config file (default: cortex.toml)\n  \
                  --bind <addr>     API bind address (serve only)"
                .to_string(),
        ),
        CliCommand::Version => Ok(format!("CORTEX v{}", env!("CARGO_PKG_VERSION"))),
        CliCommand::Observe { text, config } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            let response = rt.process(&text)?;
            rt.shutdown()?;
            Ok(response)
        }
        CliCommand::Query { text, config } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            let response = rt.process(&text)?;
            rt.shutdown()?;
            Ok(response)
        }
        CliCommand::Status { config } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            let status = format!(
                "CORTEX v{}\nState: {:?}\nEpisodes: {}\nLearning events: {}\nVocabulary: {}",
                env!("CARGO_PKG_VERSION"),
                rt.runtime_state,
                rt.state.metadata.episode_count,
                rt.state.learning.total_learning_events,
                rt.language_vocabulary.size(),
            );
            rt.shutdown()?;
            Ok(status)
        }
        CliCommand::Init { .. } => {
            let cfg = crate::config::CortexConfig::default();
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            rt.shutdown()?;
            Ok("State initialized".into())
        }
        CliCommand::Checkpoint { config } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            let _ = rt.save_state();
            rt.shutdown()?;
            Ok("Checkpoint created".into())
        }
        CliCommand::Inspect { config, component } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let mut rt = crate::cortex::CortexRuntime::new(cfg)?;
            rt.boot()?;
            let result = crate::api::handlers::handle_inspect_with_runtime(
                &rt,
                component.as_deref().unwrap_or("all"),
            );
            rt.shutdown()?;
            result
        }
        CliCommand::Migrate { config } => {
            let cfg = crate::config::CortexConfig::load(&config)
                .unwrap_or_else(|_| crate::config::CortexConfig::default());
            let handler = crate::persistence::migration::MigrationHandler::new();
            let state_path = &cfg.persistence.state;
            if std::path::Path::new(state_path).exists() {
                let data = std::fs::read(state_path).map_err(|e| {
                    CortexError::PersistenceError(format!("Failed to read state file: {}", e))
                })?;
                let current_version = handler.detect_version(&data).unwrap_or(0);
                let available = handler.available_versions();
                let latest = *available.last().unwrap_or(&1);
                Ok(format!(
                    "Migration check:\n  Current version: {}\n  Available versions: {:?}\n  Status: {}",
                    current_version,
                    available,
                    if current_version >= latest {
                        "Up to date"
                    } else {
                        "Migration available"
                    }
                ))
            } else {
                Ok("No state file found. Run 'cortex init' first.".into())
            }
        }
        CliCommand::Run { .. } | CliCommand::Serve { .. } => Err(CortexError::RuntimeError(
            "Use the cortex binary for interactive run/serve modes".into(),
        )),
    }
}
