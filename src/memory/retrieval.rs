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
                    let relevance = score_association(a, context);
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

fn score_episode(episode: &Episode, query_text: &str, context: &ContextState) -> Scalar {
    let text_overlap = compute_text_overlap(&episode.observation.text, query_text);
    let recency = compute_recency(episode.timestamp);
    let importance = episode.importance;
    let confidence = episode.confidence.overall();
    let popularity = (episode.retrieval_count.min(20) as Scalar) / 20.0;
    let context_bonus = compute_context_similarity(&episode.context, context);

    text_overlap * 0.30
        + recency * 0.20
        + importance * 0.15
        + confidence * 0.15
        + popularity * 0.10
        + context_bonus * 0.10
}

fn score_knowledge(knowledge: &Knowledge, query_text: &str, context: &ContextState) -> Scalar {
    let property_text = knowledge.properties.iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let text_overlap = compute_text_overlap(&property_text, query_text);
    let confidence = knowledge.confidence.overall();
    let stability = if knowledge.confirmation_count + knowledge.contradiction_count > 0 {
        knowledge.confirmation_count as Scalar / (knowledge.confirmation_count + knowledge.contradiction_count) as Scalar
    } else {
        0.5
    };
    let recency = compute_recency(knowledge.updated_at);
    let context_bonus = if context.active_concepts.iter().any(|c| *c == knowledge.concept) {
        0.15
    } else {
        0.0
    };

    text_overlap * 0.25
        + confidence * 0.25
        + stability * 0.20
        + recency * 0.15
        + context_bonus
}

fn score_procedure(procedure: &Procedure, query_text: &str, context: &ContextState) -> Scalar {
    let text_overlap = compute_text_overlap(&procedure.condition.description, query_text);
    let success_rate = if procedure.success_count + procedure.failure_count > 0 {
        procedure.success_count as Scalar / (procedure.success_count + procedure.failure_count) as Scalar
    } else {
        0.5
    };
    let confidence = procedure.confidence;
    let recency = procedure.last_used.map(compute_recency).unwrap_or(0.0);
    let context_bonus = compute_procedure_context_bonus(procedure, context);

    text_overlap * 0.30
        + success_rate * 0.25
        + confidence * 0.20
        + recency * 0.10
        + context_bonus * 0.15
}

fn score_association(association: &Association, context: &ContextState) -> Scalar {
    let strength = association.strength;
    let confidence = association.confidence;
    let activation = (association.activation_count.min(20) as Scalar) / 20.0;
    let recency = compute_recency(association.last_strengthened);
    let context_bonus = compute_association_context_bonus(association, context);

    strength * 0.30
        + confidence * 0.25
        + activation * 0.15
        + recency * 0.15
        + context_bonus * 0.15
}

fn compute_text_overlap(text: &str, query: &str) -> Scalar {
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    let text_words: Vec<&str> = text_lower.split_whitespace().collect();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    if query_words.is_empty() || text_words.is_empty() {
        return 0.0;
    }

    let mut match_count = 0;
    for qw in &query_words {
        if text_words.iter().any(|tw| tw.contains(qw) || qw.contains(tw)) {
            match_count += 1;
        }
    }

    match_count as Scalar / query_words.len() as Scalar
}

fn compute_recency(timestamp: Timestamp) -> Scalar {
    let age_hours = (Timestamp::now().0.saturating_sub(timestamp.0)) as Scalar / 3_600_000.0;
    1.0 / (1.0 + age_hours)
}

fn compute_context_similarity(a: &ContextState, b: &ContextState) -> Scalar {
    if a.active_concepts.is_empty() && b.active_concepts.is_empty() {
        return 0.0;
    }
    let overlap = a.active_concepts.iter()
        .filter(|c| b.active_concepts.contains(c))
        .count() as Scalar;
    let total = a.active_concepts.len().max(b.active_concepts.len()) as Scalar;
    if total == 0.0 { 0.0 } else { overlap / total }
}

fn compute_procedure_context_bonus(procedure: &Procedure, context: &ContextState) -> Scalar {
    let mut bonus = 0.0;
    if procedure.condition.required_concepts.iter().any(|c| context.active_concepts.contains(c)) {
        bonus += 0.1;
    }
    if procedure.condition.required_entities.iter().any(|e| context.world_assumptions.contains(e)) {
        bonus += 0.05;
    }
    bonus
}

fn compute_association_context_bonus(association: &Association, context: &ContextState) -> Scalar {
    let source_match = match association.source {
        InternalId::Concept(c) => context.active_concepts.contains(&c),
        InternalId::Entity(e) => context.world_assumptions.contains(&e),
        _ => false,
    };
    let target_match = match association.target {
        InternalId::Concept(c) => context.active_concepts.contains(&c),
        InternalId::Entity(e) => context.world_assumptions.contains(&e),
        _ => false,
    };
    match (source_match, target_match) {
        (true, true) => 0.15,
        (true, false) | (false, true) => 0.08,
        (false, false) => 0.0,
    }
}

fn detect_contradictions(retrieval: &mut MemoryRetrieval) {
    for (i, ek) in retrieval.semantic.iter().enumerate() {
        for j in (i + 1)..retrieval.semantic.len() {
            let ek2 = &retrieval.semantic[j];
            let same_concept = ek.knowledge.concept == ek2.knowledge.concept;
            let status_conflict = (ek.knowledge.verification_status == VerificationStatus::Verified
                && ek2.knowledge.verification_status == VerificationStatus::Contradicted)
                || (ek.knowledge.verification_status == VerificationStatus::Contradicted
                    && ek2.knowledge.verification_status == VerificationStatus::Verified);
            let relation_conflict = ek.knowledge.relations.iter().any(|ra| {
                ek2.knowledge.relations.iter().any(|rb| {
                    ra.source == rb.source
                        && ra.target == rb.target
                        && ((ra.kind == RelationKind::Supports && rb.kind == RelationKind::Contradicts)
                            || (ra.kind == RelationKind::Contradicts && rb.kind == RelationKind::Supports))
                })
            });
            let property_conflict = ek.knowledge.properties.iter().any(|pa| {
                ek2.knowledge.properties.iter().any(|pb| {
                    pa.name == pb.name && match (&pa.value, &pb.value) {
                        (PropertyValue::Boolean(a), PropertyValue::Boolean(b)) => a != b,
                        (PropertyValue::Number(a), PropertyValue::Number(b)) => (a - b).abs() > 0.01,
                        (PropertyValue::Text(a), PropertyValue::Text(b)) => a != b && (pa.confidence - pb.confidence).abs() > 0.3,
                        _ => false,
                    }
                })
            });

            if same_concept && (status_conflict || relation_conflict || property_conflict) {
                let severity = if status_conflict { 0.8 } else if relation_conflict { 0.6 } else { 0.4 };
                retrieval.contradictions.push(Contradiction {
                    claim_a: HypothesisId(ek.knowledge.id.0),
                    claim_b: HypothesisId(ek2.knowledge.id.0),
                    description: format!(
                        "Contradictory knowledge about concept {:?}: status={}, relations={}, properties={}",
                        ek.knowledge.concept, status_conflict, relation_conflict, property_conflict
                    ),
                    severity,
                    detected_at: Timestamp::now(),
                    resolved: false,
                });
            }
        }
    }

    for (i, sp) in retrieval.procedural.iter().enumerate() {
        for j in (i + 1)..retrieval.procedural.len() {
            let sp2 = &retrieval.procedural[j];
            if sp.procedure.condition.description == sp2.procedure.condition.description {
                let sr1 = if sp.procedure.success_count + sp.procedure.failure_count > 0 {
                    sp.procedure.success_count as Scalar / (sp.procedure.success_count + sp.procedure.failure_count) as Scalar
                } else {
                    0.5
                };
                let sr2 = if sp2.procedure.success_count + sp2.procedure.failure_count > 0 {
                    sp2.procedure.success_count as Scalar / (sp2.procedure.success_count + sp2.procedure.failure_count) as Scalar
                } else {
                    0.5
                };
                if (sr1 - sr2).abs() > 0.3 {
                    retrieval.contradictions.push(Contradiction {
                        claim_a: HypothesisId(sp.procedure.id.0),
                        claim_b: HypothesisId(sp2.procedure.id.0),
                        description: format!(
                            "Procedures with same condition have divergent success rates: {:.1} vs {:.1}",
                            sr1, sr2
                        ),
                        severity: 0.5,
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

    fn make_episode(text: &str, importance: f32) -> Episode {
        Episode {
            id: EpisodeId(1),
            observation: Observation::user_provided(text),
            context: ContextState::initial(),
            action: None,
            outcome: None,
            timestamp: Timestamp::now(),
            prediction: None,
            prediction_error: PredictionError::zero(),
            confidence: ConfidenceState::default(),
            source: Provenance::user_provided(),
            importance,
            retrieval_count: 0,
            last_retrieved: None,
            consolidated: false,
        }
    }

    fn make_knowledge(concept: ConceptId, confidence: f32) -> Knowledge {
        Knowledge {
            id: KnowledgeId(1),
            concept,
            properties: vec![Property {
                name: "test".into(),
                value: PropertyValue::Text("test value".into()),
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

    fn make_procedure(description: &str, success: u64, failure: u64) -> Procedure {
        Procedure {
            id: ProcedureId(1),
            condition: Condition {
                description: description.into(),
                required_concepts: Vec::new(),
                required_entities: Vec::new(),
                required_context: None,
            },
            steps: Vec::new(),
            expected_outcome: Outcome {
                success: true,
                description: "test".into(),
                result: None,
                timestamp: Timestamp::now(),
                confidence: 0.8,
            },
            success_count: success,
            failure_count: failure,
            confidence: 0.5,
            context_requirements: ContextRequirements {
                requires_world_model: false,
                requires_memory: false,
                requires_reasoning: false,
                max_context_tokens: 1024,
            },
            risk: RiskAssessment::default(),
            provenance: Provenance::user_provided(),
            created_at: Timestamp::now(),
            last_used: None,
        }
    }

    fn make_memory_state() -> MemoryState {
        MemoryState {
            working: WorkingMemory {
                input: None,
                conversation_context: ConversationContext {
                    session_id: SessionId(1),
                    turn_count: 0,
                    recent_inputs: Vec::new(),
                    recent_outputs: Vec::new(),
                    started_at: Timestamp::now(),
                },
                active_concepts: Vec::new(),
                active_hypotheses: Vec::new(),
                goals: Vec::new(),
                reasoning_state: None,
                world_assumptions: Vec::new(),
                generation_state: None,
            },
            episodic: EpisodicMemory {
                episodes: Vec::new(),
                capacity_bytes: 1024 * 1024,
                current_usage_bytes: 0,
                next_id: EpisodeId(1),
            },
            semantic: SemanticMemory {
                knowledge: Vec::new(),
                capacity_bytes: 1024 * 1024,
                current_usage_bytes: 0,
                next_id: KnowledgeId(1),
            },
            procedural: ProceduralMemory {
                procedures: Vec::new(),
                capacity_bytes: 1024 * 1024,
                current_usage_bytes: 0,
                next_id: ProcedureId(1),
            },
            associative: AssociativeMemory {
                associations: Vec::new(),
                capacity_bytes: 1024 * 1024,
                current_usage_bytes: 0,
                next_id: AssociationId(1),
            },
        }
    }

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

    #[test]
    fn test_text_overlap_case_insensitive() {
        let score = compute_text_overlap("Gravity Is A Force", "gravity force");
        assert!(score > 0.5);
    }

    #[test]
    fn test_recency() {
        let recent = compute_recency(Timestamp::now());
        assert!(recent > 0.9);

        let old = Timestamp(Timestamp::now().0 - 7_200_000);
        let old_score = compute_recency(old);
        assert!(old_score < recent);
    }

    #[test]
    fn test_context_similarity() {
        let mut a = ContextState::initial();
        a.active_concepts = vec![ConceptId(1), ConceptId(2)];
        let mut b = ContextState::initial();
        b.active_concepts = vec![ConceptId(2), ConceptId(3)];
        let sim = compute_context_similarity(&a, &b);
        assert!((sim - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_retrieve_episodic() {
        let mut state = make_memory_state();
        state.episodic.episodes.push(make_episode("gravity is a force", 0.8));
        state.episodic.episodes.push(make_episode("water boils at 100C", 0.5));

        let query = MemoryQuery {
            query_type: MemoryQueryType::Episodic,
            text: "gravity".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert!(!result.episodic.is_empty());
        assert!(result.episodic[0].episode.observation.text.contains("gravity"));
        assert!(result.episodic[0].relevance_score >= result.episodic.last().unwrap().relevance_score);
    }

    #[test]
    fn test_retrieve_semantic() {
        let mut state = make_memory_state();
        state.semantic.knowledge.push(make_knowledge(ConceptId(1), 0.8));
        state.semantic.knowledge.push(make_knowledge(ConceptId(2), 0.5));

        let query = MemoryQuery {
            query_type: MemoryQueryType::Semantic,
            text: "test".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert_eq!(result.semantic.len(), 2);
    }

    #[test]
    fn test_retrieve_max_results() {
        let mut state = make_memory_state();
        for i in 0..10 {
            state.episodic.episodes.push(make_episode("test query", 0.5));
            state.episodic.episodes.last_mut().unwrap().id = EpisodeId(i);
        }

        let query = MemoryQuery {
            query_type: MemoryQueryType::Episodic,
            text: "test query".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 3,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert_eq!(result.episodic.len(), 3);
    }

    #[test]
    fn test_contradiction_detection_status() {
        let mut state = make_memory_state();
        let mut k1 = make_knowledge(ConceptId(1), 0.9);
        k1.id = KnowledgeId(1);
        k1.verification_status = VerificationStatus::Verified;
        let mut k2 = make_knowledge(ConceptId(1), 0.2);
        k2.id = KnowledgeId(2);
        k2.verification_status = VerificationStatus::Contradicted;

        state.semantic.knowledge.push(k1);
        state.semantic.knowledge.push(k2);

        let query = MemoryQuery {
            query_type: MemoryQueryType::Semantic,
            text: "test".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert_eq!(result.contradictions.len(), 1);
        assert!(result.contradictions[0].severity > 0.5);
    }

    #[test]
    fn test_contradiction_detection_relation() {
        let mut state = make_memory_state();
        let mut k1 = make_knowledge(ConceptId(1), 0.9);
        k1.id = KnowledgeId(1);
        k1.relations.push(Relation {
            id: RelationId(1),
            kind: RelationKind::Supports,
            source: InternalId::Concept(ConceptId(1)),
            target: InternalId::Concept(ConceptId(2)),
            confidence: 0.9,
            provenance: Provenance::user_provided(),
        });
        let mut k2 = make_knowledge(ConceptId(1), 0.8);
        k2.id = KnowledgeId(2);
        k2.relations.push(Relation {
            id: RelationId(2),
            kind: RelationKind::Contradicts,
            source: InternalId::Concept(ConceptId(1)),
            target: InternalId::Concept(ConceptId(2)),
            confidence: 0.8,
            provenance: Provenance::user_provided(),
        });

        state.semantic.knowledge.push(k1);
        state.semantic.knowledge.push(k2);

        let query = MemoryQuery {
            query_type: MemoryQueryType::Semantic,
            text: "test".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert_eq!(result.contradictions.len(), 1);
    }

    #[test]
    fn test_contradiction_detection_procedure() {
        let mut state = make_memory_state();
        let mut p1 = make_procedure("cook rice", 10, 1);
        p1.id = ProcedureId(1);
        let mut p2 = make_procedure("cook rice", 1, 10);
        p2.id = ProcedureId(2);

        state.procedural.procedures.push(p1);
        state.procedural.procedures.push(p2);

        let query = MemoryQuery {
            query_type: MemoryQueryType::Procedural,
            text: "cook rice".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let context = ContextState::initial();
        let result = retrieve(&state, &query, &context).unwrap();

        assert_eq!(result.contradictions.len(), 1);
    }

    #[test]
    fn test_popularity_boost() {
        let mut state = make_memory_state();
        let mut ep1 = make_episode("test query", 0.5);
        ep1.retrieval_count = 0;
        let mut ep2 = make_episode("test query", 0.5);
        ep2.retrieval_count = 20;

        state.episodic.episodes.push(ep1);
        state.episodic.episodes.push(ep2);

        let score1 = score_episode(&state.episodic.episodes[0], "test query", &ContextState::initial());
        let score2 = score_episode(&state.episodic.episodes[1], "test query", &ContextState::initial());
        assert!(score2 > score1);
    }

    #[test]
    fn test_context_bonus() {
        let mut state = make_memory_state();
        state.semantic.knowledge.push(make_knowledge(ConceptId(1), 0.8));

        let query = MemoryQuery {
            query_type: MemoryQueryType::Semantic,
            text: "test".into(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };

        let context_no_match = ContextState::initial();
        let result_no = retrieve(&state, &query, &context_no_match).unwrap();

        let mut context_with_match = ContextState::initial();
        context_with_match.active_concepts = vec![ConceptId(1)];
        let result_yes = retrieve(&state, &query, &context_with_match).unwrap();

        assert!(result_yes.semantic[0].relevance_score > result_no.semantic[0].relevance_score);
    }
}
