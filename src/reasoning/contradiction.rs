use crate::types::*;

pub fn detect(hypotheses: &[Hypothesis]) -> Vec<Contradiction> {
    let mut contradictions = Vec::new();
    for i in 0..hypotheses.len() {
        for j in (i + 1)..hypotheses.len() {
            let a = &hypotheses[i];
            let b = &hypotheses[j];
            if propositions_contradict(&a.proposition, &b.proposition) {
                contradictions.push(Contradiction {
                    claim_a: a.id,
                    claim_b: b.id,
                    description: format!(
                        "Contradiction between '{}' and '{}'",
                        a.proposition.predicate, b.proposition.predicate
                    ),
                    severity: 0.5,
                    detected_at: Timestamp::now(),
                    resolved: false,
                });
            }
        }
    }
    contradictions
}

fn propositions_contradict(a: &Proposition, b: &Proposition) -> bool {
    a.subject == b.subject && a.predicate != b.predicate && (a.negated != b.negated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_contradictions_empty() {
        let result = detect(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_no_contradictions_different_subjects() {
        let hypotheses = vec![
            Hypothesis {
                id: HypothesisId(1),
                proposition: Proposition {
                    subject: InternalId::Concept(ConceptId(1)),
                    predicate: "is blue".into(),
                    object: None,
                    modifiers: Vec::new(),
                    negated: false,
                },
                evidence: EvidenceSet::new(),
                counter_evidence: EvidenceSet::new(),
                confidence: 0.8,
                dependencies: Vec::new(),
                contradictions: Vec::new(),
                provenance: Vec::new(),
                reasoning_type: ReasoningType::Inductive,
                created_at: Timestamp::now(),
            },
            Hypothesis {
                id: HypothesisId(2),
                proposition: Proposition {
                    subject: InternalId::Concept(ConceptId(2)),
                    predicate: "is red".into(),
                    object: None,
                    modifiers: Vec::new(),
                    negated: false,
                },
                evidence: EvidenceSet::new(),
                counter_evidence: EvidenceSet::new(),
                confidence: 0.7,
                dependencies: Vec::new(),
                contradictions: Vec::new(),
                provenance: Vec::new(),
                reasoning_type: ReasoningType::Inductive,
                created_at: Timestamp::now(),
            },
        ];
        let result = detect(&hypotheses);
        assert!(result.is_empty());
    }
}
