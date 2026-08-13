use crate::types::*;

pub fn select_replay_candidates(experiences: &[Experience], max_count: usize) -> Vec<&Experience> {
    let mut scored: Vec<(usize, Scalar)> = experiences.iter().enumerate()
        .map(|(i, e)| (i, e.error.magnitude * 0.4 + e.observation.importance * 0.3 + 0.3))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.iter().take(max_count).map(|(i, _)| &experiences[*i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_candidates_empty() {
        let result = select_replay_candidates(&[], 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_select_candidates() {
        let experiences: Vec<Experience> = (0..5).map(|i| Experience {
            observation: Observation {
                text: format!("test {}", i),
                source: Provenance::user_provided(),
                timestamp: Timestamp::now(),
                context: ContextState::initial(),
                kind: ObservationKind::UserInput,
                importance: i as f32 * 0.1,
            },
            internal_state: crate::types::StateSnapshot {
                language_vocabulary_size: 0,
                neural_active_cells: 0,
                memory_episode_count: 0,
                world_entity_count: 0,
                reasoning_hypothesis_count: 0,
                timestamp: Timestamp::now(),
            },
            prediction: Prediction {
                target: PredictionTarget::NextState,
                predicted_state: Vec::new(),
                confidence: 0.5,
                timestamp: Timestamp::now(),
                context: ContextState::initial(),
                resolved: false,
                actual: None,
                error: None,
            },
            action: None,
            outcome: None,
            error: PredictionError {
                magnitude: i as f32 * 0.1,
                dimensions: std::collections::HashMap::new(),
                timestamp: Timestamp::now(),
                prediction_id: None,
            },
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::user_provided(),
        }).collect();
        let result = select_replay_candidates(&experiences, 2);
        assert_eq!(result.len(), 2);
    }
}
