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
