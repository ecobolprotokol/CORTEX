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
        CliCommand::Run { .. } | CliCommand::Serve { .. } => Err(CortexError::RuntimeError(
            "Use the cortex binary for interactive run/serve modes".into(),
        )),
    }
}
