use crate::types::*;
use crate::learning::LearningSignal;

pub fn compute(experience: &Experience) -> LearningSignal {
    let magnitude = experience.error.magnitude;
    let attribution = experience.attribution;
    LearningSignal {
        magnitude,
        attribution,
        timestamp: Timestamp::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_signal() {
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
            error: PredictionError {
                magnitude: 0.3,
                dimensions: std::collections::HashMap::new(),
                timestamp: Timestamp::now(),
                prediction_id: None,
            },
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::user_provided(),
        };
        let signal = compute(&experience);
        assert!((signal.magnitude - 0.3).abs() < 0.001);
        assert_eq!(signal.attribution, ErrorAttribution::InputError);
    }
}
