use crate::types::ids::ConceptId;

pub struct ContextModel {
    pub active_concepts: Vec<ConceptId>,
    pub window_size: u32,
}

impl ContextModel {
    pub fn new(window_size: u32) -> Self {
        Self {
            active_concepts: Vec::new(),
            window_size,
        }
    }

    pub fn add_concept(&mut self, concept: ConceptId) {
        if self.active_concepts.len() >= self.window_size as usize {
            self.active_concepts.remove(0);
        }
        self.active_concepts.push(concept);
    }
}
