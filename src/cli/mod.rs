pub mod commands;

use crate::error::CortexError;

pub fn main() -> Result<(), CortexError> {
    let args: Vec<String> = std::env::args().collect();
    commands::dispatch(&args)
}
