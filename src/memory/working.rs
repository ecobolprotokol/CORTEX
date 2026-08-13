use crate::types::ids::{ConceptId, HypothesisId, GoalId};

#[derive(Debug, Clone, Default)]
pub struct WorkingMemory {
    pub active_concepts: Vec<ConceptId>,
    pub active_hypotheses: Vec<HypothesisId>,
    pub goals: Vec<GoalId>,
    pub input: Option<String>,
    pub max_size: usize,
}

impl WorkingMemory {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            ..Default::default()
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }

    pub fn add_concept(&mut self, concept: ConceptId) {
        if self.active_concepts.len() >= self.max_size {
            self.active_concepts.remove(0);
        }
        self.active_concepts.push(concept);
    }

    pub fn clear(&mut self) {
        self.active_concepts.clear();
        self.active_hypotheses.clear();
        self.goals.clear();
        self.input = None;
    }
}
