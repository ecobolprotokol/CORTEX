pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cortex", version, about = "CORTEX - A persistent, state-based, continually learning AI model")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(long)]
        config: Option<String>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        quiet: bool,
    },
    Serve {
        #[arg(long)]
        config: Option<String>,
        #[arg(long)]
        bind: Option<String>,
    },
    Observe {
        text: String,
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        importance: Option<f32>,
    },
    Experience {
        json_data: String,
    },
    Learn,
    Query {
        text: String,
        #[arg(long, default_value = "memory")]
        target: String,
        #[arg(long, default_value = "10")]
        max_results: u32,
    },
    Inspect {
        section: Option<String>,
    },
    Verify {
        claim: String,
    },
    Checkpoint,
    Status,
    Init {
        #[arg(long)]
        force: bool,
    },
    Migrate {
        #[arg(long)]
        dry_run: bool,
    },
    Version,
}
