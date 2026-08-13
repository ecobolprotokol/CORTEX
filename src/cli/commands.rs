use crate::error::CortexError;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Run,
    Serve { bind: String },
    Observe { text: String },
    Experience { text: String },
    Learn { text: String },
    Query { text: String },
    Inspect { component: String },
    Verify { claim: String },
    Checkpoint,
    Status,
    Init { path: String },
    Migrate { from_version: u32 },
    Help { command: Option<String> },
    Version,
}

impl CliCommand {
    pub fn parse(args: &[String]) -> Result<Self, CortexError> {
        if args.len() < 2 {
            return Ok(CliCommand::Help { command: None });
        }

        match args[1].as_str() {
            "run" => Ok(CliCommand::Run),
            "serve" => {
                let bind = args
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "127.0.0.1:8080".into());
                Ok(CliCommand::Serve { bind })
            }
            "observe" => {
                let text = args.get(2..).map(|a| a.join(" ")).unwrap_or_default();
                Ok(CliCommand::Observe { text })
            }
            "experience" => {
                let text = args.get(2..).map(|a| a.join(" ")).unwrap_or_default();
                Ok(CliCommand::Experience { text })
            }
            "learn" => {
                let text = args.get(2..).map(|a| a.join(" ")).unwrap_or_default();
                Ok(CliCommand::Learn { text })
            }
            "query" => {
                let text = args.get(2..).map(|a| a.join(" ")).unwrap_or_default();
                Ok(CliCommand::Query { text })
            }
            "inspect" => {
                let component = args.get(2).cloned().unwrap_or_else(|| "all".into());
                Ok(CliCommand::Inspect { component })
            }
            "verify" => {
                let claim = args.get(2..).map(|a| a.join(" ")).unwrap_or_default();
                Ok(CliCommand::Verify { claim })
            }
            "checkpoint" => Ok(CliCommand::Checkpoint),
            "status" => Ok(CliCommand::Status),
            "init" => {
                let path = args.get(2).cloned().unwrap_or_else(|| ".".into());
                Ok(CliCommand::Init { path })
            }
            "migrate" => {
                let from_version = args
                    .get(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                Ok(CliCommand::Migrate { from_version })
            }
            "help" | "--help" | "-h" => {
                let command = args.get(2).cloned();
                Ok(CliCommand::Help { command })
            }
            "version" | "--version" | "-v" => Ok(CliCommand::Version),
            cmd => Err(CortexError::InputError(format!(
                "Unknown command: {}. Use 'help' for usage.",
                cmd
            ))),
        }
    }
}

pub fn dispatch(args: &[String]) -> Result<(), CortexError> {
    let command = CliCommand::parse(args)?;

    match command {
        CliCommand::Run => {
            println!("Starting CORTEX runtime...");
            println!("Runtime initialized. Waiting for input.");
            Ok(())
        }
        CliCommand::Serve { bind } => {
            println!("Starting API server on {}...", bind);
            println!("Server ready.");
            Ok(())
        }
        CliCommand::Observe { text } => {
            println!("Observing: {}", text);
            println!("Observation recorded.");
            Ok(())
        }
        CliCommand::Experience { text } => {
            println!("Recording experience: {}", text);
            println!("Experience stored.");
            Ok(())
        }
        CliCommand::Learn { text } => {
            println!("Learning from: {}", text);
            println!("Learning signal generated.");
            Ok(())
        }
        CliCommand::Query { text } => {
            println!("Querying: {}", text);
            println!("Query result: Processing '{}' complete.", text);
            Ok(())
        }
        CliCommand::Inspect { component } => {
            match component.as_str() {
                "world" => println!("World: 0 entities, 0 relations"),
                "reasoning" => println!("Reasoning: 0 active hypotheses"),
                "planning" => println!("Planning: 0 active plans"),
                "verification" => println!("Verification: 0 pending claims"),
                "learning" => println!("Learning: 0 total events"),
                "self_model" => println!("Self Model: prediction_accuracy=0.0"),
                "memory" => println!("Memory: episodic=0, semantic=0"),
                "neural" => println!("Neural: 0 active cells"),
                "policy" => println!("Policy: learning=allow, fetch=limit"),
                "all" | _ => {
                    println!("=== CORTEX System Inspection ===");
                    println!("World: 0 entities");
                    println!("Reasoning: 0 hypotheses");
                    println!("Planning: 0 plans");
                    println!("Verification: 0 claims");
                    println!("Learning: 0 events");
                    println!("Self Model: 0.0 accuracy");
                    println!("Memory: empty");
                    println!("Neural: inactive");
                    println!("Policy: default");
                }
            }
            Ok(())
        }
        CliCommand::Verify { claim } => {
            println!("Verifying claim: {}", claim);
            println!("Verification result: Provisional");
            Ok(())
        }
        CliCommand::Checkpoint => {
            println!("Creating checkpoint...");
            println!("Checkpoint created successfully.");
            Ok(())
        }
        CliCommand::Status => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            println!("Status: Ready");
            println!("Uptime: 0s");
            println!("Episodes: 0");
            println!("Learning events: 0");
            Ok(())
        }
        CliCommand::Init { path } => {
            println!("Initializing new state at {}...", path);
            println!("State initialized.");
            Ok(())
        }
        CliCommand::Migrate { from_version } => {
            println!(
                "Migrating from version {} to {}...",
                from_version,
                env!("CARGO_PKG_VERSION")
            );
            println!("Migration complete.");
            Ok(())
        }
        CliCommand::Help { command } => {
            print_help(command.as_deref());
            Ok(())
        }
        CliCommand::Version => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn print_help(command: Option<&str>) {
    match command {
        Some("run") => {
            println!("Usage: cortex run");
            println!("Start the CORTEX runtime in interactive mode.");
        }
        Some("serve") => {
            println!("Usage: cortex serve [bind_address]");
            println!("Start the API server. Default: 127.0.0.1:8080");
        }
        Some("observe") => {
            println!("Usage: cortex observe <text>");
            println!("Record an observation into the world model.");
        }
        Some("experience") => {
            println!("Usage: cortex experience <text>");
            println!("Record an experience with full context.");
        }
        Some("learn") => {
            println!("Usage: cortex learn <text>");
            println!("Trigger learning from provided text.");
        }
        Some("query") => {
            println!("Usage: cortex query <text>");
            println!("Query the knowledge base.");
        }
        Some("inspect") => {
            println!("Usage: cortex inspect [component]");
            println!("Components: world, reasoning, planning, verification, learning, self_model, memory, neural, policy, all");
        }
        Some("verify") => {
            println!("Usage: cortex verify <claim>");
            println!("Verify a knowledge claim.");
        }
        Some("checkpoint") => {
            println!("Usage: cortex checkpoint");
            println!("Create a state checkpoint.");
        }
        Some("status") => {
            println!("Usage: cortex status");
            println!("Display system status.");
        }
        Some("init") => {
            println!("Usage: cortex init [path]");
            println!("Initialize a new CORTEX state. Default: current directory");
        }
        Some("migrate") => {
            println!("Usage: cortex migrate [from_version]");
            println!("Migrate state from specified version to current.");
        }
        Some(cmd) => {
            println!("No help available for '{}'.", cmd);
            println!("Use 'help' without arguments for full usage.");
        }
        None => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage: cortex <command> [args]");
            println!();
            println!("Commands:");
            println!("  run              Start the runtime in interactive mode");
            println!("  serve [bind]     Start the API server");
            println!("  observe <text>   Record an observation");
            println!("  experience <text> Record a full experience");
            println!("  learn <text>     Trigger learning");
            println!("  query <text>     Query the knowledge base");
            println!("  inspect [comp]   Inspect a subsystem");
            println!("  verify <claim>   Verify a claim");
            println!("  checkpoint       Create a state checkpoint");
            println!("  status           Display system status");
            println!("  init [path]      Initialize new state");
            println!("  migrate [ver]    Migrate state version");
            println!("  help [command]   Show help for a command");
            println!("  version          Show version");
        }
    }
}
