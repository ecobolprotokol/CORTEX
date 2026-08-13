use crate::error::{CortexError, Result};
use crate::types::*;

pub fn store(memory: &mut AssociativeMemory, association: Association) -> Result<()> {
    let size = estimate_size(&association);
    while memory.current_usage_bytes + size > memory.capacity_bytes && !memory.associations.is_empty() {
        evict_weakest(memory);
    }
    memory.current_usage_bytes += size;
    memory.associations.push(association);
    Ok(())
}

pub fn find_by_source(memory: &AssociativeMemory, source: InternalId) -> Vec<&Association> {
    memory.associations.iter().filter(|a| a.source == source).collect()
}

pub fn find_by_target(memory: &AssociativeMemory, target: InternalId) -> Vec<&Association> {
    memory.associations.iter().filter(|a| a.target == target).collect()
}

pub fn strengthen(memory: &mut AssociativeMemory, id: AssociationId, amount: f32) -> Result<()> {
    let association = memory.associations.iter_mut().find(|a| a.id == id)
        .ok_or_else(|| CortexError::MemoryError(format!("Association {} not found", id)))?;

    association.strength = (association.strength + amount).min(1.0);
    association.last_strengthened = Timestamp::now();
    association.activation_count += 1;
    Ok(())
}

pub fn find_by_kind(memory: &AssociativeMemory, kind: AssociationKind) -> Vec<&Association> {
    memory.associations.iter().filter(|a| a.kind == kind).collect()
}

fn evict_weakest(memory: &mut AssociativeMemory) {
    if memory.associations.is_empty() {
        return;
    }
    let mut min_idx = 0;
    let mut min_value = f32::MAX;
    for (i, a) in memory.associations.iter().enumerate() {
        let value = a.strength * a.confidence;
        if value < min_value {
            min_value = value;
            min_idx = i;
        }
    }
    let removed = memory.associations.remove(min_idx);
    memory.current_usage_bytes = memory.current_usage_bytes.saturating_sub(estimate_size(&removed));
}

fn estimate_size(_association: &Association) -> u64 {
    128
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_association(id: u64, source: InternalId, target: InternalId, kind: AssociationKind, strength: f32) -> Association {
        Association {
            id: AssociationId(id),
            source,
            target,
            kind,
            strength,
            confidence: 0.7,
            context: ContextState::initial(),
            provenance: Provenance::user_provided(),
            created_at: Timestamp::now(),
            last_strengthened: Timestamp::now(),
            activation_count: 0,
        }
    }

    fn make_memory(capacity: u64) -> AssociativeMemory {
        AssociativeMemory {
            associations: Vec::new(),
            capacity_bytes: capacity,
            current_usage_bytes: 0,
            next_id: AssociationId(1),
        }
    }

    #[test]
    fn test_store_and_find_by_source() {
        let mut memory = make_memory(1024 * 1024);
        let src = InternalId::Concept(ConceptId(1));
        let tgt1 = InternalId::Concept(ConceptId(2));
        let tgt2 = InternalId::Concept(ConceptId(3));

        store(&mut memory, make_association(1, src, tgt1, AssociationKind::Semantic, 0.8)).unwrap();
        store(&mut memory, make_association(2, src, tgt2, AssociationKind::Temporal, 0.6)).unwrap();
        store(&mut memory, make_association(3, tgt1, src, AssociationKind::Causal, 0.5)).unwrap();

        let found = find_by_source(&memory, src);
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|a| a.source == src));
    }

    #[test]
    fn test_find_by_target() {
        let mut memory = make_memory(1024 * 1024);
        let src1 = InternalId::Concept(ConceptId(1));
        let src2 = InternalId::Concept(ConceptId(2));
        let tgt = InternalId::Concept(ConceptId(3));

        store(&mut memory, make_association(1, src1, tgt, AssociationKind::Semantic, 0.8)).unwrap();
        store(&mut memory, make_association(2, src2, tgt, AssociationKind::Temporal, 0.6)).unwrap();
        store(&mut memory, make_association(3, tgt, src1, AssociationKind::Causal, 0.5)).unwrap();

        let found = find_by_target(&memory, tgt);
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|a| a.target == tgt));
    }

    #[test]
    fn test_strengthen() {
        let mut memory = make_memory(1024 * 1024);
        let src = InternalId::Concept(ConceptId(1));
        let tgt = InternalId::Concept(ConceptId(2));
        store(&mut memory, make_association(1, src, tgt, AssociationKind::Semantic, 0.5)).unwrap();

        strengthen(&mut memory, AssociationId(1), 0.3).unwrap();
        let a = &memory.associations[0];
        assert!((a.strength - 0.8).abs() < 0.001);
        assert_eq!(a.activation_count, 1);
    }

    #[test]
    fn test_strengthen_clamps_at_one() {
        let mut memory = make_memory(1024 * 1024);
        let src = InternalId::Concept(ConceptId(1));
        let tgt = InternalId::Concept(ConceptId(2));
        store(&mut memory, make_association(1, src, tgt, AssociationKind::Semantic, 0.9)).unwrap();

        strengthen(&mut memory, AssociationId(1), 0.5).unwrap();
        let a = &memory.associations[0];
        assert!((a.strength - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_strengthen_not_found() {
        let mut memory = make_memory(1024 * 1024);
        let result = strengthen(&mut memory, AssociationId(999), 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_by_kind() {
        let mut memory = make_memory(1024 * 1024);
        let src = InternalId::Concept(ConceptId(1));
        let tgt = InternalId::Concept(ConceptId(2));

        store(&mut memory, make_association(1, src, tgt, AssociationKind::Semantic, 0.8)).unwrap();
        store(&mut memory, make_association(2, src, tgt, AssociationKind::Temporal, 0.6)).unwrap();
        store(&mut memory, make_association(3, src, tgt, AssociationKind::Semantic, 0.5)).unwrap();

        let semantic = find_by_kind(&memory, AssociationKind::Semantic);
        assert_eq!(semantic.len(), 2);

        let temporal = find_by_kind(&memory, AssociationKind::Temporal);
        assert_eq!(temporal.len(), 1);

        let causal = find_by_kind(&memory, AssociationKind::Causal);
        assert!(causal.is_empty());
    }

    #[test]
    fn test_eviction() {
        let mut memory = make_memory(200);
        let src = InternalId::Concept(ConceptId(1));
        let tgt = InternalId::Concept(ConceptId(2));

        store(&mut memory, make_association(1, src, tgt, AssociationKind::Semantic, 0.9)).unwrap();
        store(&mut memory, make_association(2, src, tgt, AssociationKind::Temporal, 0.1)).unwrap();
        store(&mut memory, make_association(3, src, tgt, AssociationKind::Causal, 0.7)).unwrap();

        assert!(memory.associations.len() < 3);
        let ids: Vec<AssociationId> = memory.associations.iter().map(|a| a.id).collect();
        assert!(!ids.contains(&AssociationId(2)));
    }

    #[test]
    fn test_empty_memory_queries() {
        let memory = make_memory(1024 * 1024);
        let src = InternalId::Concept(ConceptId(1));
        assert!(find_by_source(&memory, src).is_empty());
        assert!(find_by_target(&memory, src).is_empty());
        assert!(find_by_kind(&memory, AssociationKind::Semantic).is_empty());
    }
}
