pub mod signal;
pub mod attribution;
pub mod replay;
pub mod stability;

use crate::config::LearningConfig;
use crate::error::Result;
use crate::types::*;

pub trait LearningSystem {
    fn record(&mut self, experience: Experience) -> Result<LearningSignal>;
    fn apply_signal(&mut self, signal: &LearningSignal) -> Result<()>;
    fn state(&self) -> &LearningState;
}

#[derive(Debug, Clone)]
pub struct LearningSignal {
    pub magnitude: Scalar,
    pub attribution: ErrorAttribution,
    pub timestamp: Timestamp,
}

pub struct LearningSystemImpl {
    config: LearningConfig,
    state: LearningState,
}

impl LearningSystemImpl {
    pub fn new(config: &LearningConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: LearningState {
                enabled: config.enabled,
                total_learning_events: 0,
                total_replay_events: 0,
                total_consolidation_events: 0,
                average_prediction_error: 0.0,
                learning_rate: config.learning_rate,
                plasticity_rate: config.plasticity,
                next_consolidation_at: config.consolidation_interval,
                pending_experiences: Vec::new(),
            },
        })
    }
}

impl LearningSystem for LearningSystemImpl {
    fn record(&mut self, experience: Experience) -> Result<LearningSignal> {
        if !self.state.enabled {
            return Ok(LearningSignal {
                magnitude: 0.0,
                attribution: experience.attribution,
                timestamp: Timestamp::now(),
            });
        }
        let signal = signal::compute(&experience);
        self.state.total_learning_events += 1;
        self.state.average_prediction_error = (self.state.average_prediction_error * 0.95 + signal.magnitude * 0.05);
        Ok(signal)
    }

    fn apply_signal(&mut self, signal: &LearningSignal) -> Result<()> {
        if !self.state.enabled {
            return Ok(());
        }
        if !stability::is_safe(signal.magnitude, self.state.average_prediction_error) {
            return Ok(());
        }
        Ok(())
    }

    fn state(&self) -> &LearningState {
        &self.state
    }
}
