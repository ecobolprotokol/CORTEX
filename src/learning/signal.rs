use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct LearningSignal {
    pub magnitude: Scalar,
    pub source: String,
    pub timestamp: u64,
}

pub struct SignalGenerator;

impl SignalGenerator {
    pub fn new() -> Self { Self }

    pub fn generate(&self, prediction_error: Scalar, novelty: Scalar) -> LearningSignal {
        LearningSignal {
            magnitude: prediction_error * 0.7 + novelty * 0.3,
            source: "prediction_error".into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}
