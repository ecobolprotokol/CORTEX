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
                    description: format!("Contradiction between {} and {}", a.proposition.predicate, b.proposition.predicate),
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
