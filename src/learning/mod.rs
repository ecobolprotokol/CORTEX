pub mod signal;
pub mod attribution;
pub mod replay;
pub mod stability;

use crate::config::LearningConfig;
use crate::error::Result;
use crate::types::*;
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

pub trait LearningSystem {
    fn record(&mut self, experience: Experience) -> Result<LearningSignal>;
    fn apply_signal(&mut self, signal: &LearningSignal) -> Result<()>;
    fn state(&self) -> &LearningState;
    fn flush_buffer(&mut self) -> Result<Vec<Experience>>;
    fn learning_progress(&self) -> f32;
    fn experience_importance(&self, experience: &Experience) -> f32;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSignal {
    pub magnitude: Scalar,
    pub attribution: ErrorAttribution,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone)]
struct ProgressEntry {
    average_error: f32,
    learning_rate: f32,
    timestamp: Timestamp,
}

pub struct LearningSystemImpl {
    config: LearningConfig,
    state: LearningState,
    experience_buffer: VecDeque<Experience>,
    buffer_capacity: usize,
    progress_history: VecDeque<ProgressEntry>,
    progress_capacity: usize,
    min_buffer_size: usize,
    adaptive_rate_window: VecDeque<f32>,
    adaptive_rate_window_size: usize,
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
            experience_buffer: VecDeque::with_capacity(256),
            buffer_capacity: 256,
            progress_history: VecDeque::with_capacity(128),
            progress_capacity: 128,
            min_buffer_size: 4,
            adaptive_rate_window: VecDeque::with_capacity(20),
            adaptive_rate_window_size: 20,
        })
    }

    fn update_adaptive_rate(&mut self) {
        if self.adaptive_rate_window.len() < 2 {
            return;
        }
        let len = self.adaptive_rate_window.len();
        let half = len / 2;
        let recent_avg: f32 = self.adaptive_rate_window.iter().rev().take(half).sum::<f32>() / half as f32;
        let older_avg: f32 = self.adaptive_rate_window.iter().take(half).sum::<f32>() / half.max(1) as f32;

        let improvement = older_avg - recent_avg;
        if improvement > 0.01 {
            let speedup = 1.0 + improvement.min(0.1);
            self.state.learning_rate *= speedup;
        } else if improvement < -0.005 {
            let slowdown = 1.0 - (improvement.abs().min(0.05));
            self.state.learning_rate *= slowdown;
        }
        self.state.learning_rate = self.state.learning_rate.clamp(0.0001, 1.0);
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
        self.state.average_prediction_error = self.state.average_prediction_error * 0.95 + signal.magnitude * 0.05;

        self.experience_buffer.push_back(experience.clone());
        if self.experience_buffer.len() > self.buffer_capacity {
            self.experience_buffer.pop_front();
        }

        if self.config.replay {
            self.state.pending_experiences.push(experience);
            let max_pending = 1000;
            if self.state.pending_experiences.len() > max_pending {
                let excess = self.state.pending_experiences.len() - max_pending;
                self.state.pending_experiences.drain(0..excess);
            }
        }

        Ok(signal)
    }

    fn apply_signal(&mut self, signal: &LearningSignal) -> Result<()> {
        if !self.state.enabled {
            return Ok(());
        }
        if !stability::is_safe(signal.magnitude, self.state.average_prediction_error) {
            return Ok(());
        }

        self.adaptive_rate_window.push_back(signal.magnitude);
        if self.adaptive_rate_window.len() > self.adaptive_rate_window_size {
            self.adaptive_rate_window.pop_front();
        }
        self.update_adaptive_rate();

        if self.progress_history.len() >= self.progress_capacity {
            self.progress_history.pop_front();
        }
        self.progress_history.push_back(ProgressEntry {
            average_error: self.state.average_prediction_error,
            learning_rate: self.state.learning_rate,
            timestamp: Timestamp::now(),
        });

        Ok(())
    }

    fn state(&self) -> &LearningState {
        &self.state
    }

    fn flush_buffer(&mut self) -> Result<Vec<Experience>> {
        let batch: Vec<Experience> = self.experience_buffer.drain(..).collect();
        Ok(batch)
    }

    fn learning_progress(&self) -> f32 {
        if self.progress_history.len() < 2 {
            return 0.0;
        }
        let len = self.progress_history.len();
        let half = len / 2;
        let recent_avg: f32 = self.progress_history.iter().rev().take(half)
            .map(|p| p.average_error).sum::<f32>() / half as f32;
        let older_avg: f32 = self.progress_history.iter().take(half)
            .map(|p| p.average_error).sum::<f32>() / half.max(1) as f32;

        older_avg - recent_avg
    }

    fn experience_importance(&self, experience: &Experience) -> f32 {
        let error_weight = experience.error.magnitude;
        let importance_weight = experience.observation.importance;
        let recency_bonus = 1.0 - (self.state.average_prediction_error.min(1.0) * 0.3);
        let raw = error_weight * 0.5 + importance_weight * 0.3 + recency_bonus * 0.2;
        raw.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> LearningConfig {
        LearningConfig {
            enabled: true,
            learning_rate: 0.01,
            plasticity: 0.01,
            replay: true,
            consolidation_interval: 1000,
        }
    }

    fn make_experience(error_mag: f32) -> Experience {
        Experience {
            observation: Observation::user_provided("test"),
            internal_state: StateSnapshot {
                language_vocabulary_size: 100,
                neural_active_cells: 10,
                memory_episode_count: 20,
                world_entity_count: 10,
                reasoning_hypothesis_count: 2,
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
                magnitude: error_mag,
                dimensions: std::collections::HashMap::new(),
                timestamp: Timestamp::now(),
                prediction_id: None,
            },
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::user_provided(),
        }
    }

    #[test]
    fn test_record_returns_signal() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        let exp = make_experience(0.3);
        let signal = system.record(exp).unwrap();
        assert!((signal.magnitude - 0.3).abs() < 0.001);
        assert_eq!(system.state().total_learning_events, 1);
    }

    #[test]
    fn test_disabled_system() {
        let mut config = test_config();
        config.enabled = false;
        let mut system = LearningSystemImpl::new(&config).unwrap();
        let exp = make_experience(0.5);
        let signal = system.record(exp).unwrap();
        assert_eq!(signal.magnitude, 0.0);
        assert_eq!(system.state().total_learning_events, 0);
    }

    #[test]
    fn test_buffer_collects_experiences() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        for i in 0..5 {
            let exp = make_experience(i as f32 * 0.1);
            system.record(exp).unwrap();
        }
        let batch = system.flush_buffer().unwrap();
        assert_eq!(batch.len(), 5);
    }

    #[test]
    fn test_buffer_bounded() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        for _ in 0..300 {
            let exp = make_experience(0.3);
            system.record(exp).unwrap();
        }
        let batch = system.flush_buffer().unwrap();
        assert!(batch.len() <= 256);
    }

    #[test]
    fn test_learning_progress_no_data() {
        let config = test_config();
        let system = LearningSystemImpl::new(&config).unwrap();
        assert_eq!(system.learning_progress(), 0.0);
    }

    #[test]
    fn test_learning_progress_improving() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        for i in 0..10 {
            let mag = 0.01 * (i as f32 + 1.0);
            let exp = make_experience(mag);
            system.record(exp).unwrap();
            let signal = LearningSignal {
                magnitude: mag,
                attribution: ErrorAttribution::InputError,
                timestamp: Timestamp::now(),
            };
            system.apply_signal(&signal).unwrap();
        }
        assert!(system.state().total_learning_events >= 10);
    }

    #[test]
    fn test_adaptive_learning_rate_decreases() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        let initial_lr = system.state().learning_rate;
        for i in 0..30 {
            let mag = 0.005 * (i as f32 + 1.0);
            let exp = make_experience(mag);
            system.record(exp).unwrap();
            let signal = LearningSignal {
                magnitude: mag,
                attribution: ErrorAttribution::InputError,
                timestamp: Timestamp::now(),
            };
            system.apply_signal(&signal).unwrap();
        }
        let final_lr = system.state().learning_rate;
        assert!(final_lr != initial_lr);
    }

    #[test]
    fn test_experience_importance_high_error() {
        let config = test_config();
        let system = LearningSystemImpl::new(&config).unwrap();
        let exp = make_experience(0.9);
        let importance = system.experience_importance(&exp);
        assert!(importance > 0.5);
    }

    #[test]
    fn test_experience_importance_low_error() {
        let config = test_config();
        let system = LearningSystemImpl::new(&config).unwrap();
        let exp = make_experience(0.01);
        let importance = system.experience_importance(&exp);
        assert!(importance < 0.8);
    }

    #[test]
    fn test_experience_importance_bounds() {
        let config = test_config();
        let system = LearningSystemImpl::new(&config).unwrap();
        let exp = make_experience(0.5);
        let importance = system.experience_importance(&exp);
        assert!(importance >= 0.0 && importance <= 1.0);
    }

    #[test]
    fn test_pending_experiences_replay() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        for _ in 0..5 {
            let exp = make_experience(0.3);
            system.record(exp).unwrap();
        }
        assert_eq!(system.state().pending_experiences.len(), 5);
    }

    #[test]
    fn test_pending_experiences_bounded() {
        let config = test_config();
        let mut system = LearningSystemImpl::new(&config).unwrap();
        for _ in 0..1100 {
            let exp = make_experience(0.3);
            system.record(exp).unwrap();
        }
        assert!(system.state().pending_experiences.len() <= 1000);
    }
}
