use crate::types::scalars::Scalar;
use crate::types::evidence::Evidence;

pub struct EvidenceEvaluator;

impl EvidenceEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate_evidence_quality(evidence: &[Evidence]) -> Scalar {
        if evidence.is_empty() {
            return 0.0;
        }

        let total_strength: Scalar = evidence.iter().map(|e| e.strength).sum();
        let avg_strength = total_strength / evidence.len() as Scalar;

        let count_bonus = (evidence.len() as Scalar).min(5.0) / 5.0;

        (avg_strength * 0.7 + count_bonus * 0.3).min(1.0)
    }

    pub fn gather_supporting(evidence: &[Evidence]) -> Vec<&Evidence> {
        evidence
            .iter()
            .filter(|e| e.polarity == crate::types::evidence::EvidencePolarity::Supports)
            .collect()
    }

    pub fn gather_counter(evidence: &[Evidence]) -> Vec<&Evidence> {
        evidence
            .iter()
            .filter(|e| e.polarity == crate::types::evidence::EvidencePolarity::Contradicts)
            .collect()
    }
}
