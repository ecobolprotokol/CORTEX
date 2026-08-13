//! CORTEX – Entry point and CLI dispatch.

use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = CortexConfig::load("cortex.toml").unwrap_or_else(|_| {
        eprintln!("No cortex.toml found, using defaults");
        CortexConfig::default()
    });

    let mut runtime = CortexRuntime::new(config)?;
    runtime.boot()?;

    println!("CORTEX v{} | Ready", env!("CARGO_PKG_VERSION"));

    runtime.run()?;

    Ok(())
}
