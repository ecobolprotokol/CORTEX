use serde::{Deserialize, Serialize};

use crate::types::common::Timestamp;
use crate::types::evidence::ConfidenceState;
use crate::types::ids::KnowledgeId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: KnowledgeId,
    pub concept: String,
    pub properties: Vec<(String, String)>,
    pub relations: Vec<KnowledgeRelation>,
    pub confidence: ConfidenceState,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub confirmation_count: u64,
    pub contradiction_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    pub target_concept: String,
    pub relation_type: String,
    pub strength: Scalar,
}

#[derive(Debug, Clone)]
pub struct SemanticMemory {
    pub knowledge: Vec<Knowledge>,
    pub capacity: usize,
    pub next_id: u64,
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self::new(512)
    }
}

impl SemanticMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            knowledge: Vec::new(),
            capacity,
            next_id: 1,
        }
    }

    pub fn store(
        &mut self,
        concept: &str,
        properties: Vec<(String, String)>,
    ) -> Knowledge {
        let k = Knowledge {
            id: KnowledgeId::from(self.next_id),
            concept: concept.to_string(),
            properties,
            relations: Vec::new(),
            confidence: ConfidenceState::default(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        };
        self.next_id += 1;

        if self.knowledge.len() >= self.capacity {
            self.evict_lowest_confidence();
        }

        self.knowledge.push(k.clone());
        k
    }

    pub fn store_with_relations(
        &mut self,
        concept: &str,
        properties: Vec<(String, String)>,
        relations: Vec<KnowledgeRelation>,
    ) -> Knowledge {
        let k = Knowledge {
            id: KnowledgeId::from(self.next_id),
            concept: concept.to_string(),
            properties,
            relations,
            confidence: ConfidenceState::default(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        };
        self.next_id += 1;

        if self.knowledge.len() >= self.capacity {
            self.evict_lowest_confidence();
        }

        self.knowledge.push(k.clone());
        k
    }

    pub fn find_by_concept(&self, concept: &str) -> Vec<&Knowledge> {
        self.knowledge
            .iter()
            .filter(|k| k.concept == concept)
            .collect()
    }

    pub fn find_by_concept_mut(&mut self, concept: &str) -> Vec<&mut Knowledge> {
        self.knowledge
            .iter_mut()
            .filter(|k| k.concept == concept)
            .collect()
    }

    pub fn update_confidence(&mut self, id: KnowledgeId, belief_delta: Scalar) {
        if let Some(k) = self.knowledge.iter_mut().find(|k| k.id == id) {
            k.confidence.belief = (k.confidence.belief + belief_delta).clamp(0.0, 1.0);
            k.confidence.uncertainty =
                (k.confidence.uncertainty - belief_delta.abs() * 0.5).clamp(0.0, 1.0);
            k.updated_at = Timestamp::now();

            if belief_delta > 0.0 {
                k.confirmation_count += 1;
            } else if belief_delta < 0.0 {
                k.contradiction_count += 1;
            }
        }
    }

    pub fn confirm(&mut self, id: KnowledgeId) {
        self.update_confidence(id, 0.05);
    }

    pub fn contradict(&mut self, id: KnowledgeId) {
        self.update_confidence(id, -0.10);
    }

    pub fn get(&self, id: KnowledgeId) -> Option<&Knowledge> {
        self.knowledge.iter().find(|k| k.id == id)
    }

    fn evict_lowest_confidence(&mut self) {
        if let Some(pos) = self
            .knowledge
            .iter()
            .enumerate()
            .min_by(|a, b| {
                a.1.confidence
                    .overall()
                    .partial_cmp(&b.1.confidence.overall())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
        {
            self.knowledge.remove(pos);
        }
    }

    pub fn all_concepts(&self) -> Vec<&str> {
        self.knowledge.iter().map(|k| k.concept.as_str()).collect()
    }

    pub fn usage_bytes(&self) -> usize {
        self.knowledge.len() * std::mem::size_of::<Knowledge>()
    }

    pub fn is_full(&self) -> bool {
        self.knowledge.len() >= self.capacity
    }
}
