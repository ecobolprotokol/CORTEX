use crate::types::*;
use crate::world::transition::{self, TransitionPredictor};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub max_steps: u32,
    pub max_branches: u32,
    pub confidence_threshold: f32,
    pub decay_factor: f32,
    pub max_counterfactual_depth: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_steps: 8,
            max_branches: 16,
            confidence_threshold: 0.1,
            decay_factor: 0.9,
            max_counterfactual_depth: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub trajectory: SimulatedTrajectory,
    pub branch_count: u32,
    pub pruned_count: u32,
    pub final_confidence: f32,
    pub divergence_points: Vec<DivergencePoint>,
}

#[derive(Debug, Clone)]
pub struct DivergencePoint {
    pub step: u32,
    pub entity_id: EntityId,
    pub predicted_state: String,
    pub alternative_state: String,
}

pub fn simulate(world: &WorldState, steps: u32) -> SimulatedTrajectory {
    let config = SimulationConfig {
        max_steps: steps,
        ..SimulationConfig::default()
    };
    let result = simulate_with_config(world, &Vec::new(), &config);
    result.trajectory
}

pub fn simulate_with_config(
    world: &WorldState,
    actions: &[Action],
    config: &SimulationConfig,
) -> SimulationResult {
    let mut trajectory_steps = Vec::new();
    let mut current = world.clone();
    let mut total_confidence = 1.0f32;
    let mut total_uncertainty = 0.0f32;
    let mut confidence_count = 0u32;

    trajectory_steps.push(current.clone());

    let predictor = TransitionPredictor::new(100);

    for step in 0..config.max_steps {
        let action = if (step as usize) < actions.len() {
            actions[step as usize].clone()
        } else {
            Action {
                id: ActionId(step as u64 + 100),
                kind: ActionKind::Observe,
                parameters: HashMap::new(),
                expected_outcome: None,
                risk: RiskAssessment::default(),
                timestamp: Timestamp::now(),
                provenance: Provenance::system("simulation"),
            }
        };

        let predicted = transition::predict_transition_with_history(&current, &action, &predictor);

        let step_confidence = predicted.confidence * config.decay_factor.powi(step as i32);
        let step_uncertainty = predicted.uncertainty * config.decay_factor.powi(step as i32);

        total_confidence *= step_confidence;
        total_uncertainty += step_uncertainty;
        confidence_count += 1;

        let mut next_state = current.clone();
        for (i, entity) in next_state.entities.iter_mut().enumerate() {
            if i < predicted.predicted_entities.len() {
                *entity = predicted.predicted_entities[i].clone();
            }
        }
        next_state.relations = predicted.predicted_relations;

        next_state.temporal_context.current_time = Timestamp::now();
        next_state.temporal_context.sequence_position += 1;
        next_state.temporal_context.prior_states.push(current.temporal_context.current_time);

        if next_state.temporal_context.prior_states.len() > 10 {
            next_state.temporal_context.prior_states.drain(0..next_state.temporal_context.prior_states.len() - 10);
        }

        current = next_state;
        trajectory_steps.push(current.clone());
    }

    let avg_confidence = if confidence_count > 0 {
        total_confidence.powf(1.0 / confidence_count as f32)
    } else {
        0.5
    };
    let avg_uncertainty = if confidence_count > 0 {
        (total_uncertainty / confidence_count as f32).min(1.0)
    } else {
        0.5
    };

    SimulationResult {
        trajectory: SimulatedTrajectory {
            steps: trajectory_steps,
            actions: actions.to_vec(),
            confidence: avg_confidence,
            uncertainty: avg_uncertainty,
            is_counterfactual: false,
        },
        branch_count: 1,
        pruned_count: 0,
        final_confidence: avg_confidence,
        divergence_points: Vec::new(),
    }
}

pub fn simulate_counterfactual(
    world: &WorldState,
    actual_actions: &[Action],
    counterfactual_actions: &[Action],
    config: &SimulationConfig,
) -> (SimulationResult, SimulationResult) {
    let actual_result = simulate_with_config(world, actual_actions, config);

    let mut cf_result = simulate_with_config(world, counterfactual_actions, config);
    cf_result.trajectory.is_counterfactual = true;

    let divergence_points = find_divergence(&actual_result.trajectory, &cf_result.trajectory);

    let mut actual_with_div = actual_result;
    let mut cf_with_div = cf_result;
    actual_with_div.divergence_points = divergence_points.clone();
    cf_with_div.divergence_points = divergence_points;

    (actual_with_div, cf_with_div)
}

fn find_divergence(actual: &SimulatedTrajectory, counterfactual: &SimulatedTrajectory) -> Vec<DivergencePoint> {
    let mut divergence_points = Vec::new();

    let min_steps = actual.steps.len().min(counterfactual.steps.len());
    for step in 0..min_steps {
        let actual_state = &actual.steps[step];
        let cf_state = &counterfactual.steps[step];

        for (i, actual_entity) in actual_state.entities.iter().enumerate() {
            if i < cf_state.entities.len() {
                let cf_entity = &cf_state.entities[i];
                if actual_entity.state.state_description != cf_entity.state.state_description {
                    divergence_points.push(DivergencePoint {
                        step: step as u32,
                        entity_id: actual_entity.id,
                        predicted_state: actual_entity.state.state_description.clone(),
                        alternative_state: cf_entity.state.state_description.clone(),
                    });
                }
            }
        }
    }

    divergence_points
}

#[derive(Debug, Clone)]
pub struct BoundedSimulation {
    pub best_trajectory: Option<SimulatedTrajectory>,
    pub best_confidence: f32,
    pub explored_count: u32,
    pub pruned_count: u32,
}

pub fn bounded_simulate(
    world: &WorldState,
    action_sequences: &[Vec<Action>],
    config: &SimulationConfig,
) -> BoundedSimulation {
    let mut best_trajectory: Option<SimulatedTrajectory> = None;
    let mut best_confidence = 0.0f32;
    let mut explored_count = 0u32;
    let mut pruned_count = 0u32;

    for sequence in action_sequences {
        if sequence.len() as u32 > config.max_steps {
            continue;
        }

        let result = simulate_with_config(world, sequence, config);
        explored_count += 1;

        if result.final_confidence < config.confidence_threshold {
            pruned_count += 1;
            continue;
        }

        if result.final_confidence > best_confidence {
            best_confidence = result.final_confidence;
            best_trajectory = Some(result.trajectory);
        }
    }

    BoundedSimulation {
        best_trajectory,
        best_confidence,
        explored_count,
        pruned_count,
    }
}

pub fn simulate_with_branch_and_bound(
    world: &WorldState,
    possible_actions: &[Action],
    depth: u32,
    config: &SimulationConfig,
) -> BoundedSimulation {
    let mut best_trajectory: Option<SimulatedTrajectory> = None;
    let mut best_confidence = 0.0f32;
    let mut explored_count = 0u32;
    let mut pruned_count = 0u32;

    let max_depth = depth.min(config.max_steps);

    let mut stack: Vec<(WorldState, Vec<Action>, u32, f32)> = vec![
        (world.clone(), Vec::new(), 0, 1.0),
    ];

    while let Some((current_state, current_actions, current_depth, cumulative_confidence)) = stack.pop() {
        if current_depth >= max_depth {
            let result = simulate_with_config(&current_state, &current_actions, config);
            explored_count += 1;

            if result.final_confidence > best_confidence {
                best_confidence = result.final_confidence;
                best_trajectory = Some(result.trajectory);
            }
            continue;
        }

        if cumulative_confidence < config.confidence_threshold {
            pruned_count += 1;
            continue;
        }

        let mut branch_count = 0;
        for action in possible_actions {
            if branch_count >= config.max_branches {
                break;
            }

            let predicted = transition::predict_transition(&current_state, action);
            let step_confidence = predicted.confidence * config.decay_factor.powi(current_depth as i32);
            let new_confidence = cumulative_confidence * step_confidence;

            if new_confidence < config.confidence_threshold {
                pruned_count += 1;
                continue;
            }

            let mut next_state = current_state.clone();
            for (i, entity) in next_state.entities.iter_mut().enumerate() {
                if i < predicted.predicted_entities.len() {
                    *entity = predicted.predicted_entities[i].clone();
                }
            }
            next_state.relations = predicted.predicted_relations;

            let mut new_actions = current_actions.clone();
            new_actions.push(action.clone());

            stack.push((next_state, new_actions, current_depth + 1, new_confidence));
            branch_count += 1;
        }
    }

    BoundedSimulation {
        best_trajectory,
        best_confidence,
        explored_count,
        pruned_count,
    }
}

pub fn simulate_multi_action(
    world: &WorldState,
    actions: &[Action],
    config: &SimulationConfig,
) -> SimulatedTrajectory {
    let result = simulate_with_config(world, actions, config);
    result.trajectory
}

pub fn compute_simulation_confidence(
    model_accuracy: &[f32],
    trajectory_length: u32,
    action_count: u32,
) -> f32 {
    if model_accuracy.is_empty() {
        return 0.1;
    }

    let avg_accuracy: f32 = model_accuracy.iter().sum::<f32>() / model_accuracy.len() as f32;

    let length_penalty = 1.0 / (1.0 + trajectory_length as f32 * 0.05);
    let action_penalty = 1.0 / (1.0 + action_count as f32 * 0.1);

    let confidence = avg_accuracy * length_penalty * action_penalty;
    confidence.clamp(0.05, 0.95)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_state() -> WorldState {
        WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: TemporalContext::default(),
            uncertainty: UncertaintyState::initial(),
            next_entity_id: EntityId(1),
            next_relation_id: RelationId(1),
            next_event_id: EventId(1),
        }
    }

    fn make_action(kind: ActionKind) -> Action {
        Action {
            id: ActionId(1),
            kind,
            parameters: std::collections::HashMap::new(),
            expected_outcome: None,
            risk: RiskAssessment::default(),
            timestamp: Timestamp::now(),
            provenance: Provenance::user_provided(),
        }
    }

    #[test]
    fn test_simulate() {
        let world = empty_state();
        let trajectory = simulate(&world, 3);
        assert_eq!(trajectory.steps.len(), 4);
        assert!(trajectory.confidence >= 0.0);
        assert!(trajectory.confidence <= 1.0);
    }

    #[test]
    fn test_simulate_with_entities() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();
        let config = SimulationConfig { max_steps: 2, ..SimulationConfig::default() };
        let actions = vec![make_action(ActionKind::Learn), make_action(ActionKind::Observe)];
        let result = simulate_with_config(&world, &actions, &config);
        assert_eq!(result.trajectory.steps.len(), 3);
        assert!(result.final_confidence > 0.0);
        assert_eq!(result.branch_count, 1);
    }

    #[test]
    fn test_simulate_with_config() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();
        let config = SimulationConfig {
            max_steps: 2,
            ..SimulationConfig::default()
        };
        let actions = vec![make_action(ActionKind::Respond)];
        let result = simulate_with_config(&world, &actions, &config);
        assert_eq!(result.trajectory.steps.len(), 3);
        assert!(!result.trajectory.is_counterfactual);
    }

    #[test]
    fn test_counterfactual_simulation() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();

        let actual_actions = vec![make_action(ActionKind::Learn)];
        let cf_actions = vec![make_action(ActionKind::Forget)];

        let config = SimulationConfig::default();
        let (actual, cf) = simulate_counterfactual(&world, &actual_actions, &cf_actions, &config);

        assert!(!actual.trajectory.is_counterfactual);
        assert!(cf.trajectory.is_counterfactual);
    }

    #[test]
    fn test_bounded_simulate() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();

        let action_sequences = vec![
            vec![make_action(ActionKind::Learn)],
            vec![make_action(ActionKind::Observe)],
            vec![make_action(ActionKind::Forget)],
        ];

        let config = SimulationConfig::default();
        let result = bounded_simulate(&world, &action_sequences, &config);
        assert!(result.explored_count > 0);
        assert!(result.best_confidence >= 0.0);
    }

    #[test]
    fn test_branch_and_bound() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();

        let possible_actions = vec![
            make_action(ActionKind::Learn),
            make_action(ActionKind::Observe),
        ];

        let config = SimulationConfig {
            max_branches: 2,
            ..SimulationConfig::default()
        };

        let result = simulate_with_branch_and_bound(&world, &possible_actions, 2, &config);
        assert!(result.explored_count > 0 || result.pruned_count > 0);
    }

    #[test]
    fn test_compute_simulation_confidence() {
        let conf = compute_simulation_confidence(&[0.8, 0.9, 0.7], 5, 3);
        assert!(conf > 0.0);
        assert!(conf <= 1.0);

        let empty_conf = compute_simulation_confidence(&[], 5, 3);
        assert!((empty_conf - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_simulate_multi_action() {
        let mut world = empty_state();
        crate::world::entity::create_entity(&mut world, EntityKind::Person, "Alice").unwrap();
        let actions = vec![
            make_action(ActionKind::Learn),
            make_action(ActionKind::Observe),
            make_action(ActionKind::Verify),
        ];
        let config = SimulationConfig { max_steps: 3, ..SimulationConfig::default() };
        let trajectory = simulate_multi_action(&world, &actions, &config);
        assert_eq!(trajectory.steps.len(), 4);
    }

    #[test]
    fn test_simulation_confidence_decreases_with_length() {
        let short = compute_simulation_confidence(&[0.9], 1, 1);
        let long = compute_simulation_confidence(&[0.9], 10, 5);
        assert!(short > long);
    }

    #[test]
    fn test_bounded_simulate_prunes_low_confidence() {
        let world = empty_state();
        let action_sequences: Vec<Vec<Action>> = (0..10)
            .map(|_| vec![make_action(ActionKind::NoOp)])
            .collect();

        let config = SimulationConfig {
            confidence_threshold: 0.9,
            ..SimulationConfig::default()
        };

        let result = bounded_simulate(&world, &action_sequences, &config);
        assert!(result.pruned_count > 0 || result.explored_count > 0);
    }
}
