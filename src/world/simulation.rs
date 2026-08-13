use serde::{Deserialize, Serialize};

use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub steps: Vec<SimulationStep>,
    pub final_state: String,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub trajectory_length: usize,
    pub divergences: Vec<StateDivergence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStep {
    pub step_index: u32,
    pub action: String,
    pub state_before: String,
    pub state_after: String,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDivergence {
    pub at_step: u32,
    pub expected: String,
    pub actual: String,
    pub divergence_magnitude: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualScenario {
    pub description: String,
    pub modified_action: String,
    pub at_step: u32,
    pub result: Option<SimulationResult>,
}

pub struct WorldSimulator {
    pub max_horizon: u32,
    pub default_confidence: Scalar,
    pub counterfactual_scenarios: Vec<CounterfactualScenario>,
}

impl WorldSimulator {
    pub fn new() -> Self {
        Self {
            max_horizon: 10,
            default_confidence: 0.5,
            counterfactual_scenarios: Vec::new(),
        }
    }

    pub fn with_horizon(mut self, horizon: u32) -> Self {
        self.max_horizon = horizon;
        self
    }

    pub fn simulate(
        &self,
        initial_state: &str,
        actions: &[String],
        transition_fn: Option<&dyn Fn(&str, &str) -> (String, Scalar)>,
    ) -> SimulationResult {
        let mut steps = Vec::new();
        let mut current_state = initial_state.to_string();
        let mut total_confidence = 1.0f32;
        let mut divergences = Vec::new();

        let effective_horizon = actions.len().min(self.max_horizon as usize);

        for (i, action) in actions.iter().take(effective_horizon).enumerate() {
            let state_before = current_state.clone();

            let (new_state, confidence) = if let Some(f) = transition_fn {
                f(&current_state, action)
            } else {
                let predicted = format!("Step {}: after '{}'", i + 1, action);
                (predicted, self.default_confidence)
            };

            if confidence < 0.3 {
                divergences.push(StateDivergence {
                    at_step: i as u32,
                    expected: state_before.clone(),
                    actual: new_state.clone(),
                    divergence_magnitude: 1.0 - confidence,
                });
            }

            total_confidence *= confidence;

            steps.push(SimulationStep {
                step_index: i as u32,
                action: action.clone(),
                state_before,
                state_after: new_state.clone(),
                confidence,
            });

            current_state = new_state;
        }

        let uncertainty = 1.0 - total_confidence;

        SimulationResult {
            steps: steps.clone(),
            final_state: current_state,
            confidence: total_confidence,
            uncertainty,
            trajectory_length: steps.len(),
            divergences,
        }
    }

    pub fn counterfactual(
        &mut self,
        original: &SimulationResult,
        modified_action: &str,
        at_step: u32,
        transition_fn: Option<&dyn Fn(&str, &str) -> (String, Scalar)>,
    ) -> SimulationResult {
        if at_step as usize >= original.steps.len() {
            return SimulationResult {
                steps: Vec::new(),
                final_state: String::new(),
                confidence: 0.0,
                uncertainty: 1.0,
                trajectory_length: 0,
                divergences: Vec::new(),
            };
        }

        let initial_state = if at_step == 0 {
            original.steps[0].state_before.clone()
        } else {
            original.steps[(at_step - 1) as usize].state_after.clone()
        };

        let mut remaining_actions: Vec<String> = original
            .steps
            .iter()
            .skip(at_step as usize + 1)
            .map(|s| s.action.clone())
            .collect();
        remaining_actions.insert(0, modified_action.to_string());

        let result = self.simulate(&initial_state, &remaining_actions, transition_fn);

        self.counterfactual_scenarios.push(CounterfactualScenario {
            description: format!(
                "What if '{}' was done instead at step {}?",
                modified_action, at_step
            ),
            modified_action: modified_action.to_string(),
            at_step,
            result: Some(result.clone()),
        });

        result
    }

    pub fn compare_trajectories(
        &self,
        a: &SimulationResult,
        b: &SimulationResult,
    ) -> Scalar {
        let min_len = a.steps.len().min(b.steps.len());
        if min_len == 0 {
            return 0.0;
        }

        let total_diff: Scalar = (0..min_len)
            .map(|i| {
                let conf_a = a.steps[i].confidence;
                let conf_b = b.steps[i].confidence;
                (conf_a - conf_b).abs()
            })
            .sum();

        total_diff / min_len as Scalar
    }

    pub fn monte_carlo_simulate(
        &self,
        initial_state: &str,
        action: &str,
        samples: u32,
        transition_fn: Option<&dyn Fn(&str, &str) -> (String, Scalar)>,
    ) -> (String, Scalar, Scalar) {
        let mut results = Vec::new();
        let mut confidences = Vec::new();

        for _ in 0..samples {
            let result = self.simulate(
                initial_state,
                &[action.to_string()],
                transition_fn,
            );
            results.push(result.final_state);
            confidences.push(result.confidence);
        }

        let avg_confidence = confidences.iter().sum::<Scalar>() / samples as Scalar;
        let variance = confidences
            .iter()
            .map(|c| (c - avg_confidence).powi(2))
            .sum::<Scalar>()
            / samples as Scalar;

        let most_common = results
            .iter()
            .max_by_key(|r| results.iter().filter(|x| *x == *r).count())
            .cloned()
            .unwrap_or_default();

        (most_common, avg_confidence, variance)
    }
}

impl Default for WorldSimulator {
    fn default() -> Self {
        Self::new()
    }
}
