use crate::types::*;

pub fn evaluate_evidence(set: &EvidenceSet) -> Scalar {
    if set.is_empty() {
        return 0.0;
    }
    let supporting = set.supporting();
    let contradicting = set.contradicting();
    let support_strength: Scalar = supporting.iter().map(|e| e.strength).sum();
    let contra_strength: Scalar = contradicting.iter().map(|e| e.strength).sum();
    (support_strength - contra_strength).max(0.0).min(1.0)
}
