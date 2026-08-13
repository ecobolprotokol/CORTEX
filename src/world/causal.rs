use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct CausalHypothesis {
    pub cause: String,
    pub effect: String,
    pub strength: Scalar,
    pub evidence_count: u32,
}

pub struct CausalModel {
    pub hypotheses: Vec<CausalHypothesis>,
}

impl CausalModel {
    pub fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
        }
    }

    pub fn add_hypothesis(&mut self, cause: &str, effect: &str) {
        self.hypotheses.push(CausalHypothesis {
            cause: cause.to_string(),
            effect: effect.to_string(),
            strength: 0.1,
            evidence_count: 0,
        });
    }

    pub fn strengthen(&mut self, cause: &str, effect: &str) {
        if let Some(h) = self.hypotheses.iter_mut()
            .find(|h| h.cause == cause && h.effect == effect)
        {
            h.evidence_count += 1;
            h.strength = (h.strength + 0.1).min(1.0);
        }
    }
}
