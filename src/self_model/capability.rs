use crate::types::*;

pub fn estimate_capability(
    prediction_error: Scalar,
    memory_pressure: Scalar,
    episode_count: u64,
) -> CapabilityEstimate {
    let learning_factor = (episode_count as f32 / 1000.0).min(1.0);
    CapabilityEstimate {
        language_accuracy: 0.5 + learning_factor * 0.3,
        prediction_accuracy: (1.0 - prediction_error).max(0.0),
        verification_reliability: 0.5 + learning_factor * 0.2,
        planning_success: 0.5 + learning_factor * 0.1,
        memory_retrieval_success: (1.0 - memory_pressure).max(0.0),
        reasoning_consistency: 0.5 + learning_factor * 0.2,
        resource_availability: (1.0 - memory_pressure).max(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_capability() {
        let cap = estimate_capability(0.2, 0.1, 100);
        assert!(cap.language_accuracy > 0.5);
        assert!(cap.prediction_accuracy > 0.7);
        assert!(cap.memory_retrieval_success > 0.8);
    }

    #[test]
    fn test_estimate_capability_low_episodes() {
        let cap = estimate_capability(0.5, 0.5, 0);
        assert!(cap.language_accuracy <= 0.5);
    }
}
