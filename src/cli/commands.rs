use crate::error::CortexError;

pub fn dispatch(args: &[String]) -> Result<(), CortexError> {
    if args.len() < 2 {
        println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match args[1].as_str() {
        "run" => {
            println!("Starting CORTEX runtime...");
            Ok(())
        }
        "serve" => {
            println!("Starting API server...");
            Ok(())
        }
        "observe" => {
            let text = args.get(2).map(|s| s.as_str()).unwrap_or("");
            println!("Observing: {}", text);
            Ok(())
        }
        "query" => {
            let text = args.get(2).map(|s| s.as_str()).unwrap_or("");
            println!("Query: {}", text);
            Ok(())
        }
        "status" => {
            println!("Status: Ready");
            Ok(())
        }
        "init" => {
            println!("Initializing new state...");
            Ok(())
        }
        "checkpoint" => {
            println!("Creating checkpoint...");
            Ok(())
        }
        "help" | "--help" | "-h" => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            println!("Commands: run, serve, observe, query, status, init, checkpoint, help");
            Ok(())
        }
        "version" | "--version" | "-v" => {
            println!("CORTEX v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        cmd => {
            Err(CortexError::InputError(format!("Unknown command: {}", cmd)))
        }
    }
}
