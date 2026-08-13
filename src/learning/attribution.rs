use crate::types::*;

pub fn attribute(experience: &Experience) -> ErrorAttribution {
    experience.attribution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute() {
        let experience = Experience {
            observation: Observation::user_provided("test"),
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
            error: PredictionError::zero(),
            attribution: ErrorAttribution::MemoryError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::user_provided(),
        };
        assert_eq!(attribute(&experience), ErrorAttribution::MemoryError);
    }
}
