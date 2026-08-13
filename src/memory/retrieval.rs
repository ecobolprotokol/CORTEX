use crate::types::*;

pub fn retrieve(state: &MemoryState, query: &MemoryQuery, context: &ContextState) -> crate::error::Result<MemoryRetrieval> {
    let mut retrieval = MemoryRetrieval::default();
    let max = query.max_results as usize;

    match query.query_type {
        MemoryQueryType::Episodic | MemoryQueryType::All => {
            let mut scored: Vec<ScoredEpisode> = state.episodic.episodes.iter()
                .map(|ep| {
                    let relevance = score_episode(ep, &query.text, context);
                    ScoredEpisode {
                        episode: ep.clone(),
                        relevance_score: relevance,
                    }
                })
                .filter(|se| se.relevance_score > query.min_confidence)
                .collect();
            scored.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(max);
            retrieval.episodic = scored;
        }
        _ => {}
    }

    match query.query_type {
        MemoryQueryType::Semantic | MemoryQueryType::All => {
            let mut scored: Vec<ScoredKnowledge> = state.semantic.knowledge.iter()
                .filter(|k| k.confidence.overall() >= query.min_confidence)
                .map(|k| {
                    let relevance = score_knowledge(k, &query.text, context);
                    ScoredKnowledge {
                        knowledge: k.clone(),
                        relevance_score: relevance,
                    }
                })
                .collect();
            scored.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(max);
            retrieval.semantic = scored;
        }
        _ => {}
    }

    match query.query_type {
        MemoryQueryType::Procedural | MemoryQueryType::All => {
            let mut scored: Vec<ScoredProcedure> = state.procedural.procedures.iter()
                .map(|p| {
                    let relevance = score_procedure(p, &query.text, context);
                    ScoredProcedure {
                        procedure: p.clone(),
                        relevance_score: relevance,
                    }
                })
                .collect();
            scored.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(max);
            retrieval.procedural = scored;
        }
        _ => {}
    }

    match query.query_type {
        MemoryQueryType::Associative | MemoryQueryType::All => {
            let mut scored: Vec<ScoredAssociation> = state.associative.associations.iter()
                .map(|a| {
                    let relevance = a.strength * a.confidence;
                    ScoredAssociation {
                        association: a.clone(),
                        relevance_score: relevance,
                    }
                })
                .collect();
            scored.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(max);
            retrieval.associative = scored;
        }
        _ => {}
    }

    detect_contradictions(&mut retrieval);

    Ok(retrieval)
}

fn score_episode(episode: &Episode, query_text: &str, _context: &ContextState) -> Scalar {
    let text_overlap = compute_text_overlap(&episode.observation.text, query_text);
    let recency = 1.0 / (1.0 + (Timestamp::now().0.saturating_sub(episode.timestamp.0)) as Scalar / 3_600_000.0);
    let confidence = episode.confidence.overall();

    text_overlap * 0.4 + recency * 0.3 + episode.importance * 0.15 + confidence * 0.15
}

fn score_knowledge(knowledge: &Knowledge, query_text: &str, _context: &ContextState) -> Scalar {
    let concept_text = knowledge.properties.iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let text_overlap = compute_text_overlap(&concept_text, query_text);
    let confidence = knowledge.confidence.overall();
    let stability = knowledge.confirmation_count as Scalar / (knowledge.confirmation_count + knowledge.contradiction_count + 1) as Scalar;

    text_overlap * 0.4 + confidence * 0.3 + stability * 0.3
}

fn score_procedure(procedure: &Procedure, query_text: &str, _context: &ContextState) -> Scalar {
    let text_overlap = compute_text_overlap(&procedure.condition.description, query_text);
    let success_rate = if procedure.success_count + procedure.failure_count > 0 {
        procedure.success_count as Scalar / (procedure.success_count + procedure.failure_count) as Scalar
    } else {
        0.5
    };
    text_overlap * 0.5 + success_rate * 0.3 + procedure.confidence * 0.2
}

fn compute_text_overlap(text: &str, query: &str) -> Scalar {
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    let text_words: Vec<&str> = text_lower.split_whitespace().collect();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    if query_words.is_empty() || text_words.is_empty() {
        return 0.0;
    }

    let overlap = query_words.iter()
        .filter(|qw| text_words.iter().any(|tw| tw.contains(*qw) || qw.contains(*tw)))
        .count() as Scalar;

    overlap / query_words.len() as Scalar
}

fn detect_contradictions(retrieval: &mut MemoryRetrieval) {
    for (i, ek) in retrieval.semantic.iter().enumerate() {
        for (j, ek2) in retrieval.semantic.iter().enumerate() {
            if i >= j { continue; }
            if ek.knowledge.concept == ek2.knowledge.concept {
                if ek.knowledge.verification_status == VerificationStatus::Verified
                    && ek2.knowledge.verification_status == VerificationStatus::Contradicted
                    || (ek.knowledge.verification_status == VerificationStatus::Contradicted
                        && ek2.knowledge.verification_status == VerificationStatus::Verified)
                {
                    retrieval.contradictions.push(Contradiction {
                        claim_a: HypothesisId(ek.knowledge.id.0),
                        claim_b: HypothesisId(ek2.knowledge.id.0),
                        description: format!("Contradictory knowledge about same concept"),
                        severity: 0.7,
                        detected_at: Timestamp::now(),
                        resolved: false,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_overlap() {
        let score = compute_text_overlap("gravity is a force", "gravity force");
        assert!(score > 0.5);
    }

    #[test]
    fn test_text_overlap_empty() {
        assert_eq!(compute_text_overlap("", "test"), 0.0);
        assert_eq!(compute_text_overlap("test", ""), 0.0);
    }
}
