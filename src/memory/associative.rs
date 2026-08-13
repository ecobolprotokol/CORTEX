use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut AssociativeMemory, association: Association) -> Result<()> {
    memory.current_usage_bytes += estimate_size(&association);
    memory.associations.push(association);
    Ok(())
}

fn estimate_size(_association: &Association) -> u64 {
    128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_association() {
        let mut memory = AssociativeMemory {
            associations: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: AssociationId(1),
        };
        let association = Association {
            id: AssociationId(1),
            source: InternalId::Concept(ConceptId(1)),
            target: InternalId::Concept(ConceptId(2)),
            kind: AssociationKind::Semantic,
            strength: 0.8,
            confidence: 0.7,
            context: ContextState::initial(),
            provenance: Provenance::user_provided(),
            created_at: Timestamp::now(),
            last_strengthened: Timestamp::now(),
            activation_count: 0,
        };
        store(&mut memory, association).unwrap();
        assert_eq!(memory.associations.len(), 1);
    }
}
