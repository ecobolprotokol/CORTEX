use crate::types::ids::HypothesisId;
use crate::types::scalars::Scalar;
use serde::{Deserialize, Serialize};

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
    pub supporting_factors: Vec<String>,
    pub contradicting_factors: Vec<String>,
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
    pub next_id: u64,
}

impl HypothesisGenerator {
    pub fn new(max_hypotheses: usize) -> Self {
        Self {
            max_hypotheses,
            next_id: 1,
        }
    }

    pub fn generate(&mut self, input: &str, context: &[String]) -> Vec<Hypothesis> {
        let mut hypotheses = Vec::new();
        let words: Vec<&str> = input.split_whitespace().collect();
        let word_count = words.len();

        hypotheses.push(Hypothesis {
            id: HypothesisId::from(self.next_id),
            proposition: format!("Input suggests: {}", input),
            confidence: 0.5,
            reasoning_type: ReasoningType::Inductive,
            evidence_count: 0,
            supporting_factors: vec![format!("Direct observation of {} words", word_count)],
            contradicting_factors: Vec::new(),
        });
        self.next_id += 1;

        for (i, ctx) in context
            .iter()
            .take(self.max_hypotheses.saturating_sub(1))
            .enumerate()
        {
            let rtype = match i % 4 {
                0 => ReasoningType::Abductive,
                1 => ReasoningType::Deductive,
                2 => ReasoningType::Analogical,
                _ => ReasoningType::Temporal,
            };
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: format!("Context indicates: {}", ctx),
                confidence: 0.3,
                reasoning_type: rtype,
                evidence_count: 0,
                supporting_factors: vec![format!("Context clue from entry {}", i + 1)],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        if word_count > 5 {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: format!("Complex input detected: {} semantic units", word_count / 2),
                confidence: 0.4,
                reasoning_type: ReasoningType::Constraint,
                evidence_count: 0,
                supporting_factors: vec!["Input complexity analysis".into()],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        if input.contains('?') {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: "Input is interrogative; may require information retrieval".into(),
                confidence: 0.6,
                reasoning_type: ReasoningType::Deductive,
                evidence_count: 0,
                supporting_factors: vec!["Question mark detected".into()],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        if input.contains("if") || input.contains("would") {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: "Counterfactual or conditional reasoning required".into(),
                confidence: 0.45,
                reasoning_type: ReasoningType::Counterfactual,
                evidence_count: 0,
                supporting_factors: vec!["Conditional keyword detected".into()],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        hypotheses.truncate(self.max_hypotheses);
        hypotheses
    }

    pub fn generate_from_problem(
        &mut self,
        problem: &str,
        known_facts: &[String],
        constraints: &[String],
    ) -> Vec<Hypothesis> {
        let mut hypotheses = Vec::new();

        for (i, fact) in known_facts.iter().enumerate().take(self.max_hypotheses / 2) {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: format!("Given fact {}: {}", i + 1, fact),
                confidence: 0.6,
                reasoning_type: ReasoningType::Deductive,
                evidence_count: 1,
                supporting_factors: vec!["Known fact".into()],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        for constraint in constraints
            .iter()
            .take(self.max_hypotheses.saturating_sub(hypotheses.len()))
        {
            hypotheses.push(Hypothesis {
                id: HypothesisId::from(self.next_id),
                proposition: format!("Constraint: {}", constraint),
                confidence: 0.5,
                reasoning_type: ReasoningType::Constraint,
                evidence_count: 0,
                supporting_factors: vec!["Problem constraint".into()],
                contradicting_factors: Vec::new(),
            });
            self.next_id += 1;
        }

        let derived = format!(
            "Problem analysis: '{}' with {} facts and {} constraints",
            problem,
            known_facts.len(),
            constraints.len()
        );
        hypotheses.push(Hypothesis {
            id: HypothesisId::from(self.next_id),
            proposition: derived,
            confidence: 0.4,
            reasoning_type: ReasoningType::Abductive,
            evidence_count: 0,
            supporting_factors: vec!["Problem decomposition".into()],
            contradicting_factors: Vec::new(),
        });
        self.next_id += 1;

        hypotheses.truncate(self.max_hypotheses);
        hypotheses
    }
}
