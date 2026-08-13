use crate::types::scalars::Scalar;
use crate::types::evidence::{Evidence, EvidencePolarity, EvidenceSet, EvidenceContent};
use crate::types::common::Timestamp;

#[derive(Debug, Clone)]
pub struct EvidenceQuality {
    pub overall: Scalar,
    pub strength_score: Scalar,
    pub diversity_score: Scalar,
    pub recency_score: Scalar,
    pub source_reliability: Scalar,
}

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

        let polarity_diversity = {
            let supports = evidence
                .iter()
                .filter(|e| e.polarity == EvidencePolarity::Supports)
                .count();
            let contradicts = evidence
                .iter()
                .filter(|e| e.polarity == EvidencePolarity::Contradicts)
                .count();
            if supports > 0 && contradicts > 0 {
                0.3
            } else {
                0.0
            }
        };

        (avg_strength * 0.5 + count_bonus * 0.2 + polarity_diversity + 0.0)
            .min(1.0)
    }

    pub fn gather_supporting(evidence: &[Evidence]) -> Vec<&Evidence> {
        evidence
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Supports)
            .collect()
    }

    pub fn gather_counter(evidence: &[Evidence]) -> Vec<&Evidence> {
        evidence
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Contradicts)
            .collect()
    }

    pub fn compute_evidence_set_quality(set: &EvidenceSet) -> EvidenceQuality {
        let items = &set.items;
        let strength_score = if items.is_empty() {
            0.0
        } else {
            items.iter().map(|e| e.strength).sum::<Scalar>() / items.len() as Scalar
        };

        let mut polarities = std::collections::HashSet::new();
        for e in items {
            polarities.insert(e.polarity);
        }
        let diversity_score = polarities.len() as Scalar / 3.0;

        let now = Timestamp::now();
        let recency_score = if items.is_empty() {
            0.0
        } else {
            let avg_age: Scalar = items
                .iter()
                .map(|e| {
                    let age_ms = now.0.saturating_sub(e.timestamp.0);
                    (age_ms as Scalar / 3600000.0).min(24.0)
                })
                .sum::<Scalar>()
                / items.len() as Scalar;
            (1.0 - avg_age / 24.0).max(0.0)
        };

        let source_reliability = if items.is_empty() {
            0.0
        } else {
            0.5
        };

        let overall = strength_score * 0.35
            + diversity_score * 0.2
            + recency_score * 0.2
            + source_reliability * 0.25;

        EvidenceQuality {
            overall: overall.min(1.0),
            strength_score,
            diversity_score,
            recency_score,
            source_reliability,
        }
    }

    pub fn merge_evidence_sets(a: &EvidenceSet, b: &EvidenceSet) -> EvidenceSet {
        let mut merged = a.clone();
        for item in &b.items {
            if !merged.items.iter().any(|e| e.id == item.id) {
                merged.items.push(item.clone());
            }
        }
        merged
    }

    pub fn filter_by_strength(evidence: &[Evidence], min_strength: Scalar) -> Vec<&Evidence> {
        evidence
            .iter()
            .filter(|e| e.strength >= min_strength)
            .collect()
    }

    pub fn weighted_average(evidence: &[Evidence]) -> Scalar {
        if evidence.is_empty() {
            return 0.0;
        }
        let total_weight: Scalar = evidence.iter().map(|e| e.strength).sum();
        if total_weight < crate::types::scalars::SCALAR_EPSILON {
            return 0.0;
        }
        evidence
            .iter()
            .map(|e| e.strength * e.strength)
            .sum::<Scalar>()
            / total_weight
    }

    pub fn detect_inconsistencies(evidence: &[Evidence]) -> Vec<(usize, usize, Scalar)> {
        let mut inconsistencies = Vec::new();
        for i in 0..evidence.len() {
            for j in (i + 1)..evidence.len() {
                let conflict = match (evidence[i].polarity, evidence[j].polarity) {
                    (EvidencePolarity::Supports, EvidencePolarity::Contradicts)
                    | (EvidencePolarity::Contradicts, EvidencePolarity::Supports) => {
                        (evidence[i].strength - evidence[j].strength).abs()
                    }
                    _ => 0.0,
                };
                if conflict > 0.1 {
                    inconsistencies.push((i, j, conflict));
                }
            }
        }
        inconsistencies
    }
}

impl Default for EvidenceEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
