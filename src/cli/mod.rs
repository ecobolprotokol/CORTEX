pub mod commands;

pub use commands::{dispatch, CliCommand};

use crate::error::CortexError;

pub fn main() -> Result<(), CortexError> {
    let args: Vec<String> = std::env::args().collect();
    commands::dispatch(&args)
}
