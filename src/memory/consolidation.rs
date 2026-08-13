use crate::types::*;

pub fn consolidate(episodic: &mut EpisodicMemory, semantic: &mut SemanticMemory) -> u32 {
    let mut consolidated = 0;
    let candidates: Vec<Episode> = episodic.episodes.iter()
        .filter(|e| !e.consolidated && e.retrieval_count > 2)
        .cloned()
        .collect();
    for episode in candidates {
        let knowledge = Knowledge {
            id: semantic.next_id,
            concept: ConceptId(0),
            properties: Vec::new(),
            relations: Vec::new(),
            evidence: EvidenceSet::new(),
            confidence: episode.confidence.clone(),
            provenance: vec![episode.source.clone()],
            verification_status: VerificationStatus::Inferred,
            created_at: episode.timestamp,
            updated_at: Timestamp::now(),
            confirmation_count: 0,
            contradiction_count: 0,
        };
        semantic.next_id = semantic.next_id.next();
        semantic.knowledge.push(knowledge);
        if let Some(ep) = episodic.episodes.iter_mut().find(|e| e.id == episode.id) {
            ep.consolidated = true;
        }
        consolidated += 1;
    }
    consolidated
}
