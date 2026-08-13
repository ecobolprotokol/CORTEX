//! CORTEX – Entry point and CLI dispatch.

pub mod config;
pub mod error;
pub mod types;
pub mod cortex;
pub mod runtime;
pub mod language;
pub mod neural;
pub mod memory;
pub mod world;
pub mod reasoning;
pub mod planning;
pub mod verification;
pub mod learning;
pub mod self_model;
pub mod policy;
pub mod internet;
pub mod persistence;
pub mod api;
pub mod cli;
pub mod observability;

use config::CortexConfig;
use runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = CortexConfig::load("cortex.toml").unwrap_or_else(|_| {
        eprintln!("No cortex.toml found, using defaults");
        CortexConfig::default()
    });

    let mut runtime = cortex::CortexRuntime::new(config)?;
    runtime.boot()?;

    println!("CORTEX v{} | Ready", env!("CARGO_PKG_VERSION"));

    runtime.run()?;

    Ok(())
}
