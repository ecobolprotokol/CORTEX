use crate::types::*;

pub struct Vocabulary {
    pub symbols: Vec<Symbol>,
    pub capacity: u32,
}

impl Vocabulary {
    pub fn new(capacity: u32) -> Self {
        Self {
            symbols: Vec::new(),
            capacity,
        }
    }

    pub fn lookup(&self, text: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.text == text)
    }

    pub fn size(&self) -> u32 {
        self.symbols.len() as u32
    }
}
