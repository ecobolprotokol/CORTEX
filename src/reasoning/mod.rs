pub mod hypothesis;
pub mod evidence;
pub mod contradiction;

use crate::config::ReasoningConfig;
use crate::error::Result;
use crate::types::*;

pub trait ReasoningEngine {
    fn evaluate(&mut self, representation: &crate::neural::NeuralRepresentation, memories: &MemoryRetrieval, world: &WorldState) -> Result<ReasoningResult>;
    fn state(&self) -> &ReasoningState;
}

pub struct ReasoningEngineImpl {
    config: ReasoningConfig,
    state: ReasoningState,
}

impl ReasoningEngineImpl {
    pub fn new(config: &ReasoningConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: ReasoningState {
                active_hypotheses: Vec::new(),
                conclusion: None,
                premises: Vec::new(),
                evidence_index: std::collections::HashMap::new(),
                contradiction_log: Vec::new(),
                budget_remaining: config.max_steps,
                next_hypothesis_id: HypothesisId(1),
            },
        })
    }
}

impl ReasoningEngine for ReasoningEngineImpl {
    fn evaluate(&mut self, _representation: &crate::neural::NeuralRepresentation, memories: &MemoryRetrieval, _world: &WorldState) -> Result<ReasoningResult> {
        let hypotheses = hypothesis::generate(memories, &mut self.state);
        let mut budget = self.config.max_steps;
        let mut evaluated = Vec::new();
        for hyp in &hypotheses {
            if budget == 0 { break; }
            evaluated.push(hyp.clone());
            budget -= 1;
        }
        let contradictions = contradiction::detect(&evaluated);
        let ranked = hypothesis::rank(evaluated, &contradictions);
        let conclusion = if budget > 0 && !ranked.is_empty() {
            let top = &ranked[0];
            Some(Conclusion {
                hypothesis_id: top.id,
                proposition: top.proposition.clone(),
                confidence: top.confidence,
                evidence_strength: top.evidence.total_strength(),
                reasoning_steps: self.config.max_steps - budget,
                bounded: budget == 0,
            })
        } else {
            None
        };
        self.state.budget_remaining = budget;
        self.state.conclusion = conclusion.clone();
        Ok(ReasoningResult {
            hypotheses: ranked,
            contradictions,
            budget_remaining: budget,
            conclusion,
        })
    }

    fn state(&self) -> &ReasoningState {
        &self.state
    }
}
