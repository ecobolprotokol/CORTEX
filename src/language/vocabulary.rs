use std::collections::HashMap;

use crate::types::ids::SymbolId;

pub struct Vocabulary {
    pub token_to_id: HashMap<String, SymbolId>,
    pub id_to_token: HashMap<SymbolId, String>,
    pub frequency: HashMap<SymbolId, u64>,
    pub associations: HashMap<SymbolId, Vec<SymbolId>>,
    pub next_id: u32,
    pub capacity: u32,
}

impl Vocabulary {
    pub fn new(capacity: u32) -> Self {
        Self {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            frequency: HashMap::new(),
            associations: HashMap::new(),
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

    pub fn frequency(&self, id: SymbolId) -> u64 {
        self.frequency.get(&id).copied().unwrap_or(0)
    }

    pub fn associations(&self, id: SymbolId) -> Vec<SymbolId> {
        self.associations.get(&id).cloned().unwrap_or_default()
    }

    pub fn add_association(&mut self, source: SymbolId, target: SymbolId) {
        self.associations
            .entry(source)
            .or_default()
            .push(target);
        self.associations
            .entry(target)
            .or_default()
            .push(source);
    }

    pub fn confidence(&self, id: SymbolId) -> f32 {
        let freq = self.frequency(id);
        let total: u64 = self.frequency.values().sum();
        if total == 0 {
            return 0.0;
        }
        let base = freq as f32 / total as f32;
        (base * 10.0).min(1.0)
    }

    pub fn id_for(&self, token: &str) -> Option<SymbolId> {
        self.token_to_id.get(token).copied()
    }

    pub fn token_for(&self, id: SymbolId) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }
}
