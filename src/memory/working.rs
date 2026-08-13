use crate::types::common::Timestamp;
use crate::types::ids::{ConceptId, GoalId, HypothesisId};

#[derive(Debug, Clone)]
pub struct WorkingMemory {
    pub active_concepts: Vec<ConceptId>,
    pub active_hypotheses: Vec<HypothesisId>,
    pub goals: Vec<GoalId>,
    pub input: Option<String>,
    max_size: usize,
    pub created_at: Timestamp,
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self::new(128)
    }
}

impl WorkingMemory {
    pub fn new(max_size: usize) -> Self {
        Self {
            active_concepts: Vec::new(),
            active_hypotheses: Vec::new(),
            goals: Vec::new(),
            input: None,
            max_size,
            created_at: Timestamp::now(),
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

    pub fn add_hypothesis(&mut self, hypothesis: HypothesisId) {
        if self.active_hypotheses.len() >= self.max_size {
            self.active_hypotheses.remove(0);
        }
        self.active_hypotheses.push(hypothesis);
    }

    pub fn add_goal(&mut self, goal: GoalId) {
        if self.goals.len() >= self.max_size {
            self.goals.remove(0);
        }
        self.goals.push(goal);
    }

    pub fn clear(&mut self) {
        self.active_concepts.clear();
        self.active_hypotheses.clear();
        self.goals.clear();
        self.input = None;
    }

    pub fn advance_time(&mut self) {
        if self.active_concepts.len() > self.max_size / 2 {
            let drain_count = self.active_concepts.len() - self.max_size / 4;
            self.active_concepts.drain(..drain_count);
        }
        if self.active_hypotheses.len() > self.max_size / 2 {
            let drain_count = self.active_hypotheses.len() - self.max_size / 4;
            self.active_hypotheses.drain(..drain_count);
        }
    }

    pub fn active_count(&self) -> usize {
        self.active_concepts.len() + self.active_hypotheses.len() + self.goals.len()
    }

    pub fn is_full(&self) -> bool {
        self.active_count() >= self.max_size
    }

    pub fn usage_bytes(&self) -> usize {
        let input_bytes = self.input.as_ref().map_or(0, |s| s.len());
        let concepts_bytes = self.active_concepts.len() * std::mem::size_of::<ConceptId>();
        let hypotheses_bytes = self.active_hypotheses.len() * std::mem::size_of::<HypothesisId>();
        let goals_bytes = self.goals.len() * std::mem::size_of::<GoalId>();
        input_bytes + concepts_bytes + hypotheses_bytes + goals_bytes
    }
}
