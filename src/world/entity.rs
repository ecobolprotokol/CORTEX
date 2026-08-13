use crate::error::Result;
use crate::types::*;

pub fn create_entity(state: &mut WorldState, kind: EntityKind, name: &str) -> Result<EntityId> {
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
