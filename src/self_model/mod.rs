pub mod capability;

use crate::error::Result;
use crate::types::*;

pub trait SelfModelInterface {
    fn estimate(&self) -> &SelfModel;
    fn update(&mut self, metrics: &ModelMetrics);
}

pub struct ModelMetrics {
    pub prediction_error: Scalar,
    pub memory_pressure: Scalar,
    pub episode_count: u64,
}

pub struct SelfModelImpl {
    model: SelfModel,
}

impl SelfModelImpl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            model: SelfModel {
                capabilities: CapabilityEstimate {
                    language_accuracy: 0.5,
                    prediction_accuracy: 0.5,
                    verification_reliability: 0.5,
                    planning_success: 0.5,
                    memory_retrieval_success: 0.5,
                    reasoning_consistency: 0.5,
                    resource_availability: 1.0,
                },
                limitations: Limitations {
                    known_limitations: Vec::new(),
                    resource_constraints: Vec::new(),
                    capability_gaps: Vec::new(),
                },
                prediction_accuracy: 0.5,
                uncertainty_level: 0.5,
                memory_health: MemoryHealth {
                    pressure: MemoryPressure::Low,
                    fragmentation: 0.0,
                    consolidation_backlog: 0,
                },
                last_updated: Timestamp::now(),
            },
        })
    }
}

impl SelfModelInterface for SelfModelImpl {
    fn estimate(&self) -> &SelfModel {
        &self.model
    }

    fn update(&mut self, metrics: &ModelMetrics) {
        self.model.prediction_accuracy = (self.model.prediction_accuracy * 0.9 + (1.0 - metrics.prediction_error) * 0.1);
        self.model.uncertainty_level = metrics.prediction_error;
        self.model.last_updated = Timestamp::now();
    }
}
