pub mod capability;

pub use capability::{SelfModel, CapabilityAssessment};

use crate::error::CortexError;
use crate::types::scalars::Scalar;

pub trait SelfModelInterface {
    fn assess_capabilities(&self) -> Result<CapabilityAssessment, CortexError>;
    fn prediction_accuracy(&self) -> f32;
}

pub struct SelfModelManager {
    model: SelfModel,
    accuracy_history: Vec<Scalar>,
    capability_history: Vec<CapabilityAssessment>,
}

impl SelfModelManager {
    pub fn new() -> Self {
        Self {
            model: SelfModel::new(),
            accuracy_history: Vec::new(),
            capability_history: Vec::new(),
        }
    }

    pub fn update_from_experience(
        &mut self,
        prediction_correct: bool,
        context: &str,
    ) {
        let accuracy_delta = if prediction_correct { 0.01 } else { -0.02 };
        self.model.prediction_accuracy =
            (self.model.prediction_accuracy + accuracy_delta).clamp(0.0, 1.0);

        self.accuracy_history.push(self.model.prediction_accuracy);
        if self.accuracy_history.len() > 1000 {
            self.accuracy_history.remove(0);
        }

        if self.accuracy_history.len() % 100 == 0 {
            self.update_capabilities();
        }
    }

    fn update_capabilities(&mut self) {
        let recent: Vec<Scalar> = self
            .accuracy_history
            .iter()
            .rev()
            .take(100)
            .cloned()
            .collect();

        if recent.is_empty() {
            return;
        }

        let avg = recent.iter().sum::<Scalar>() / recent.len() as Scalar;
        let variance = recent
            .iter()
            .map(|x| (x - avg).powi(2))
            .sum::<Scalar>()
            / recent.len() as Scalar;

        self.model.capabilities.prediction_accuracy = avg;
        self.model.capabilities.verification_reliability = (avg * 0.9).min(1.0);
        self.model.capabilities.reasoning_consistency = (1.0 - variance).max(0.0);

        self.model.uncertainty_level = variance.sqrt();
        self.model.prediction_accuracy = avg;

        self.capability_history.push(self.model.capabilities.clone());
        if self.capability_history.len() > 100 {
            self.capability_history.remove(0);
        }
    }

    pub fn get_model(&self) -> &SelfModel {
        &self.model
    }

    pub fn get_mut_model(&mut self) -> &mut SelfModel {
        &mut self.model
    }

    pub fn accuracy_trend(&self) -> Scalar {
        if self.accuracy_history.len() < 10 {
            return 0.0;
        }

        let recent: Vec<Scalar> = self
            .accuracy_history
            .iter()
            .rev()
            .take(10)
            .cloned()
            .collect();
        let older: Vec<Scalar> = self
            .accuracy_history
            .iter()
            .rev()
            .skip(10)
            .take(10)
            .cloned()
            .collect();

        let recent_avg = recent.iter().sum::<Scalar>() / recent.len() as Scalar;
        let older_avg = if older.is_empty() {
            recent_avg
        } else {
            older.iter().sum::<Scalar>() / older.len() as Scalar
        };

        recent_avg - older_avg
    }

    pub fn capability_trend(&self) -> Vec<(String, Scalar)> {
        if self.capability_history.len() < 2 {
            return Vec::new();
        }

        let current = &self.capability_history[self.capability_history.len() - 1];
        let previous = &self.capability_history[self.capability_history.len() - 2];

        vec![
            (
                "prediction_accuracy".into(),
                current.prediction_accuracy - previous.prediction_accuracy,
            ),
            (
                "verification_reliability".into(),
                current.verification_reliability - previous.verification_reliability,
            ),
            (
                "reasoning_consistency".into(),
                current.reasoning_consistency - previous.reasoning_consistency,
            ),
        ]
    }
}

impl Default for SelfModelManager {
    fn default() -> Self {
        Self::new()
    }
}
