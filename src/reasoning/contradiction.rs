use crate::types::ids::HypothesisId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct Contradiction {
    pub claim_a: HypothesisId,
    pub claim_b: HypothesisId,
    pub description: String,
    pub severity: Scalar,
}

pub struct ContradictionDetector;

impl ContradictionDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self, propositions: &[(HypothesisId, String)]) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();

        for i in 0..propositions.len() {
            for j in (i + 1)..propositions.len() {
                let (id_a, prop_a) = &propositions[i];
                let (id_b, prop_b) = &propositions[j];

                if prop_a.contains("not") && prop_b.contains(&prop_a.replace("not", "").trim()) {
                    contradictions.push(Contradiction {
                        claim_a: *id_a,
                        claim_b: *id_b,
                        description: format!("'{}' contradicts '{}'", prop_a, prop_b),
                        severity: 0.8,
                    });
                }
            }
        }

        contradictions
    }
}
