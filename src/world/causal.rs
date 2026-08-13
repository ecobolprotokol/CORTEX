use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::common::Timestamp;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalHypothesis {
    pub cause: String,
    pub effect: String,
    pub strength: Scalar,
    pub evidence_count: u32,
    pub false_positive_count: u32,
    pub created_at: Timestamp,
    pub last_observed: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CausalLink {
    pub direction: CausalDirection,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CausalDirection {
    Direct,
    Inverse,
    Correlational,
    Spurious,
}

pub struct CausalModel {
    pub hypotheses: Vec<CausalHypothesis>,
    pub co_occurrence_matrix: HashMap<(String, String), u32>,
    pub temporal_order: Vec<(String, String, Timestamp)>,
}

impl CausalModel {
    pub fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
            co_occurrence_matrix: HashMap::new(),
            temporal_order: Vec::new(),
        }
    }

    pub fn add_hypothesis(&mut self, cause: &str, effect: &str) {
        let exists = self
            .hypotheses
            .iter()
            .any(|h| h.cause == cause && h.effect == effect);
        if !exists {
            self.hypotheses.push(CausalHypothesis {
                cause: cause.to_string(),
                effect: effect.to_string(),
                strength: 0.1,
                evidence_count: 0,
                false_positive_count: 0,
                created_at: Timestamp::now(),
                last_observed: Timestamp::now(),
            });
        }
    }

    pub fn strengthen(&mut self, cause: &str, effect: &str) {
        if let Some(h) = self
            .hypotheses
            .iter_mut()
            .find(|h| h.cause == cause && h.effect == effect)
        {
            h.evidence_count += 1;
            h.strength = (h.strength + 0.1).min(1.0);
            h.last_observed = Timestamp::now();
        }
    }

    pub fn weaken(&mut self, cause: &str, effect: &str) {
        if let Some(h) = self
            .hypotheses
            .iter_mut()
            .find(|h| h.cause == cause && h.effect == effect)
        {
            h.false_positive_count += 1;
            h.strength = (h.strength - 0.05).max(0.01);
        }
    }

    pub fn record_co_occurrence(&mut self, cause: &str, effect: &str) {
        *self
            .co_occurrence_matrix
            .entry((cause.to_string(), effect.to_string()))
            .or_insert(0) += 1;
    }

    pub fn record_temporal_order(&mut self, cause: &str, effect: &str) {
        self.temporal_order
            .push((cause.to_string(), effect.to_string(), Timestamp::now()));
    }

    pub fn classify_link(&self, cause: &str, effect: &str) -> CausalLink {
        let co_occurrences = self
            .co_occurrence_matrix
            .get(&(cause.to_string(), effect.to_string()))
            .copied()
            .unwrap_or(0);

        let temporal_correct = self
            .temporal_order
            .iter()
            .any(|(c, e, _)| c == cause && e == effect);

        let hypothesis = self
            .hypotheses
            .iter()
            .find(|h| h.cause == cause && h.effect == effect);

        let strength = hypothesis.map(|h| h.strength).unwrap_or(0.0);

        let direction = if strength > 0.7 && temporal_correct {
            CausalDirection::Direct
        } else if strength > 0.4 && co_occurrences > 2 {
            CausalDirection::Correlational
        } else if co_occurrences > 0 && !temporal_correct {
            CausalDirection::Spurious
        } else {
            CausalDirection::Correlational
        };

        CausalLink {
            direction,
            confidence: strength,
        }
    }

    pub fn get_strongest_causes(&self, effect: &str) -> Vec<&CausalHypothesis> {
        let mut causes: Vec<&CausalHypothesis> = self
            .hypotheses
            .iter()
            .filter(|h| h.effect == effect)
            .collect();
        causes.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        causes
    }

    pub fn get_strongest_effects(&self, cause: &str) -> Vec<&CausalHypothesis> {
        let mut effects: Vec<&CausalHypothesis> = self
            .hypotheses
            .iter()
            .filter(|h| h.cause == cause)
            .collect();
        effects.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        effects
    }

    pub fn compute_granger_causality(&self, cause: &str, effect: &str) -> Scalar {
        let co_occur = self
            .co_occurrence_matrix
            .get(&(cause.to_string(), effect.to_string()))
            .copied()
            .unwrap_or(0) as Scalar;

        let total_causes = self
            .co_occurrence_matrix
            .iter()
            .filter(|((c, _), _)| c == cause)
            .map(|(_, v)| v)
            .sum::<u32>() as Scalar;

        if total_causes > 0.0 {
            (co_occur / total_causes).min(1.0)
        } else {
            0.0
        }
    }

    pub fn detect_spurious(&self, threshold: Scalar) -> Vec<&CausalHypothesis> {
        self.hypotheses
            .iter()
            .filter(|h| {
                let false_ratio = if h.evidence_count > 0 {
                    h.false_positive_count as Scalar / h.evidence_count as Scalar
                } else {
                    0.0
                };
                false_ratio > threshold || h.strength < 0.1
            })
            .collect()
    }
}

impl Default for CausalModel {
    fn default() -> Self {
        Self::new()
    }
}
