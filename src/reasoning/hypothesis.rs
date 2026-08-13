use serde::{Deserialize, Serialize};
use crate::types::ids::HypothesisId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningType {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
    Temporal,
    Causal,
    Counterfactual,
    Constraint,
    Consistency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: HypothesisId,
    pub proposition: String,
    pub confidence: Scalar,
    pub reasoning_type: ReasoningType,
    pub evidence_count: u32,
}

#[derive(Debug, Clone)]
pub struct ReasoningResult {
    pub hypotheses: Vec<Hypothesis>,
    pub conclusion: Option<String>,
    pub steps_used: u32,
    pub budget_remaining: u32,
}

pub struct HypothesisGenerator {
    pub max_hypotheses: usize,
}

impl HypothesisGenerator {
    pub fn new(max_hypotheses: usize) -> Self {
        Self { max_hypotheses }
    }

    pub fn generate(&self, input: &str, context: &[String]) -> Vec<Hypothesis> {
        let mut hypotheses = Vec::new();

        hypotheses.push(Hypothesis {
            id: HypothesisId::from(1),
            proposition: format!("Input suggests: {}", input),
            confidence: 0.5,
            reasoning_type: ReasoningType::Inductive,
            evidence_count: 0,
        });

        for (i, ctx) in context.iter().take(self.max_hypotheses - 1).enumerate() {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from((i + 2) as u64),
                proposition: format!("Context indicates: {}", ctx),
                confidence: 0.3,
                reasoning_type: ReasoningType::Abductive,
                evidence_count: 0,
            });
        }

        hypotheses.truncate(self.max_hypotheses);
        hypotheses
    }
}
