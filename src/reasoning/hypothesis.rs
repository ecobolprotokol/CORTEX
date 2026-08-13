use crate::types::*;

pub fn generate(memories: &MemoryRetrieval, state: &mut ReasoningState) -> Vec<Hypothesis> {
    let mut hypotheses = Vec::new();

    for scored_knowledge in &memories.semantic {
        let id = state.next_hypothesis_id;
        state.next_hypothesis_id = id.next();
        let knowledge = &scored_knowledge.knowledge;
        let predicate = if knowledge.properties.is_empty() {
            format!("concept-{}", knowledge.concept.0)
        } else {
            knowledge.properties.iter().map(|p| format!("{}={}", p.name, format_property_value(&p.value))).collect::<Vec<_>>().join(", ")
        };
        hypotheses.push(Hypothesis {
            id,
            proposition: Proposition {
                subject: InternalId::Concept(knowledge.concept),
                predicate,
                object: None,
                modifiers: Vec::new(),
                negated: false,
            },
            evidence: knowledge.evidence.clone(),
            counter_evidence: EvidenceSet::new(),
            confidence: knowledge.confidence.overall() * scored_knowledge.relevance_score,
            dependencies: Vec::new(),
            contradictions: Vec::new(),
            provenance: knowledge.provenance.clone(),
            reasoning_type: ReasoningType::Inductive,
            created_at: Timestamp::now(),
        });
    }

    for scored_episode in &memories.episodic {
        let id = state.next_hypothesis_id;
        state.next_hypothesis_id = id.next();
        let episode = &scored_episode.episode;
        hypotheses.push(Hypothesis {
            id,
            proposition: Proposition {
                subject: InternalId::Episode(episode.id),
                predicate: episode.observation.text.clone(),
                object: None,
                modifiers: Vec::new(),
                negated: false,
            },
            evidence: EvidenceSet::new(),
            counter_evidence: EvidenceSet::new(),
            confidence: episode.confidence.overall() * scored_episode.relevance_score * 0.7,
            dependencies: Vec::new(),
            contradictions: Vec::new(),
            provenance: vec![episode.source.clone()],
            reasoning_type: ReasoningType::Analogical,
            created_at: Timestamp::now(),
        });
    }

    hypotheses.truncate(10);
    hypotheses
}

pub fn rank(mut hypotheses: Vec<Hypothesis>, contradictions: &[Contradiction]) -> Vec<Hypothesis> {
    for hyp in &mut hypotheses {
        let contradiction_penalty = contradictions.iter()
            .filter(|c| c.claim_a == hyp.id || c.claim_b == hyp.id)
            .map(|c| c.severity)
            .sum::<Scalar>();
        hyp.confidence = (hyp.confidence - contradiction_penalty * 0.2).max(0.0);
    }
    hypotheses.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    hypotheses
}

fn format_property_value(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Text(t) => t.clone(),
        PropertyValue::Number(n) => format!("{:.2}", n),
        PropertyValue::Boolean(b) => b.to_string(),
        PropertyValue::ConceptRef(id) => format!("concept-{}", id.0),
        PropertyValue::EntityRef(id) => format!("entity-{}", id.0),
        PropertyValue::List(items) => format!("[{} items]", items.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn empty_memories() -> MemoryRetrieval {
        MemoryRetrieval {
            episodic: Vec::new(),
            semantic: Vec::new(),
            procedural: Vec::new(),
            associative: Vec::new(),
            relevance_scores: HashMap::new(),
            contradictions: Vec::new(),
        }
    }

    #[test]
    fn test_generate_empty() {
        let mut state = ReasoningState {
            active_hypotheses: Vec::new(),
            conclusion: None,
            premises: Vec::new(),
            evidence_index: HashMap::new(),
            contradiction_log: Vec::new(),
            budget_remaining: 32,
            next_hypothesis_id: HypothesisId(1),
        };
        let memories = empty_memories();
        let hypotheses = generate(&memories, &mut state);
        assert!(hypotheses.is_empty());
    }
}
