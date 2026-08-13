use crate::types::*;

pub fn consolidate(episodic: &mut EpisodicMemory, semantic: &mut SemanticMemory) -> u32 {
    let mut consolidated = 0;
    let candidates: Vec<EpisodeId> = episodic.episodes.iter()
        .filter(|e| !e.consolidated && e.retrieval_count > 2)
        .map(|e| e.id)
        .collect();

    for episode_id in candidates {
        if let Some(episode) = episodic.episodes.iter().find(|e| e.id == episode_id) {
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
            if let Some(ep) = episodic.episodes.iter_mut().find(|e| e.id == episode_id) {
                ep.consolidated = true;
            }
            consolidated += 1;
        }
    }
    consolidated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidate_no_candidates() {
        let mut episodic = EpisodicMemory {
            episodes: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: EpisodeId(1),
        };
        let mut semantic = SemanticMemory {
            knowledge: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: KnowledgeId(1),
        };
        let result = consolidate(&mut episodic, &mut semantic);
        assert_eq!(result, 0);
    }
}
