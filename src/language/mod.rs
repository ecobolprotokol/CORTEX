pub mod tokenizer;
pub mod vocabulary;
pub mod syntax;
pub mod semantics;
pub mod language_model;
pub mod decoder;
pub mod context;

use crate::error::CortexError;
use crate::types::state::LanguageState;

pub trait LanguageCore {
    fn encode(&mut self, text: &str) -> Result<LanguageState, CortexError>;
    fn generate(&self, state: &LanguageState) -> Result<String, CortexError>;
    fn vocabulary_size(&self) -> u32;
}
