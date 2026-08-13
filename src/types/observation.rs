use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::common::{ContextState, Timestamp};
use crate::types::evidence::{EvidenceSet, Provenance};
use crate::types::ids::ActionId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub text: String,
    pub source: Provenance,
    pub timestamp: Timestamp,
    pub context: ContextState,
    pub kind: ObservationKind,
    pub importance: Scalar,
}

impl Observation {
    pub fn user_provided(text: &str) -> Self {
        Self {
            text: text.to_string(),
            source: Provenance::user_provided(),
            timestamp: Timestamp::now(),
            context: ContextState::initial(),
            kind: ObservationKind::UserInput,
            importance: 0.5,
        }
    }

    pub fn from_internet(text: &str, url: &str) -> Self {
        Self {
            text: text.to_string(),
            source: Provenance::internet(url),
            timestamp: Timestamp::now(),
            context: ContextState::initial(),
            kind: ObservationKind::Internet,
            importance: 0.3,
        }
    }
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
    pub observation: Observation,
    pub internal_state: HashMap<String, Scalar>,
    pub prediction: Option<Prediction>,
    pub action: Option<Action>,
    pub outcome: Option<Outcome>,
    pub error: Option<PredictionError>,
    pub attribution: Option<String>,
    pub evidence: EvidenceSet,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: ActionId,
    pub kind: ActionKind,
    pub parameters: HashMap<String, ActionParameter>,
    pub expected_outcome: Option<Outcome>,
    pub risk: Scalar,
    pub timestamp: Timestamp,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionKind {
    Respond,
    Observe,
    Query,
    Learn,
    Plan,
    Verify,
    Fetch,
    Store,
    Forget,
    Consolidate,
    Checkpoint,
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionParameter {
    Text(String),
    Number(Scalar),
    Integer(i64),
    Boolean(bool),
    List(Vec<ActionParameter>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub success: bool,
    pub description: String,
    pub result: Option<String>,
    pub timestamp: Timestamp,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub target: PredictionTarget,
    pub predicted_state: Vec<Scalar>,
    pub confidence: Scalar,
    pub timestamp: Timestamp,
    pub context: ContextState,
    pub resolved: bool,
    pub actual: Option<Vec<Scalar>>,
    pub error: Option<PredictionError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTarget {
    NextToken,
    NextState,
    NextAction,
    Outcome,
    Transition,
    Intent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    pub magnitude: Scalar,
    pub dimensions: HashMap<String, Scalar>,
    pub timestamp: Timestamp,
    pub prediction_id: Option<u64>,
}

impl PredictionError {
    pub fn compute(predicted: &[Scalar], actual: &[Scalar]) -> Self {
        let magnitude = predicted
            .iter()
            .zip(actual.iter())
            .map(|(p, a)| (p - a).powi(2))
            .sum::<Scalar>()
            .sqrt();

        Self {
            magnitude,
            dimensions: HashMap::new(),
            timestamp: Timestamp::now(),
            prediction_id: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude < crate::types::scalars::SCALAR_EPSILON
    }
}
