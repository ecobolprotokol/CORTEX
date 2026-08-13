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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let world = WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: TemporalContext::default(),
            uncertainty: UncertaintyState::initial(),
            next_entity_id: EntityId(1),
            next_relation_id: RelationId(1),
            next_event_id: EventId(1),
        };
        let trajectory = simulate(&world, 3);
        assert_eq!(trajectory.steps.len(), 3);
        assert_eq!(trajectory.confidence, 0.5);
    }
}
