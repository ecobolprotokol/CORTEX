use std::collections::HashMap;

use crate::types::ids::SymbolId;

pub struct Vocabulary {
    pub token_to_id: HashMap<String, SymbolId>,
    pub id_to_token: HashMap<SymbolId, String>,
    pub frequency: HashMap<SymbolId, u64>,
    pub next_id: u32,
    pub capacity: u32,
}

impl Vocabulary {
    pub fn new(capacity: u32) -> Self {
        Self {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            frequency: HashMap::new(),
            next_id: 1,
            capacity,
        }
    }

    pub fn lookup_or_create(&mut self, token: &str) -> SymbolId {
        if let Some(&id) = self.token_to_id.get(token) {
            *self.frequency.entry(id).or_insert(0) += 1;
            return id;
        }

        if self.next_id >= self.capacity {
            return SymbolId::NULL;
        }

        let id = SymbolId::from(self.next_id as u64);
        self.next_id += 1;
        self.token_to_id.insert(token.to_string(), id);
        self.id_to_token.insert(id, token.to_string());
        self.frequency.insert(id, 1);
        id
    }

    pub fn size(&self) -> u32 {
        self.next_id - 1
    }
}
