use super::common::*;
use super::ids::*;
use super::scalars::Scalar;
use super::evidence::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub text: String,
    pub source: Provenance,
    pub timestamp: Timestamp,
    pub context: ContextState,
    pub kind: ObservationKind,
    pub importance: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationKind {
    UserInput,
    Environment,
    Internet,
    Internal,
    Feedback,
    Correction,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: ActionId,
    pub kind: ActionKind,
    pub parameters: HashMap<String, ActionParameter>,
    pub expected_outcome: Option<Outcome>,
    pub risk: RiskAssessment,
    pub timestamp: Timestamp,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn zero() -> Self {
        Self {
            magnitude: 0.0,
            dimensions: HashMap::new(),
            timestamp: Timestamp::now(),
            prediction_id: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude < super::scalars::SCALAR_EPSILON
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedResponse {
    pub text: String,
    pub confidence: Scalar,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentInput {
    pub text: String,
    pub timestamp: Timestamp,
    pub kind: ObservationKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationState {
    pub tokens_generated: u32,
    pub max_tokens: u32,
    pub current_candidates: Vec<CandidateContinuation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub query_type: MemoryQueryType,
    pub text: String,
    pub concept_ids: Vec<ConceptId>,
    pub time_range: Option<(Timestamp, Timestamp)>,
    pub max_results: u32,
    pub min_confidence: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryQueryType {
    Semantic,
    Episodic,
    Procedural,
    Associative,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRetrieval {
    pub episodic: Vec<ScoredEpisode>,
    pub semantic: Vec<ScoredKnowledge>,
    pub procedural: Vec<ScoredProcedure>,
    pub associative: Vec<ScoredAssociation>,
    pub relevance_scores: HashMap<u64, Scalar>,
    pub contradictions: Vec<super::evidence::Contradiction>,
}

impl Default for MemoryRetrieval {
    fn default() -> Self {
        Self {
            episodic: Vec::new(),
            semantic: Vec::new(),
            procedural: Vec::new(),
            associative: Vec::new(),
            relevance_scores: HashMap::new(),
            contradictions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredEpisode {
    pub episode: super::state::Episode,
    pub relevance_score: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredKnowledge {
    pub knowledge: super::state::Knowledge,
    pub relevance_score: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredProcedure {
    pub procedure: super::state::Procedure,
    pub relevance_score: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredAssociation {
    pub association: super::state::Association,
    pub relevance_score: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub inferences: Vec<Inference>,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inference {
    pub proposition: super::state::Proposition,
    pub confidence: Scalar,
    pub evidence_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRepresentation {
    pub concepts: Vec<ConceptId>,
    pub entities: Vec<EntityId>,
    pub relations: Vec<super::state::Relation>,
    pub goal: Option<String>,
}
