use crate::types::*;

pub fn simulate(world: &WorldState, steps: u32) -> SimulatedTrajectory {
    let mut trajectory_steps = Vec::new();
    let mut current = world.clone();
    for _ in 0..steps {
        trajectory_steps.push(current.clone());
    }
    SimulatedTrajectory {
        steps: trajectory_steps,
        actions: Vec::new(),
        confidence: 0.5,
        uncertainty: 0.5,
        is_counterfactual: false,
    }
}
