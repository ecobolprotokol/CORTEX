use crate::error::CortexError;
use crate::types::*;
use std::collections::HashMap;

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

pub fn find_entity_by_id<'a>(state: &'a WorldState, id: EntityId) -> Option<&'a Entity> {
    state.entities.iter().find(|e| e.id == id)
}

pub fn find_entity_by_id_mut<'a>(state: &'a mut WorldState, id: EntityId) -> Option<&'a mut Entity> {
    state.entities.iter_mut().find(|e| e.id == id)
}

pub fn lookup_by_partial_name<'a>(state: &'a WorldState, query: &str) -> Vec<&'a Entity> {
    let query_lower = query.to_lowercase();
    state
        .entities
        .iter()
        .filter(|e| {
            let name_lower = e.identity.name.to_lowercase();
            if name_lower.contains(&query_lower) {
                return true;
            }
            e.identity.aliases.iter().any(|alias| alias.to_lowercase().contains(&query_lower))
        })
        .collect()
}

pub fn entity_similarity(a: &Entity, b: &Entity) -> f32 {
    let mut score = 0.0f32;
    let mut total_weight = 0.0f32;

    let kind_weight = 0.25;
    total_weight += kind_weight;
    if a.kind == b.kind {
        score += kind_weight;
    }

    let name_weight = 0.30;
    total_weight += name_weight;
    let name_sim = name_similarity(&a.identity.name, &b.identity.name);
    score += name_sim * name_weight;

    let alias_weight = 0.10;
    total_weight += alias_weight;
    let alias_sim = alias_overlap_score(&a.identity.aliases, &b.identity.aliases);
    score += alias_sim * alias_weight;

    let state_weight = 0.20;
    total_weight += state_weight;
    let state_sim = state_description_similarity(&a.state.state_description, &b.state.state_description);
    score += state_sim * state_weight;

    let prop_weight = 0.15;
    total_weight += prop_weight;
    let prop_sim = property_overlap_score(&a.properties, &b.properties);
    score += prop_sim * prop_weight;

    if total_weight > 0.0 {
        score / total_weight
    } else {
        0.0
    }
}

fn name_similarity(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    if a_lower == b_lower {
        return 1.0;
    }
    let a_chars: Vec<char> = a_lower.chars().collect();
    let b_chars: Vec<char> = b_lower.chars().collect();
    let max_len = a_chars.len().max(b_chars.len()) as f32;
    if max_len == 0.0 {
        return 1.0;
    }
    let common = a_chars.iter().filter(|c| b_chars.contains(c)).count() as f32;
    common / max_len
}

fn alias_overlap_score(a_aliases: &[String], b_aliases: &[String]) -> f32 {
    if a_aliases.is_empty() && b_aliases.is_empty() {
        return 0.5;
    }
    let a_set: std::collections::HashSet<&str> = a_aliases.iter().map(|s| s.as_str()).collect();
    let b_set: std::collections::HashSet<&str> = b_aliases.iter().map(|s| s.as_str()).collect();
    let intersection = a_set.intersection(&b_set).count() as f32;
    let union = a_set.union(&b_set).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

fn state_description_similarity(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }
    let a_words: std::collections::HashSet<&str> = a.split_whitespace().collect();
    let b_words: std::collections::HashSet<&str> = b.split_whitespace().collect();
    if a_words.is_empty() && b_words.is_empty() {
        return 1.0;
    }
    let intersection = a_words.intersection(&b_words).count() as f32;
    let union = a_words.union(&b_words).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

fn property_overlap_score(a_props: &[Property], b_props: &[Property]) -> f32 {
    if a_props.is_empty() && b_props.is_empty() {
        return 0.5;
    }
    if a_props.is_empty() || b_props.is_empty() {
        return 0.0;
    }
    let a_names: std::collections::HashSet<&str> = a_props.iter().map(|p| p.name.as_str()).collect();
    let b_names: std::collections::HashSet<&str> = b_props.iter().map(|p| p.name.as_str()).collect();
    let intersection = a_names.intersection(&b_names).count() as f32;
    let union = a_names.union(&b_names).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

pub fn merge_entities(state: &mut WorldState, primary_id: EntityId, secondary_id: EntityId) -> std::result::Result<EntityId, CortexError> {
    let primary_idx = state.entities.iter().position(|e| e.id == primary_id)
        .ok_or_else(|| CortexError::WorldModelError(format!("Primary entity {:?} not found", primary_id)))?;
    let secondary_idx = state.entities.iter().position(|e| e.id == secondary_id)
        .ok_or_else(|| CortexError::WorldModelError(format!("Secondary entity {:?} not found", secondary_id)))?;

    if primary_idx == secondary_idx {
        return Ok(primary_id);
    }

    let similarity = {
        let a = &state.entities[primary_idx];
        let b = &state.entities[secondary_idx];
        entity_similarity(a, b)
    };

    if similarity < 0.3 {
        return Err(CortexError::WorldModelError(format!(
            "Entities too dissimilar to merge (similarity: {:.2})",
            similarity
        )));
    }

    let secondary = state.entities.remove(secondary_idx);
    let primary = &mut state.entities[primary_idx];

    for alias in &secondary.identity.aliases {
        if !primary.identity.aliases.contains(alias) {
            primary.identity.aliases.push(alias.clone());
        }
    }
    if !primary.identity.aliases.contains(&secondary.identity.name)
        && primary.identity.name != secondary.identity.name
    {
        primary.identity.aliases.push(secondary.identity.name);
    }

    for prop in &secondary.properties {
        if !primary.properties.iter().any(|p| p.name == prop.name) {
            primary.properties.push(prop.clone());
        }
    }

    for rel_id in &secondary.relations {
        if !primary.relations.contains(rel_id) {
            primary.relations.push(*rel_id);
        }
    }

    primary.confidence = primary.confidence.max(secondary.confidence);
    primary.identity.identity_confidence = primary.identity.identity_confidence.max(secondary.identity.identity_confidence);
    primary.updated_at = Timestamp::now();

    for entity in &mut state.entities {
        entity.relations.retain(|r| *r != RelationId(secondary_id.0));
    }

    for relation in &mut state.relations {
        if let InternalId::Entity(id) = &relation.source {
            if *id == secondary_id {
                relation.source = InternalId::Entity(primary_id);
            }
        }
        if let InternalId::Entity(id) = &relation.target {
            if *id == secondary_id {
                relation.target = InternalId::Entity(primary_id);
            }
        }
    }

    Ok(primary_id)
}

pub fn activate_entity(
    state: &mut WorldState,
    entity_id: EntityId,
    new_state_description: &str,
    observation_confidence: f32,
) -> std::result::Result<(), CortexError> {
    let entity = state.entities.iter_mut().find(|e| e.id == entity_id)
        .ok_or_else(|| CortexError::WorldModelError(format!("Entity {:?} not found", entity_id)))?;

    let confidence_boost = observation_confidence * 0.3;
    entity.confidence = (entity.confidence + confidence_boost).min(1.0);
    entity.identity.identity_confidence = (entity.identity.identity_confidence + confidence_boost * 0.5).min(1.0);

    entity.state.state_description = new_state_description.to_string();
    entity.state.state_timestamp = Timestamp::now();
    entity.state.state_confidence = (entity.state.state_confidence * 0.7 + observation_confidence * 0.3).min(1.0);

    entity.updated_at = Timestamp::now();

    Ok(())
}

pub fn compute_activation_spread(
    state: &WorldState,
    source_id: EntityId,
    max_hops: u32,
    decay: f32,
) -> HashMap<EntityId, f32> {
    let mut activations: HashMap<EntityId, f32> = HashMap::new();
    activations.insert(source_id, 1.0);

    let mut frontier: Vec<(EntityId, u32)> = vec![(source_id, 0)];

    while let Some((current_id, depth)) = frontier.pop() {
        if depth >= max_hops {
            continue;
        }
        let current_activation = activations.get(&current_id).copied().unwrap_or(0.0);
        let next_activation = current_activation * decay;

        if next_activation < 0.01 {
            continue;
        }

        if let Some(entity) = state.entities.iter().find(|e| e.id == current_id) {
            for rel_id in &entity.relations {
                if let Some(relation) = state.relations.iter().find(|r| r.id == *rel_id) {
                    let neighbor_id = match relation.source {
                        InternalId::Entity(id) if id == current_id => {
                            match relation.target {
                                InternalId::Entity(id) => Some(id),
                                _ => None,
                            }
                        }
                        InternalId::Entity(id) => Some(id),
                        _ => None,
                    };

                    if let Some(nid) = neighbor_id {
                        let existing = activations.get(&nid).copied().unwrap_or(0.0);
                        let new_activation = existing.max(next_activation);
                        if new_activation > existing {
                            activations.insert(nid, new_activation);
                            frontier.push((nid, depth + 1));
                        }
                    }
                }
            }
        }
    }

    activations
}

pub fn remove_entity(state: &mut WorldState, id: EntityId) -> std::result::Result<Entity, CortexError> {
    let idx = state.entities.iter().position(|e| e.id == id)
        .ok_or_else(|| CortexError::WorldModelError(format!("Entity {:?} not found", id)))?;
    let removed = state.entities.remove(idx);

    for rel_id in &removed.relations {
        state.relations.retain(|r| r.id != *rel_id);
    }

    for entity in &mut state.entities {
        entity.relations.retain(|r| *r != RelationId(id.0));
    }

    Ok(removed)
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

    #[test]
    fn test_create_entity() {
        let mut state = empty_state();
        let id = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        assert_eq!(id, EntityId(1));
        assert_eq!(state.entities.len(), 1);
        assert_eq!(state.next_entity_id, EntityId(2));
    }

    #[test]
    fn test_find_entity() {
        let mut state = empty_state();
        create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        assert!(find_entity(&state, "Alice").is_some());
        assert!(find_entity(&state, "Bob").is_none());
    }

    #[test]
    fn test_find_entity_by_id() {
        let mut state = empty_state();
        let id = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        assert!(find_entity_by_id(&state, id).is_some());
        assert!(find_entity_by_id(&state, EntityId(99)).is_none());
    }

    #[test]
    fn test_lookup_by_partial_name() {
        let mut state = empty_state();
        create_entity(&mut state, EntityKind::Person, "Alice Smith").unwrap();
        create_entity(&mut state, EntityKind::Person, "Bob Jones").unwrap();
        create_entity(&mut state, EntityKind::Object, "Alice's Book").unwrap();

        let results = lookup_by_partial_name(&state, "alice");
        assert_eq!(results.len(), 2);
        let results = lookup_by_partial_name(&state, "smith");
        assert_eq!(results.len(), 1);
        let results = lookup_by_partial_name(&state, "zzz");
        assert!(results.is_empty());
    }

    #[test]
    fn test_lookup_by_alias() {
        let mut state = empty_state();
        let id = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let entity = find_entity_by_id_mut(&mut state, id).unwrap();
        entity.identity.aliases.push("Ally".to_string());

        let results = lookup_by_partial_name(&state, "ally");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_entity_similarity_same() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let a = find_entity_by_id(&state, id1).unwrap();
        let b = find_entity_by_id(&state, id2).unwrap();
        let sim = entity_similarity(a, b);
        assert!(sim > 0.8);
    }

    #[test]
    fn test_entity_similarity_different_kinds() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::Object, "Alice").unwrap();
        let a = find_entity_by_id(&state, id1).unwrap();
        let b = find_entity_by_id(&state, id2).unwrap();
        let sim = entity_similarity(a, b);
        assert!(sim < 0.85);
    }

    #[test]
    fn test_entity_similarity_different_names() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::Person, "Zzzzzz").unwrap();
        let a = find_entity_by_id(&state, id1).unwrap();
        let b = find_entity_by_id(&state, id2).unwrap();
        let sim = entity_similarity(a, b);
        assert!(sim < 0.7);
    }

    #[test]
    fn test_merge_entities() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::Person, "Alice Smith").unwrap();
        find_entity_by_id_mut(&mut state, id2).unwrap().identity.aliases.push("Ally".to_string());

        let merged = merge_entities(&mut state, id1, id2).unwrap();
        assert_eq!(merged, id1);
        assert_eq!(state.entities.len(), 1);
        let merged_entity = find_entity_by_id(&state, id1).unwrap();
        assert!(merged_entity.identity.aliases.contains(&"Ally".to_string()));
        assert!(merged_entity.identity.aliases.contains(&"Alice Smith".to_string()));
    }

    #[test]
    fn test_merge_too_dissimilar() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::ConceptualObject, "XYZ").unwrap();
        find_entity_by_id_mut(&mut state, id2).unwrap().state.state_description = "xyz".to_string();
        let result = merge_entities(&mut state, id1, id2);
        assert!(result.is_err());
    }

    #[test]
    fn test_activate_entity() {
        let mut state = empty_state();
        let id = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        activate_entity(&mut state, id, "running", 0.8).unwrap();
        let entity = find_entity_by_id(&state, id).unwrap();
        assert_eq!(entity.state.state_description, "running");
        assert!(entity.state.state_confidence > 0.5);
        assert!(entity.confidence > 0.5);
    }

    #[test]
    fn test_activation_spread() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let id2 = create_entity(&mut state, EntityKind::Person, "Bob").unwrap();

        let rel_id = state.next_relation_id;
        state.next_relation_id = rel_id.next();
        let relation = Relation {
            id: rel_id,
            kind: RelationKind::RelatedTo,
            source: InternalId::Entity(id1),
            target: InternalId::Entity(id2),
            confidence: 0.9,
            provenance: Provenance::system("test"),
        };
        state.relations.push(relation);
        let entity = find_entity_by_id_mut(&mut state, id1).unwrap();
        entity.relations.push(rel_id);

        let activations = compute_activation_spread(&state, id1, 2, 0.5);
        assert!(activations.get(&id2).copied().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn test_remove_entity() {
        let mut state = empty_state();
        let id1 = create_entity(&mut state, EntityKind::Person, "Alice").unwrap();
        let _id2 = create_entity(&mut state, EntityKind::Person, "Bob").unwrap();
        remove_entity(&mut state, id1).unwrap();
        assert_eq!(state.entities.len(), 1);
        assert!(find_entity_by_id(&state, id1).is_none());
    }
}
