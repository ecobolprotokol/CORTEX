pub mod context;
pub mod decoder;
pub mod language_model;
pub mod semantics;
pub mod syntax;
pub mod tokenizer;
pub mod vocabulary;

use crate::error::CortexError;
use crate::types::common::ContextState;
use crate::types::state::LanguageState;

pub trait LanguageCore {
    fn encode(&mut self, text: &str, context: &ContextState) -> Result<LanguageState, CortexError>;
    fn generate(&self, state: &LanguageState) -> Result<String, CortexError>;
    fn vocabulary_size(&self) -> u32;
}
