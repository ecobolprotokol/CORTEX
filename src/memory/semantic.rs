use crate::error::{CortexError, Result};
use crate::types::*;

pub fn store(memory: &mut SemanticMemory, knowledge: Knowledge) -> Result<()> {
    let size = estimate_size(&knowledge);
    while memory.current_usage_bytes + size > memory.capacity_bytes && !memory.knowledge.is_empty() {
        evict_lowest(memory);
    }
    memory.current_usage_bytes += size;
    memory.knowledge.push(knowledge);
    Ok(())
}

pub fn find_by_concept(memory: &SemanticMemory, concept: ConceptId) -> Vec<&Knowledge> {
    memory.knowledge.iter().filter(|k| k.concept == concept).collect()
}

pub fn update_confidence(memory: &mut SemanticMemory, id: KnowledgeId, evidence_polarity: EvidencePolarity) -> Result<()> {
    let knowledge = memory.knowledge.iter_mut().find(|k| k.id == id)
        .ok_or_else(|| CortexError::MemoryError(format!("Knowledge {} not found", id)))?;

    match evidence_polarity {
        EvidencePolarity::Supports => {
            knowledge.confirmation_count += 1;
            let boost = 0.05 * (1.0 - knowledge.confidence.belief);
            knowledge.confidence.belief = (knowledge.confidence.belief + boost).min(1.0);
            knowledge.confidence.evidence_strength = (knowledge.confidence.evidence_strength + 0.03).min(1.0);
            knowledge.confidence.uncertainty = (knowledge.confidence.uncertainty - 0.03).max(0.0);
            knowledge.confidence.consistency = (knowledge.confidence.consistency + 0.02).min(1.0);
            if knowledge.confirmation_count >= 3 {
                knowledge.verification_status = VerificationStatus::Verified;
            } else if knowledge.verification_status == VerificationStatus::Observed
                || knowledge.verification_status == VerificationStatus::Unknown
            {
                knowledge.verification_status = VerificationStatus::Supported;
            }
        }
        EvidencePolarity::Contradicts => {
            knowledge.contradiction_count += 1;
            let penalty = 0.1 * knowledge.confidence.belief;
            knowledge.confidence.belief = (knowledge.confidence.belief - penalty).max(0.0);
            knowledge.confidence.evidence_strength = (knowledge.confidence.evidence_strength - 0.05).max(0.0);
            knowledge.confidence.uncertainty = (knowledge.confidence.uncertainty + 0.05).min(1.0);
            knowledge.confidence.consistency = (knowledge.confidence.consistency - 0.05).max(0.0);
            if knowledge.contradiction_count > knowledge.confirmation_count {
                knowledge.verification_status = VerificationStatus::Contradicted;
            } else {
                knowledge.verification_status = VerificationStatus::Provisional;
            }
        }
        EvidencePolarity::Neutral => {
            knowledge.confidence.uncertainty = (knowledge.confidence.uncertainty + 0.01).min(1.0);
        }
    }

    knowledge.updated_at = Timestamp::now();
    Ok(())
}

pub fn find_contradictions(memory: &SemanticMemory) -> Vec<(KnowledgeId, KnowledgeId)> {
    let mut contradictions = Vec::new();
    for (i, a) in memory.knowledge.iter().enumerate() {
        for b in memory.knowledge.iter().skip(i + 1) {
            if a.concept != b.concept {
                continue;
            }
            let is_contradiction = (a.verification_status == VerificationStatus::Verified
                && b.verification_status == VerificationStatus::Contradicted)
                || (a.verification_status == VerificationStatus::Contradicted
                    && b.verification_status == VerificationStatus::Verified)
                || has_contradicting_relation(a, b)
                || has_conflicting_property(a, b);
            if is_contradiction {
                contradictions.push((a.id, b.id));
            }
        }
    }
    contradictions
}

pub fn merge_knowledge(memory: &mut SemanticMemory, id_a: KnowledgeId, id_b: KnowledgeId) -> Result<Knowledge> {
    let idx_a = memory.knowledge.iter().position(|k| k.id == id_a)
        .ok_or_else(|| CortexError::MemoryError(format!("Knowledge {} not found", id_a)))?;
    let idx_b = memory.knowledge.iter().position(|k| k.id == id_b)
        .ok_or_else(|| CortexError::MemoryError(format!("Knowledge {} not found", id_b)))?;

    if idx_a == idx_b {
        return Err(CortexError::MemoryError("Cannot merge knowledge with itself".into()));
    }

    let b = memory.knowledge.remove(idx_b);
    let b_size = estimate_size(&b);
    let a = &mut memory.knowledge[idx_a];

    for prop in &b.properties {
        if !a.properties.iter().any(|p| p.name == prop.name && value_discriminator(&p.value) == value_discriminator(&prop.value)) {
            a.properties.push(prop.clone());
        }
    }

    for rel in &b.relations {
        if !a.relations.iter().any(|r| r.kind == rel.kind && r.source == rel.source && r.target == rel.target) {
            a.relations.push(rel.clone());
        }
    }

    a.evidence = a.evidence.merge(&b.evidence);
    a.confirmation_count = a.confirmation_count.max(b.confirmation_count);
    a.contradiction_count = a.contradiction_count.max(b.contradiction_count);

    let merged_belief = (a.confidence.belief + b.confidence.belief) / 2.0;
    let merged_evidence = (a.confidence.evidence_strength + b.confidence.evidence_strength) / 2.0;
    let merged_consistency = (a.confidence.consistency + b.confidence.consistency) / 2.0;
    a.confidence.belief = merged_belief;
    a.confidence.evidence_strength = merged_evidence;
    a.confidence.consistency = merged_consistency;

    for prov in &b.provenance {
        if !a.provenance.iter().any(|p| p.source.id == prov.source.id) {
            a.provenance.push(prov.clone());
        }
    }

    a.updated_at = Timestamp::now();
    memory.current_usage_bytes = memory.current_usage_bytes.saturating_sub(b_size);

    let merged = a.clone();
    Ok(merged)
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
    let props_size: u64 = knowledge.properties.iter().map(|p| {
        let name_size = p.name.len() as u64;
        let value_size = match &p.value {
            PropertyValue::Text(s) => s.len() as u64,
            PropertyValue::List(items) => items.len() as u64 * 16,
            _ => 8,
        };
        name_size + value_size + 24
    }).sum();
    let rels_size = knowledge.relations.len() as u64 * 64;
    let evidence_size = knowledge.evidence.items.len() as u64 * 48;
    let prov_size = knowledge.provenance.len() as u64 * 32;
    128 + props_size + rels_size + evidence_size + prov_size
}

fn has_contradicting_relation(a: &Knowledge, b: &Knowledge) -> bool {
    for ra in &a.relations {
        for rb in &b.relations {
            if ra.source == rb.source && ra.target == rb.target {
                if (ra.kind == RelationKind::Supports && rb.kind == RelationKind::Contradicts)
                    || (ra.kind == RelationKind::Contradicts && rb.kind == RelationKind::Supports)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn has_conflicting_property(a: &Knowledge, b: &Knowledge) -> bool {
    for pa in &a.properties {
        for pb in &b.properties {
            if pa.name == pb.name {
                match (&pa.value, &pb.value) {
                    (PropertyValue::Boolean(va), PropertyValue::Boolean(vb)) if va != vb => return true,
                    (PropertyValue::Number(va), PropertyValue::Number(vb)) if (va - vb).abs() > 0.01 => return true,
                    (PropertyValue::Text(va), PropertyValue::Text(vb)) if va != vb => {
                        let confidence_diff = (pa.confidence - pb.confidence).abs();
                        if confidence_diff > 0.3 {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    false
}

fn value_discriminator(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Text(s) => format!("Text({})", s),
        PropertyValue::Number(n) => format!("Number({})", n),
        PropertyValue::Boolean(b) => format!("Boolean({})", b),
        PropertyValue::ConceptRef(id) => format!("ConceptRef({})", id),
        PropertyValue::EntityRef(id) => format!("EntityRef({})", id),
        PropertyValue::List(items) => format!("List(len={})", items.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_knowledge(id: u64, concept: ConceptId, confidence: f32) -> Knowledge {
        Knowledge {
            id: KnowledgeId(id),
            concept,
            properties: vec![Property {
                name: "type".into(),
                value: PropertyValue::Text("test".into()),
                confidence: 0.8,
                provenance: Provenance::user_provided(),
            }],
            relations: Vec::new(),
            evidence: EvidenceSet::new(),
            confidence: ConfidenceState {
                belief: confidence,
                evidence_strength: confidence,
                source_quality: 0.5,
                consistency: 0.5,
                uncertainty: 1.0 - confidence,
                prediction_reliability: 0.0,
                verification_status: VerificationStatus::Observed,
            },
            provenance: vec![Provenance::user_provided()],
            verification_status: VerificationStatus::Observed,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        }
    }

    fn make_memory(capacity: u64) -> SemanticMemory {
        SemanticMemory {
            knowledge: Vec::new(),
            capacity_bytes: capacity,
            current_usage_bytes: 0,
            next_id: KnowledgeId(1),
        }
    }

    #[test]
    fn test_store_and_find() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_knowledge(1, ConceptId(1), 0.8)).unwrap();
        store(&mut memory, make_knowledge(2, ConceptId(2), 0.5)).unwrap();
        store(&mut memory, make_knowledge(3, ConceptId(1), 0.9)).unwrap();

        let found = find_by_concept(&memory, ConceptId(1));
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|k| k.concept == ConceptId(1)));
    }

    #[test]
    fn test_eviction_evicts_lowest_confidence() {
        let mut memory = make_memory(300);
        store(&mut memory, make_knowledge(1, ConceptId(1), 0.9)).unwrap();
        store(&mut memory, make_knowledge(2, ConceptId(2), 0.3)).unwrap();
        store(&mut memory, make_knowledge(3, ConceptId(3), 0.7)).unwrap();
        store(&mut memory, make_knowledge(4, ConceptId(4), 0.1)).unwrap();

        assert!(memory.knowledge.len() < 4);
        let ids: Vec<KnowledgeId> = memory.knowledge.iter().map(|k| k.id).collect();
        assert!(!ids.contains(&KnowledgeId(2)));
    }

    #[test]
    fn test_update_confidence_support() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_knowledge(1, ConceptId(1), 0.5)).unwrap();

        update_confidence(&mut memory, KnowledgeId(1), EvidencePolarity::Supports).unwrap();
        let k = &memory.knowledge[0];
        assert_eq!(k.confirmation_count, 1);
        assert!(k.confidence.belief > 0.5);
    }

    #[test]
    fn test_update_confidence_contradiction() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_knowledge(1, ConceptId(1), 0.5)).unwrap();

        update_confidence(&mut memory, KnowledgeId(1), EvidencePolarity::Contradicts).unwrap();
        let k = &memory.knowledge[0];
        assert_eq!(k.contradiction_count, 1);
        assert!(k.confidence.belief < 0.5);
        assert_eq!(k.verification_status, VerificationStatus::Contradicted);
    }

    #[test]
    fn test_update_confidence_not_found() {
        let mut memory = make_memory(1024 * 1024);
        let result = update_confidence(&mut memory, KnowledgeId(999), EvidencePolarity::Supports);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_contradictions_by_status() {
        let mut memory = make_memory(1024 * 1024);
        let mut k1 = make_knowledge(1, ConceptId(1), 0.9);
        k1.verification_status = VerificationStatus::Verified;
        let mut k2 = make_knowledge(2, ConceptId(1), 0.2);
        k2.verification_status = VerificationStatus::Contradicted;

        store(&mut memory, k1).unwrap();
        store(&mut memory, k2).unwrap();

        let contradictions = find_contradictions(&memory);
        assert_eq!(contradictions.len(), 1);
        assert_eq!(contradictions[0], (KnowledgeId(1), KnowledgeId(2)));
    }

    #[test]
    fn test_find_contradictions_by_relation() {
        let mut memory = make_memory(1024 * 1024);
        let mut k1 = make_knowledge(1, ConceptId(1), 0.9);
        k1.relations.push(Relation {
            id: RelationId(1),
            kind: RelationKind::Supports,
            source: InternalId::Concept(ConceptId(1)),
            target: InternalId::Concept(ConceptId(2)),
            confidence: 0.9,
            provenance: Provenance::user_provided(),
        });
        let mut k2 = make_knowledge(2, ConceptId(1), 0.8);
        k2.relations.push(Relation {
            id: RelationId(2),
            kind: RelationKind::Contradicts,
            source: InternalId::Concept(ConceptId(1)),
            target: InternalId::Concept(ConceptId(2)),
            confidence: 0.8,
            provenance: Provenance::user_provided(),
        });

        store(&mut memory, k1).unwrap();
        store(&mut memory, k2).unwrap();

        let contradictions = find_contradictions(&memory);
        assert_eq!(contradictions.len(), 1);
    }

    #[test]
    fn test_find_contradictions_by_property() {
        let mut memory = make_memory(1024 * 1024);
        let mut k1 = make_knowledge(1, ConceptId(1), 0.9);
        k1.properties[0].value = PropertyValue::Boolean(true);
        k1.properties[0].confidence = 0.9;

        let mut k2 = make_knowledge(2, ConceptId(1), 0.8);
        k2.properties[0].value = PropertyValue::Boolean(false);
        k2.properties[0].confidence = 0.9;

        store(&mut memory, k1).unwrap();
        store(&mut memory, k2).unwrap();

        let contradictions = find_contradictions(&memory);
        assert_eq!(contradictions.len(), 1);
    }

    #[test]
    fn test_find_no_contradictions_different_concepts() {
        let mut memory = make_memory(1024 * 1024);
        let mut k1 = make_knowledge(1, ConceptId(1), 0.9);
        k1.verification_status = VerificationStatus::Verified;
        let mut k2 = make_knowledge(2, ConceptId(2), 0.2);
        k2.verification_status = VerificationStatus::Contradicted;

        store(&mut memory, k1).unwrap();
        store(&mut memory, k2).unwrap();

        let contradictions = find_contradictions(&memory);
        assert!(contradictions.is_empty());
    }

    #[test]
    fn test_merge_knowledge() {
        let mut memory = make_memory(1024 * 1024);
        let mut k1 = make_knowledge(1, ConceptId(1), 0.8);
        k1.properties.push(Property {
            name: "color".into(),
            value: PropertyValue::Text("red".into()),
            confidence: 0.9,
            provenance: Provenance::user_provided(),
        });

        let mut k2 = make_knowledge(2, ConceptId(1), 0.6);
        k2.properties[0] = Property {
            name: "type".into(),
            value: PropertyValue::Text("test".into()),
            confidence: 0.7,
            provenance: Provenance::user_provided(),
        };
        k2.properties.push(Property {
            name: "size".into(),
            value: PropertyValue::Text("large".into()),
            confidence: 0.8,
            provenance: Provenance::user_provided(),
        });

        store(&mut memory, k1).unwrap();
        store(&mut memory, k2).unwrap();

        let merged = merge_knowledge(&mut memory, KnowledgeId(1), KnowledgeId(2)).unwrap();
        assert_eq!(merged.properties.len(), 3);
        assert_eq!(memory.knowledge.len(), 1);
    }

    #[test]
    fn test_merge_same_id_fails() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_knowledge(1, ConceptId(1), 0.8)).unwrap();
        let result = merge_knowledge(&mut memory, KnowledgeId(1), KnowledgeId(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_size_estimation() {
        let k = make_knowledge(1, ConceptId(1), 0.8);
        let size = estimate_size(&k);
        assert!(size >= 128);

        let mut k_large = k.clone();
        k_large.properties.push(Property {
            name: "description".into(),
            value: PropertyValue::Text("a".repeat(100)),
            confidence: 0.5,
            provenance: Provenance::user_provided(),
        });
        let size_large = estimate_size(&k_large);
        assert!(size_large > size);
    }
}
