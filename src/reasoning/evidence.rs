use crate::types::*;

pub fn evaluate_evidence(set: &EvidenceSet) -> Scalar {
    if set.is_empty() {
        return 0.0;
    }
    let supporting = set.supporting();
    let contradicting = set.contradicting();
    let support_strength: Scalar = supporting.iter().map(|e| e.strength).sum();
    let contra_strength: Scalar = contradicting.iter().map(|e| e.strength).sum();
    let total = support_strength + contra_strength;
    if total == 0.0 {
        0.0
    } else {
        (support_strength - contra_strength).abs() / total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_empty() {
        let set = EvidenceSet::new();
        assert_eq!(evaluate_evidence(&set), 0.0);
    }

    #[test]
    fn test_evidence_supporting() {
        let mut set = EvidenceSet::new();
        set.add(Evidence {
            id: EvidenceId(1),
            source: Provenance::user_provided(),
            content: EvidenceContent::Text("test".into()),
            strength: 0.8,
            polarity: EvidencePolarity::Supports,
            timestamp: Timestamp::now(),
            related: Vec::new(),
        });
        let score = evaluate_evidence(&set);
        assert!(score > 0.0);
    }
}
