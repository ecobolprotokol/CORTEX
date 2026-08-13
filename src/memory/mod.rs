pub mod associative;
pub mod consolidation;
pub mod episodic;
pub mod procedural;
pub mod retrieval;
pub mod semantic;
pub mod working;

use std::collections::HashMap;

use crate::types::observation::Observation;

pub struct MemoryRetrieval {
    pub episodes: Vec<Observation>,
    pub knowledge: Vec<String>,
    pub procedures: Vec<String>,
    pub associations: Vec<(u64, u64, String)>,
    pub relevance_scores: HashMap<u64, f32>,
}

impl MemoryRetrieval {
    pub fn empty() -> Self {
        Self {
            episodes: Vec::new(),
            knowledge: Vec::new(),
            procedures: Vec::new(),
            associations: Vec::new(),
            relevance_scores: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
            && self.knowledge.is_empty()
            && self.procedures.is_empty()
            && self.associations.is_empty()
    }

    pub fn total_results(&self) -> usize {
        self.episodes.len() + self.knowledge.len() + self.procedures.len() + self.associations.len()
    }
}
