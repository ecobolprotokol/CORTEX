pub mod hypothesis;
pub mod evidence;
pub mod contradiction;

pub use hypothesis::{HypothesisGenerator, Hypothesis, ReasoningType, ReasoningResult};
pub use evidence::EvidenceEvaluator;
pub use contradiction::{ContradictionDetector, Contradiction};

use crate::error::CortexError;
use crate::types::scalars::Scalar;
use crate::types::ids::HypothesisId;

pub trait ReasoningEngine {
    fn evaluate(&mut self, input: &str) -> Result<ReasoningResult, CortexError>;
    fn max_steps(&self) -> u32;
}

pub fn rank_hypotheses(
    hypotheses: &mut Vec<Hypothesis>,
) {
    hypotheses.sort_by(|a, b| {
        let score_a = compute_hypothesis_score(a);
        let score_b = compute_hypothesis_score(b);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

pub fn compute_hypothesis_score(h: &Hypothesis) -> Scalar {
    let evidence_bonus = (h.evidence_count as Scalar * 0.1).min(0.3);
    h.confidence + evidence_bonus
}

pub fn select_top_hypothesis(
    hypotheses: &[Hypothesis],
    threshold: Scalar,
) -> Option<&Hypothesis> {
    hypotheses
        .iter()
        .filter(|h| compute_hypothesis_score(h) >= threshold)
        .max_by(|a, b| {
            compute_hypothesis_score(a)
                .partial_cmp(&compute_hypothesis_score(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub struct FullReasoningPipeline {
    generator: HypothesisGenerator,
    evidence_evaluator: EvidenceEvaluator,
    contradiction_detector: ContradictionDetector,
    max_steps: u32,
}

impl FullReasoningPipeline {
    pub fn new(max_steps: u32, max_hypotheses: usize) -> Self {
        Self {
            generator: HypothesisGenerator::new(max_hypotheses),
            evidence_evaluator: EvidenceEvaluator::new(),
            contradiction_detector: ContradictionDetector::new(),
            max_steps,
        }
    }

    pub fn reason(&mut self, input: &str, context: &[String]) -> ReasoningResult {
        let mut hypotheses = self.generator.generate(input, context);

        let propositions: Vec<(HypothesisId, String)> = hypotheses
            .iter()
            .map(|h| (h.id, h.proposition.clone()))
            .collect();
        let _contradictions = self.contradiction_detector.detect(&propositions);

        rank_hypotheses(&mut hypotheses);

        let conclusion = select_top_hypothesis(&hypotheses, 0.3).map(|h| h.proposition.clone());

        let steps_used = hypotheses.len() as u32 * 2;
        ReasoningResult {
            hypotheses,
            conclusion,
            steps_used: steps_used.min(self.max_steps),
            budget_remaining: self.max_steps.saturating_sub(steps_used),
        }
    }
}
