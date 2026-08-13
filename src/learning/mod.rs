pub mod signal;
pub mod attribution;
pub mod replay;
pub mod stability;

pub use signal::{SignalGenerator, LearningSignal};
pub use attribution::{AttributionEngine, ErrorSource, ErrorAttribution};
pub use replay::ReplayBuffer;
pub use stability::StabilityGuard;

use crate::error::CortexError;
use crate::types::scalars::Scalar;
use crate::types::observation::{PredictionError, Experience};

pub trait LearningSystem {
    fn record_experience(&mut self, experience: &str) -> Result<(), CortexError>;
    fn apply_signal(&mut self, signal: &LearningSignal) -> Result<(), CortexError>;
    fn learning_rate(&self) -> f32;
}

pub struct LearningPipeline {
    signal_generator: SignalGenerator,
    attribution_engine: AttributionEngine,
    stability_guard: StabilityGuard,
    learning_rate: Scalar,
    total_learning_events: u64,
}

impl LearningPipeline {
    pub fn new(learning_rate: Scalar, plasticity_bound: Scalar) -> Self {
        Self {
            signal_generator: SignalGenerator::new(),
            attribution_engine: AttributionEngine::new(),
            stability_guard: StabilityGuard::new(plasticity_bound, plasticity_bound),
            learning_rate,
            total_learning_events: 0,
        }
    }

    pub fn process_prediction_error(
        &mut self,
        error: &PredictionError,
        context: &str,
    ) -> Option<LearningSignal> {
        if error.is_zero() {
            return None;
        }

        let novelty = self.assess_novelty(context);
        let signal = self.signal_generator.generate(error.magnitude, novelty);

        let attribution = self.attribution_engine.attribute(error.magnitude, context);
        tracing::debug!(
            "Learning signal: magnitude={:.4}, source={:?}",
            signal.magnitude,
            attribution.source
        );

        let clamped_magnitude = self.stability_guard.clamp_update(signal.magnitude * self.learning_rate);

        self.total_learning_events += 1;

        Some(LearningSignal {
            magnitude: clamped_magnitude,
            source: signal.source,
            timestamp: signal.timestamp,
            priority: signal.priority,
            signal_type: signal.signal_type,
        })
    }

    fn assess_novelty(&self, context: &str) -> Scalar {
        let word_count = context.split_whitespace().count();
        let unique_ratio = {
            let words: Vec<&str> = context.split_whitespace().collect();
            let unique: std::collections::HashSet<&str> = words.iter().cloned().collect();
            if words.is_empty() {
                0.0
            } else {
                unique.len() as Scalar / words.len() as Scalar
            }
        };
        ((word_count as Scalar * 0.05 + unique_ratio * 0.5).min(1.0))
    }

    pub fn total_events(&self) -> u64 {
        self.total_learning_events
    }
}
