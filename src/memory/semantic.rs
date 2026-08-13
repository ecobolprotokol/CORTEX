use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut SemanticMemory, knowledge: Knowledge) -> Result<()> {
    while memory.current_usage_bytes >= memory.capacity_bytes && !memory.knowledge.is_empty() {
        evict_lowest(memory);
    }
    memory.current_usage_bytes += estimate_size(&knowledge);
    memory.knowledge.push(knowledge);
    Ok(())
}

pub fn find_by_concept(memory: &SemanticMemory, concept: ConceptId) -> Vec<&Knowledge> {
    memory.knowledge.iter().filter(|k| k.concept == concept).collect()
}

fn evict_lowest(memory: &mut SemanticMemory) {
    if memory.knowledge.is_empty() {
        return;
    }
    let mut min_idx = 0;
    let mut min_conf = f32::MAX;
    for (i, k) in memory.knowledge.iter().enumerate() {
        let conf = k.confidence.overall();
        if conf < min_conf {
            min_conf = conf;
            min_idx = i;
        }
    }
    let removed = memory.knowledge.remove(min_idx);
    memory.current_usage_bytes = memory.current_usage_bytes.saturating_sub(estimate_size(&removed));
}

fn estimate_size(knowledge: &Knowledge) -> u64 {
    (knowledge.properties.len() * 64 + knowledge.relations.len() * 64 + 128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_knowledge(concept: ConceptId) -> Knowledge {
        Knowledge {
            id: KnowledgeId(1),
            concept,
            properties: vec![Property {
                name: "type".into(),
                value: PropertyValue::Text("test".into()),
                confidence: 0.8,
                provenance: Provenance::user_provided(),
            }],
            relations: Vec::new(),
            evidence: EvidenceSet::new(),
            confidence: ConfidenceState::default(),
            provenance: vec![Provenance::user_provided()],
            verification_status: VerificationStatus::Observed,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        }
    }

    #[test]
    fn test_store_knowledge() {
        let mut memory = SemanticMemory {
            knowledge: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: KnowledgeId(1),
        };
        store(&mut memory, make_knowledge(ConceptId(1))).unwrap();
        assert_eq!(memory.knowledge.len(), 1);
    }

    #[test]
    fn test_find_by_concept() {
        let mut memory = SemanticMemory {
            knowledge: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: KnowledgeId(1),
        };
        store(&mut memory, make_knowledge(ConceptId(1))).unwrap();
        store(&mut memory, make_knowledge(ConceptId(2))).unwrap();

        let found = find_by_concept(&memory, ConceptId(1));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].concept, ConceptId(1));
    }
}
