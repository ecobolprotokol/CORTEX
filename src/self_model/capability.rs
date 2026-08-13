use crate::types::scalars::Scalar;
use crate::types::state::CapabilitySet;

#[derive(Debug, Clone)]
pub struct CapabilityAssessment {
    pub language_accuracy: Scalar,
    pub prediction_accuracy: Scalar,
    pub verification_reliability: Scalar,
    pub planning_success: Scalar,
    pub memory_retrieval_success: Scalar,
    pub reasoning_consistency: Scalar,
    pub resource_availability: Scalar,
    pub overall: Scalar,
}

impl Default for CapabilityAssessment {
    fn default() -> Self {
        let mut assessment = Self {
            language_accuracy: 0.5,
            prediction_accuracy: 0.5,
            verification_reliability: 0.5,
            planning_success: 0.5,
            memory_retrieval_success: 0.5,
            reasoning_consistency: 0.5,
            resource_availability: 1.0,
            overall: 0.0,
        };
        assessment.overall = assessment.compute_overall();
        assessment
    }
}

impl CapabilityAssessment {
    pub fn compute_overall(&self) -> Scalar {
        (self.language_accuracy * 0.15
            + self.prediction_accuracy * 0.2
            + self.verification_reliability * 0.2
            + self.planning_success * 0.15
            + self.memory_retrieval_success * 0.15
            + self.reasoning_consistency * 0.15)
            * self.resource_availability
    }

    pub fn from_capability_set(set: &CapabilitySet) -> Self {
        let mut assessment = Self {
            language_accuracy: set.language_accuracy,
            prediction_accuracy: set.prediction_accuracy,
            verification_reliability: set.verification_reliability,
            planning_success: set.planning_success,
            memory_retrieval_success: set.memory_retrieval_success,
            reasoning_consistency: set.reasoning_consistency,
            resource_availability: 1.0,
            overall: 0.0,
        };
        assessment.overall = assessment.compute_overall();
        assessment
    }

    pub fn weakest_capability(&self) -> (&'static str, Scalar) {
        let capabilities = [
            ("language_accuracy", self.language_accuracy),
            ("prediction_accuracy", self.prediction_accuracy),
            ("verification_reliability", self.verification_reliability),
            ("planning_success", self.planning_success),
            ("memory_retrieval_success", self.memory_retrieval_success),
            ("reasoning_consistency", self.reasoning_consistency),
        ];

        capabilities
            .iter()
            .min_by(|a, b| {
                a.1.partial_cmp(b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, value)| (*name, *value))
            .unwrap_or(("unknown", 0.0))
    }

    pub fn strongest_capability(&self) -> (&'static str, Scalar) {
        let capabilities = [
            ("language_accuracy", self.language_accuracy),
            ("prediction_accuracy", self.prediction_accuracy),
            ("verification_reliability", self.verification_reliability),
            ("planning_success", self.planning_success),
            ("memory_retrieval_success", self.memory_retrieval_success),
            ("reasoning_consistency", self.reasoning_consistency),
        ];

        capabilities
            .iter()
            .max_by(|a, b| {
                a.1.partial_cmp(b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, value)| (*name, *value))
            .unwrap_or(("unknown", 0.0))
    }
}

pub struct SelfModel {
    pub capabilities: CapabilitySet,
    pub limitations: crate::types::state::LimitationSet,
    pub prediction_accuracy: Scalar,
    pub uncertainty_level: Scalar,
    pub memory_health: crate::types::state::MemoryHealth,
}

impl SelfModel {
    pub fn new() -> Self {
        Self {
            capabilities: CapabilitySet::default(),
            limitations: crate::types::state::LimitationSet::default(),
            prediction_accuracy: 0.0,
            uncertainty_level: 1.0,
            memory_health: crate::types::state::MemoryHealth::default(),
        }
    }

    pub fn assess(&self) -> CapabilityAssessment {
        CapabilityAssessment::from_capability_set(&self.capabilities)
    }

    pub fn update_prediction_accuracy(&mut self, correct: bool) {
        let delta = if correct { 0.01 } else { -0.02 };
        self.prediction_accuracy = (self.prediction_accuracy + delta).clamp(0.0, 1.0);
        self.capabilities.prediction_accuracy = self.prediction_accuracy;
    }

    pub fn add_limitation(&mut self, limitation: String) {
        if !self.limitations.known_limitations.contains(&limitation) {
            self.limitations.known_limitations.push(limitation);
        }
    }

    pub fn add_resource_constraint(&mut self, constraint: String) {
        if !self.limitations.resource_constraints.contains(&constraint) {
            self.limitations.resource_constraints.push(constraint);
        }
    }

    pub fn confidence_in_task(&self, task_type: &str) -> Scalar {
        match task_type {
            "language" => self.capabilities.language_accuracy,
            "prediction" => self.capabilities.prediction_accuracy,
            "verification" => self.capabilities.verification_reliability,
            "planning" => self.capabilities.planning_success,
            "memory" => self.capabilities.memory_retrieval_success,
            "reasoning" => self.capabilities.reasoning_consistency,
            _ => self.prediction_accuracy,
        }
    }

    pub fn should_attempt_task(&self, task_type: &str, required_confidence: Scalar) -> bool {
        self.confidence_in_task(task_type) >= required_confidence
    }
}

impl Default for SelfModel {
    fn default() -> Self {
        Self::new()
    }
}
