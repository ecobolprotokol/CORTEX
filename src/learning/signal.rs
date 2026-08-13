use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct LearningSignal {
    pub magnitude: Scalar,
    pub source: String,
    pub timestamp: u64,
    pub priority: Scalar,
    pub signal_type: SignalType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType {
    PredictionError,
    NoveltyDetection,
    ContradictionResolution,
    MemoryConsolidation,
    RuleRefinement,
    ConfidenceAdjustment,
}

pub struct SignalGenerator {
    pub error_weight: Scalar,
    pub novelty_weight: Scalar,
    pub contradiction_weight: Scalar,
    pub decay_factor: Scalar,
}

impl SignalGenerator {
    pub fn new() -> Self {
        Self {
            error_weight: 0.7,
            novelty_weight: 0.3,
            contradiction_weight: 0.5,
            decay_factor: 0.95,
        }
    }

    pub fn generate(&self, prediction_error: Scalar, novelty: Scalar) -> LearningSignal {
        let magnitude = prediction_error * self.error_weight + novelty * self.novelty_weight;
        let priority = (prediction_error * 0.6 + novelty * 0.4).min(1.0);

        let signal_type = if prediction_error > 0.5 {
            SignalType::PredictionError
        } else if novelty > 0.5 {
            SignalType::NoveltyDetection
        } else {
            SignalType::ConfidenceAdjustment
        };

        LearningSignal {
            magnitude,
            source: "prediction_error".into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            priority,
            signal_type,
        }
    }

    pub fn generate_from_error(&self, error_magnitude: Scalar) -> LearningSignal {
        let magnitude = error_magnitude * self.error_weight;
        LearningSignal {
            magnitude,
            source: "direct_error".into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            priority: error_magnitude,
            signal_type: SignalType::PredictionError,
        }
    }

    pub fn generate_consolidation_signal(
        &self,
        memory_age: Scalar,
        access_frequency: Scalar,
        importance: Scalar,
    ) -> LearningSignal {
        let magnitude = (importance * 0.4 + access_frequency * 0.3 + (1.0 - memory_age) * 0.3)
            .min(1.0);

        LearningSignal {
            magnitude,
            source: "consolidation".into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            priority: magnitude,
            signal_type: SignalType::MemoryConsolidation,
        }
    }

    pub fn apply_decay(&self, signal: &mut LearningSignal, elapsed_steps: u32) {
        let decay = self.decay_factor.powi(elapsed_steps as i32);
        signal.magnitude *= decay;
        signal.priority *= decay;
    }

    pub fn aggregate_signals(&self, signals: &[LearningSignal]) -> LearningSignal {
        if signals.is_empty() {
            return self.generate(0.0, 0.0);
        }

        let total_magnitude: Scalar = signals.iter().map(|s| s.magnitude).sum();
        let max_priority: Scalar = signals
            .iter()
            .map(|s| s.priority)
            .fold(0.0f32, Scalar::max);
        let avg_magnitude = total_magnitude / signals.len() as Scalar;

        LearningSignal {
            magnitude: avg_magnitude,
            source: "aggregated".into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            priority: max_priority,
            signal_type: SignalType::PredictionError,
        }
    }
}

impl Default for SignalGenerator {
    fn default() -> Self {
        Self::new()
    }
}
