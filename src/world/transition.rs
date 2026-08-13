use crate::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TransitionRecord {
    pub from_state: String,
    pub to_state: String,
    pub action_kind: ActionKind,
    pub count: u64,
    pub last_seen: Timestamp,
}

#[derive(Debug, Clone)]
pub struct TransitionPredictor {
    pub history: Vec<TransitionRecord>,
    pub max_history: usize,
    pub base_confidence: f32,
}

impl Default for TransitionPredictor {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            max_history: 1000,
            base_confidence: 0.5,
        }
    }
}

impl TransitionPredictor {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history,
            base_confidence: 0.5,
        }
    }

    pub fn record_transition(&mut self, from: &str, to: &str, action_kind: ActionKind) {
        if let Some(existing) = self.history.iter_mut().find(|r| {
            r.from_state == from && r.to_state == to && r.action_kind == action_kind
        }) {
            existing.count += 1;
            existing.last_seen = Timestamp::now();
        } else {
            self.history.push(TransitionRecord {
                from_state: from.to_string(),
                to_state: to.to_string(),
                action_kind,
                count: 1,
                last_seen: Timestamp::now(),
            });
        }
        if self.history.len() > self.max_history {
            let min_count = self.history.iter().map(|r| r.count).min().unwrap_or(0);
            self.history.retain(|r| r.count > min_count);
            if self.history.len() > self.max_history {
                self.history.drain(0..self.history.len() - self.max_history);
            }
        }
    }

    pub fn transition_frequency(&self, from: &str, to: &str, action_kind: ActionKind) -> f32 {
        let total: u64 = self.history.iter()
            .filter(|r| r.from_state == from && r.action_kind == action_kind)
            .map(|r| r.count)
            .sum();
        if total == 0 {
            return 0.0;
        }
        let matching: u64 = self.history.iter()
            .filter(|r| r.from_state == from && r.to_state == to && r.action_kind == action_kind)
            .map(|r| r.count)
            .sum();
        matching as f32 / total as f32
    }

    pub fn most_likely_next(&self, from: &str, action_kind: ActionKind) -> Option<(String, f32)> {
        let candidates: Vec<(&TransitionRecord, f32)> = self.history.iter()
            .filter(|r| r.from_state == from && r.action_kind == action_kind)
            .map(|r| {
                let total: u64 = self.history.iter()
                    .filter(|h| h.from_state == from && h.action_kind == action_kind)
                    .map(|h| h.count)
                    .sum();
                let freq = if total > 0 { r.count as f32 / total as f32 } else { 0.0 };
                (r, freq)
            })
            .collect();

        candidates.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(r, freq)| (r.to_state.clone(), freq))
    }
}

pub fn predict_transition(state: &WorldState, action: &crate::types::Action) -> PredictedState {
    let predictor = TransitionPredictor::default();
    predict_transition_with_history(state, action, &predictor)
}

pub fn predict_transition_with_history(
    state: &WorldState,
    action: &crate::types::Action,
    predictor: &TransitionPredictor,
) -> PredictedState {
    let affected_ids = identify_affected_entities(state, action);
    let mut predicted_entities = Vec::new();

    for entity in &state.entities {
        if affected_ids.contains(&entity.id) {
            let mut predicted = entity.clone();
            apply_action_effects(&mut predicted, action, state);
            predicted_entities.push(predicted);
        } else {
            predicted_entities.push(entity.clone());
        }
    }

    let predicted_relations = predict_relation_changes(state, action, &affected_ids);
    let confidence = compute_transition_confidence(state, action, predictor, &affected_ids);
    let uncertainty = 1.0 - confidence;

    PredictedState {
        predicted_entities,
        predicted_relations,
        confidence,
        uncertainty,
        prediction_horizon: 1,
    }
}

fn identify_affected_entities(state: &WorldState, action: &Action) -> Vec<EntityId> {
    let mut affected = Vec::new();

    if let Some(target) = action.parameters.get("target_entity") {
        if let ActionParameter::Text(name) = target {
            for entity in &state.entities {
                if entity.identity.name.to_lowercase() == name.to_lowercase()
                    || entity.identity.aliases.iter().any(|a| a.to_lowercase() == name.to_lowercase())
                {
                    affected.push(entity.id);
                }
            }
        } else if let ActionParameter::Integer(id_val) = target {
            affected.push(EntityId(*id_val as u64));
        }
    }

    if let Some(scope) = action.parameters.get("scope") {
        if let ActionParameter::Text(scope_text) = scope {
            match scope_text.as_str() {
                "all" => {
                    affected.extend(state.entities.iter().map(|e| e.id));
                }
                "related" => {
                    for entity in &state.entities {
                        for rel_id in &entity.relations {
                            if let Some(relation) = state.relations.iter().find(|r| r.id == *rel_id) {
                                match (&relation.source, &relation.target) {
                                    (InternalId::Entity(src), _) if affected.contains(src) => {
                                        if let InternalId::Entity(tgt) = &relation.target {
                                            affected.push(*tgt);
                                        }
                                    }
                                    (_, InternalId::Entity(tgt)) if affected.contains(tgt) => {
                                        if let InternalId::Entity(src) = &relation.source {
                                            affected.push(*src);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                "participants" => {
                    for event in &state.active_events {
                        affected.extend(&event.participants);
                    }
                }
                _ => {}
            }
        }
    }

    match action.kind {
        ActionKind::Respond | ActionKind::Query => {
            if affected.is_empty() && !state.entities.is_empty() {
                affected.push(state.entities[0].id);
            }
        }
        ActionKind::Forget => {
            if let Some(name) = action.parameters.get("target_entity") {
                if let ActionParameter::Text(n) = name {
                    for entity in &state.entities {
                        if entity.identity.name.to_lowercase() == n.to_lowercase() {
                            affected.push(entity.id);
                        }
                    }
                }
            }
        }
        ActionKind::Consolidate => {
            affected.extend(state.entities.iter().map(|e| e.id));
        }
        _ => {}
    }

    affected.dedup();
    affected.retain(|id| state.entities.iter().any(|e| e.id == *id));
    affected
}

fn apply_action_effects(entity: &mut Entity, action: &Action, _state: &WorldState) {
    let confidence_delta = match action.kind {
        ActionKind::Respond => {
            entity.state.state_confidence = (entity.state.state_confidence + 0.1).min(1.0);
            entity.state.state_description = format!("responded_to");
            0.05
        }
        ActionKind::Observe => {
            entity.state.state_confidence = (entity.state.state_confidence + 0.15).min(1.0);
            entity.state.state_description = format!("observed");
            0.08
        }
        ActionKind::Learn => {
            entity.confidence = (entity.confidence + 0.1).min(1.0);
            entity.identity.identity_confidence = (entity.identity.identity_confidence + 0.05).min(1.0);
            entity.state.state_description = format!("learning");
            0.1
        }
        ActionKind::Forget => {
            entity.confidence = (entity.confidence - 0.2).max(0.0);
            entity.state.state_confidence = (entity.state.state_confidence - 0.15).max(0.0);
            entity.state.state_description = format!("fading");
            -0.1
        }
        ActionKind::Verify => {
            entity.confidence = (entity.confidence + 0.2).min(1.0);
            entity.identity.identity_confidence = (entity.identity.identity_confidence + 0.1).min(1.0);
            entity.state.state_description = format!("verified");
            0.15
        }
        ActionKind::Consolidate => {
            entity.confidence = (entity.confidence + 0.05).min(1.0);
            entity.state.state_confidence = (entity.state.state_confidence + 0.05).min(1.0);
            entity.state.state_description = format!("consolidated");
            0.05
        }
        ActionKind::Store => {
            entity.confidence = (entity.confidence + 0.03).min(1.0);
            entity.state.state_description = format!("stored");
            0.03
        }
        ActionKind::NoOp => 0.0,
        ActionKind::Query | ActionKind::Plan | ActionKind::Fetch | ActionKind::Checkpoint => {
            0.02
        }
    };

    entity.confidence = (entity.confidence + confidence_delta).min(1.0);
    entity.state.state_timestamp = Timestamp::now();
    entity.updated_at = Timestamp::now();
}

fn predict_relation_changes(
    state: &WorldState,
    action: &Action,
    affected_ids: &[EntityId],
) -> Vec<Relation> {
    let mut relations = state.relations.clone();

    if action.kind == ActionKind::Store || action.kind == ActionKind::Consolidate {
        if affected_ids.len() >= 2 {
            let already_connected: Vec<(EntityId, EntityId)> = relations.iter()
                .filter_map(|r| {
                    if let (InternalId::Entity(a), InternalId::Entity(b)) = (&r.source, &r.target) {
                        Some((*a, *b))
                    } else {
                        None
                    }
                })
                .collect();

            for i in 0..affected_ids.len() {
                for j in (i + 1)..affected_ids.len() {
                    let a = affected_ids[i];
                    let b = affected_ids[j];
                    let connected = already_connected.iter().any(|(x, y)| {
                        (*x == a && *y == b) || (*x == b && *y == a)
                    });
                    if !connected {
                        let rel_id = state.next_relation_id;
                        relations.push(Relation {
                            id: rel_id,
                            kind: RelationKind::RelatedTo,
                            source: InternalId::Entity(a),
                            target: InternalId::Entity(b),
                            confidence: 0.4,
                            provenance: Provenance::derived(&[action.provenance.clone()]),
                        });
                    }
                }
            }
        }
    }

    if action.kind == ActionKind::Forget {
        let forget_target_names: Vec<String> = action.parameters.iter()
            .filter_map(|(_, v)| {
                if let ActionParameter::Text(name) = v {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        if !forget_target_names.is_empty() {
            let forget_entity_ids: Vec<EntityId> = state.entities.iter()
                .filter(|e| forget_target_names.iter().any(|n| n.to_lowercase() == e.identity.name.to_lowercase()))
                .map(|e| e.id)
                .collect();

            relations.retain(|r| {
                let involves_forgotten = match (&r.source, &r.target) {
                    (InternalId::Entity(a), _) | (_, InternalId::Entity(a)) => forget_entity_ids.contains(a),
                    _ => false,
                };
                !involves_forgotten
            });
        }
    }

    relations
}

fn compute_transition_confidence(
    state: &WorldState,
    action: &Action,
    predictor: &TransitionPredictor,
    affected_ids: &[EntityId],
) -> f32 {
    if affected_ids.is_empty() {
        return 0.1;
    }

    let entity_confidence: f32 = affected_ids.iter()
        .filter_map(|id| state.entities.iter().find(|e| e.id == *id))
        .map(|e| e.confidence)
        .sum::<f32>() / affected_ids.len() as f32;

    let pattern_bonus = predictor.history.iter()
        .filter(|r| r.action_kind == action.kind)
        .map(|r| r.count as f32)
        .sum::<f32>();
    let pattern_factor = if pattern_bonus > 0.0 {
        (pattern_bonus.ln() / 10.0).min(0.3)
    } else {
        0.0
    };

    let risk_factor = 1.0 - (action.risk.score * 0.3);

    let affected_density = affected_ids.len() as f32 / state.entities.len().max(1) as f32;
    let complexity_penalty = affected_density * 0.15;

    let base = 0.3;
    let confidence = base
        + entity_confidence * 0.3
        + pattern_factor
        + risk_factor * 0.15
        - complexity_penalty;

    confidence.clamp(0.05, 0.95)
}

pub fn predict_cascading_effects(
    state: &WorldState,
    action: &Action,
    max_depth: u32,
) -> Vec<PredictedState> {
    let mut trajectory = Vec::new();
    let mut current_state = state.clone();
    let mut current_action = action.clone();

    for step in 0..max_depth {
        let predicted = predict_transition(&current_state, &current_action);
        trajectory.push(predicted.clone());

        let cascading_action = Action {
            id: ActionId(action.id.0 + step as u64 + 1),
            kind: ActionKind::Observe,
            parameters: HashMap::new(),
            expected_outcome: None,
            risk: RiskAssessment::default(),
            timestamp: Timestamp::now(),
            provenance: Provenance::system("cascade_predictor"),
        };
        current_action = cascading_action;

        let mut next_state = current_state.clone();
        for (i, entity) in next_state.entities.iter_mut().enumerate() {
            if i < predicted.predicted_entities.len() {
                *entity = predicted.predicted_entities[i].clone();
            }
        }
        current_state = next_state;
    }

    trajectory
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

    fn make_action_with_target(kind: ActionKind, target: &str) -> Action {
        let mut params = std::collections::HashMap::new();
        params.insert("target_entity".to_string(), ActionParameter::Text(target.to_string()));
        Action {
            id: ActionId(1),
            kind,
            parameters: params,
            expected_outcome: None,
            risk: RiskAssessment::default(),
            timestamp: Timestamp::now(),
            provenance: Provenance::user_provided(),
        }
    }

    #[test]
    fn test_predict_transition() {
        let state = empty_state();
        let action = make_action(ActionKind::Respond);
        let predicted = predict_transition(&state, &action);
        assert!(predicted.confidence >= 0.0);
        assert!(predicted.confidence <= 1.0);
    }

    #[test]
    fn test_predict_with_entities() {
        let mut state = empty_state();
        let id = crate::world::entity::create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let action = make_action_with_target(ActionKind::Observe, "Alice");
        let predicted = predict_transition(&state, &action);
        assert!(!predicted.predicted_entities.is_empty());
        let alice = predicted.predicted_entities.iter().find(|e| e.id == id).unwrap();
        assert_eq!(alice.state.state_description, "observed");
    }

    #[test]
    fn test_forget_reduces_confidence() {
        let mut state = empty_state();
        let id = crate::world::entity::create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let original_confidence = state.entities.iter().find(|e| e.id == id).unwrap().confidence;
        let action = make_action_with_target(ActionKind::Forget, "Alice");
        let predicted = predict_transition(&state, &action);
        let alice = predicted.predicted_entities.iter().find(|e| e.id == id).unwrap();
        assert!(alice.confidence < original_confidence);
    }

    #[test]
    fn test_transition_predictor_history() {
        let mut predictor = TransitionPredictor::new(100);
        predictor.record_transition("idle", "active", ActionKind::Observe);
        predictor.record_transition("idle", "active", ActionKind::Observe);
        predictor.record_transition("idle", "fading", ActionKind::Observe);

        let freq = predictor.transition_frequency("idle", "active", ActionKind::Observe);
        assert!((freq - 2.0 / 3.0).abs() < 0.01);

        let most_likely = predictor.most_likely_next("idle", ActionKind::Observe);
        assert!(most_likely.is_some());
        let (state, prob) = most_likely.unwrap();
        assert_eq!(state, "active");
        assert!(prob > 0.6);
    }

    #[test]
    fn test_cascading_effects() {
        let mut state = empty_state();
        crate::world::entity::create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let action = make_action_with_target(ActionKind::Learn, "Alice");
        let trajectory = predict_cascading_effects(&state, &action, 3);
        assert_eq!(trajectory.len(), 3);
    }

    #[test]
    fn test_confidence_range() {
        let mut state = empty_state();
        for i in 0..5 {
            crate::world::entity::create_entity(
                &mut state,
                EntityKind::Person,
                &format!("Person{}", i),
            ).unwrap();
        }
        let action = make_action(ActionKind::Consolidate);
        let predicted = predict_transition(&state, &action);
        assert!(predicted.confidence >= 0.0);
        assert!(predicted.confidence <= 1.0);
        assert!(predicted.uncertainty >= 0.0);
        assert!(predicted.uncertainty <= 1.0);
    }

    #[test]
    fn test_scope_all() {
        let mut state = empty_state();
        crate::world::entity::create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        crate::world::entity::create_entity(&mut state, EntityKind::Person, "Bob").unwrap();
        let mut action = make_action(ActionKind::Verify);
        action.parameters.insert("scope".to_string(), ActionParameter::Text("all".to_string()));
        let predicted = predict_transition(&state, &action);
        for entity in &predicted.predicted_entities {
            assert_eq!(entity.state.state_description, "verified");
        }
    }

    #[test]
    fn test_predictor_max_history() {
        let mut predictor = TransitionPredictor::new(5);
        for i in 0..10 {
            predictor.record_transition("a", &format!("b{}", i), ActionKind::Observe);
        }
        assert!(predictor.history.len() <= 5);
    }
}
