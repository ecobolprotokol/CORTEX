pub mod capability;

use crate::error::Result;
use crate::types::*;

pub trait SelfModelInterface {
    fn estimate(&self) -> &SelfModel;
    fn update(&mut self, metrics: &ModelMetrics);
    fn update_memory_health(&mut self, pressure: f32, fragmentation: f32, backlog: u32);
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
        self.model.prediction_accuracy =
            self.model.prediction_accuracy * 0.9 + (1.0 - metrics.prediction_error).max(0.0) * 0.1;
        self.model.uncertainty_level = metrics.prediction_error.clamp(0.0, 1.0);

        self.model.capabilities.prediction_accuracy =
            self.model.capabilities.prediction_accuracy * 0.95 + self.model.prediction_accuracy * 0.05;

        self.model.memory_health.consolidation_backlog =
            (metrics.episode_count as f32 * 0.01) as u32;

        let pressure_level = if metrics.memory_pressure > 0.9 {
            MemoryPressure::Critical
        } else if metrics.memory_pressure > 0.7 {
            MemoryPressure::High
        } else if metrics.memory_pressure > 0.4 {
            MemoryPressure::Moderate
        } else {
            MemoryPressure::Low
        };
        self.model.memory_health.pressure = pressure_level;

        if metrics.memory_pressure > 0.8 {
            self.model.capabilities.resource_availability =
                (self.model.capabilities.resource_availability * 0.9 + 0.1 * (1.0 - metrics.memory_pressure)).max(0.0);
            if !self.model.limitations.resource_constraints.contains(&"high_memory_pressure".to_string()) {
                self.model.limitations.resource_constraints.push("high_memory_pressure".to_string());
            }
        } else {
            self.model.limitations.resource_constraints.retain(|s| s != "high_memory_pressure");
        }

        self.model.last_updated = Timestamp::now();
    }

    fn update_memory_health(&mut self, pressure: f32, fragmentation: f32, backlog: u32) {
        let pressure_level = if pressure > 0.9 {
            MemoryPressure::Critical
        } else if pressure > 0.7 {
            MemoryPressure::High
        } else if pressure > 0.4 {
            MemoryPressure::Moderate
        } else {
            MemoryPressure::Low
        };
        self.model.memory_health.pressure = pressure_level;
        self.model.memory_health.fragmentation = fragmentation.clamp(0.0, 1.0);
        self.model.memory_health.consolidation_backlog = backlog;
        self.model.last_updated = Timestamp::now();
    }
}
