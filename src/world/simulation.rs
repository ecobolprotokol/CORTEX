use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub steps: Vec<String>,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
}

pub struct WorldSimulator;

impl WorldSimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn simulate(&self, initial_state: &str, actions: &[String], horizon: u32) -> SimulationResult {
        let mut steps = vec![initial_state.to_string()];
        for (i, action) in actions.iter().enumerate() {
            if i as u32 >= horizon {
                break;
            }
            steps.push(format!("Step {}: after '{}'", i + 1, action));
        }
        SimulationResult {
            steps,
            confidence: 0.5,
            uncertainty: 0.5,
        }
    }
}
