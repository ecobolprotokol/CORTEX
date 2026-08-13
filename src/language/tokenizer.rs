use crate::error::CortexError;

pub struct Tokenizer {
    pub tokens: Vec<String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn tokenize(&mut self, text: &str) -> Result<Vec<String>, CortexError> {
        let tokens: Vec<String> = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        self.tokens = tokens.clone();
        Ok(tokens)
    }
}
