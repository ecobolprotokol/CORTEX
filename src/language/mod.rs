pub mod tokenizer;
pub mod vocabulary;
pub mod syntax;
pub mod semantics;
pub mod language_model;
pub mod decoder;
pub mod context;

use crate::config::LanguageConfig;
use crate::error::Result;
use crate::types::*;

pub trait LanguageCore {
    fn encode(&mut self, text: &str, context: &ContextState) -> Result<LanguageState>;
    fn generate(&mut self, verified: &VerifiedResult) -> Result<GeneratedResponse>;
    fn predict(&self, state: &LanguageState) -> Result<Vec<CandidateContinuation>>;
    fn vocabulary_size(&self) -> u32;
    fn state(&self) -> &LanguageState;
}

pub struct LanguageCoreImpl {
    config: LanguageConfig,
    state: LanguageState,
}

impl LanguageCoreImpl {
    pub fn new(config: &LanguageConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: LanguageState {
                symbols: Vec::new(),
                tokens: Vec::new(),
                vocabulary_size: 0,
                next_symbol_id: SymbolId(1),
            },
        })
    }
}

impl LanguageCore for LanguageCoreImpl {
    fn encode(&mut self, text: &str, _context: &ContextState) -> Result<LanguageState> {
        let symbols = tokenizer::tokenize(text, &mut self.state)?;
        let syntax_analysis = syntax::analyze(&symbols);
        let semantic_analysis = semantics::extract(&symbols, &syntax_analysis);

        let tokens: Vec<Token> = symbols.iter().enumerate().map(|(i, s)| {
            Token {
                id: TokenId(i as u64),
                symbol_id: s.id,
                position: i as u32,
                weight: s.activation,
            }
        }).collect();

        let vocabulary_size = self.state.vocabulary_size;
        Ok(LanguageState {
            symbols,
            tokens,
            vocabulary_size,
            next_symbol_id: self.state.next_symbol_id,
        })
    }

    fn generate(&mut self, verified: &VerifiedResult) -> Result<GeneratedResponse> {
        decoder::generate(verified)
    }

    fn predict(&self, state: &LanguageState) -> Result<Vec<CandidateContinuation>> {
        language_model::predict(state, &self.config)
    }

    fn vocabulary_size(&self) -> u32 {
        self.state.vocabulary_size
    }

    fn state(&self) -> &LanguageState {
        &self.state
    }
}
