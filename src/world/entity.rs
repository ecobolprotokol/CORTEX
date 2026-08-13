use crate::error::CortexError;
use crate::types::*;

pub fn create_entity(state: &mut WorldState, kind: EntityKind, name: &str) -> std::result::Result<EntityId, CortexError> {
    let id = state.next_entity_id;
    state.next_entity_id = id.next();
    let entity = Entity {
        id,
        kind,
        identity: IdentityState {
            name: name.to_string(),
            aliases: Vec::new(),
            unique_identifier: None,
            identity_confidence: 0.5,
        },
        properties: Vec::new(),
        state: EntityState {
            state_description: "initial".into(),
            state_properties: Vec::new(),
            state_timestamp: Timestamp::now(),
            state_confidence: 0.5,
        },
        relations: Vec::new(),
        confidence: 0.5,
        provenance: Vec::new(),
        created_at: Timestamp::now(),
        updated_at: Timestamp::now(),
    };
    state.entities.push(entity);
    Ok(id)
}

pub fn find_entity<'a>(state: &'a WorldState, name: &str) -> Option<&'a Entity> {
    state.entities.iter().find(|e| e.identity.name == name)
}

pub fn entity_count(state: &WorldState) -> usize {
    state.entities.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let mut state = WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: TemporalContext::default(),
            uncertainty: UncertaintyState::initial(),
            next_entity_id: EntityId(1),
            next_relation_id: RelationId(1),
            next_event_id: EventId(1),
        };
        let id = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        assert_eq!(id, EntityId(1));
        assert_eq!(state.entities.len(), 1);
        assert_eq!(state.next_entity_id, EntityId(2));
    }

    #[test]
    fn test_find_entity() {
        let mut state = WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: TemporalContext::default(),
            uncertainty: UncertaintyState::initial(),
            next_entity_id: EntityId(1),
            next_relation_id: RelationId(1),
            next_event_id: EventId(1),
        };
        create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        assert!(find_entity(&state, "Alice").is_some());
        assert!(find_entity(&state, "Bob").is_none());
    }
}
