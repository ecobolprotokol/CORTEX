use crate::types::*;

pub fn predict_transition(state: &WorldState, action: &super::observation::Action) -> PredictedState {
    let affected = identify_affected(state, action);
    PredictedState {
        predicted_entities: affected,
        predicted_relations: state.relations.clone(),
        confidence: 0.5,
        uncertainty: 0.5,
        prediction_horizon: 1,
    }
}

fn identify_affected(state: &WorldState, _action: &super::observation::Action) -> Vec<Entity> {
    state.entities.clone()
}
