use serde::{Deserialize, Serialize};

use crate::types::common::{Duration, Timestamp};
use crate::types::ids::{
    ActionId, EpisodeId, EvidenceId, FieldId, HypothesisId, MemoryId, SourceId,
};
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub text: String,
    pub source: SourceId,
    pub timestamp: Timestamp,
    pub context: Vec<MemoryId>,
    pub kind: ObservationKind,
    pub importance: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservationKind {
    UserInput,
    Environment,
    Internet,
    Internal,
    Feedback,
    Correction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub episode: EpisodeId,
    pub observation: Observation,
    pub actions: Vec<Action>,
    pub outcome: Outcome,
    pub predictions: Vec<Prediction>,
    pub prediction_errors: Vec<PredictionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: ActionId,
    pub kind: ActionKind,
    pub target: FieldId,
    pub parameters: Vec<Scalar>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionKind {
    Activate,
    Deactivate,
    ModifyWeight,
    CreateLink,
    RemoveLink,
    Query,
    Store,
    Infer,
    Plan,
    Observe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub success: bool,
    pub reward: Scalar,
    pub duration: Duration,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub target: PredictionTarget,
    pub predicted: Scalar,
    pub confidence: Scalar,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTarget {
    Reward { episode: EpisodeId },
    Observation { source: SourceId },
    Transition { from: FieldId, to: FieldId },
    Completion { goal: FieldId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    pub prediction: Prediction,
    pub actual: Scalar,
    pub error_magnitude: Scalar,
    pub evidence: Vec<EvidenceId>,
}
