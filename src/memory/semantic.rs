use serde::{Deserialize, Serialize};
use crate::types::ids::KnowledgeId;
use crate::types::common::Timestamp;
use crate::types::evidence::ConfidenceState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: KnowledgeId,
    pub concept: String,
    pub properties: Vec<(String, String)>,
    pub confidence: ConfidenceState,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub confirmation_count: u64,
    pub contradiction_count: u64,
}

#[derive(Debug, Clone)]
pub struct SemanticMemory {
    pub knowledge: Vec<Knowledge>,
    pub capacity: usize,
    pub next_id: u64,
}

impl SemanticMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            knowledge: Vec::new(),
            capacity,
            next_id: 1,
        }
    }

    pub fn store(&mut self, concept: &str, properties: Vec<(String, String)>) -> Knowledge {
        let k = Knowledge {
            id: KnowledgeId::from(self.next_id),
            concept: concept.to_string(),
            properties,
            confidence: ConfidenceState::default(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        };
        self.next_id += 1;
        self.knowledge.push(k.clone());
        k
    }

    pub fn find_by_concept(&self, concept: &str) -> Vec<&Knowledge> {
        self.knowledge.iter()
            .filter(|k| k.concept == concept)
            .collect()
    }
}
