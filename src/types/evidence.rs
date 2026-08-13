use serde::{Deserialize, Serialize};

use crate::types::common::Timestamp;
use crate::types::ids::{EvidenceId, ProvenanceId, SourceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: EvidenceId,
    pub source: ProvenanceId,
    pub content: EvidenceContent,
    pub strength: f32,
    pub polarity: EvidencePolarity,
    pub timestamp: Timestamp,
    pub related: Vec<EvidenceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceContent {
    Observation(String),
    Inference(String),
    Testimony(String),
    Measurement { value: f64, unit: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidencePolarity {
    Supports,
    Contradicts,
    Neutral,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvidenceSet {
    pub items: Vec<Evidence>,
}

impl EvidenceSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self) -> Vec<&Evidence> {
        self.items
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Supports)
            .collect()
    }

    pub fn contradicts(&self) -> Vec<&Evidence> {
        self.items
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Contradicts)
            .collect()
    }

    pub fn total_strength(&self) -> f32 {
        self.items.iter().map(|e| e.strength).sum()
    }

    pub fn net_strength(&self) -> f32 {
        self.items
            .iter()
            .map(|e| match e.polarity {
                EvidencePolarity::Supports => e.strength,
                EvidencePolarity::Contradicts => -e.strength,
                EvidencePolarity::Neutral => 0.0,
            })
            .sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub category: ProvenanceCategory,
    pub source: SourceId,
    pub source_identity: SourceIdentity,
    pub timestamp: Timestamp,
    pub retrieval_context: RetrievalContext,
    pub content_hash: [u8; 32],
    pub evidence: Vec<EvidenceId>,
    pub verification_status: VerificationStatus,
    pub confidence: ConfidenceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProvenanceCategory {
    Observed,
    UserProvided,
    Internet,
    Derived,
    Inferred,
    Replayed,
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub kind: SourceKind,
    pub name: String,
    pub identity: SourceIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceKind {
    User,
    Tool,
    Internet,
    Internal,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub display_name: String,
    pub trust_level: f32,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalContext {
    pub query: String,
    pub method: String,
    pub timestamp: Timestamp,
    pub result_count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConfidenceState {
    pub direct: f32,
    pub corroborated: f32,
    pub staleness: f32,
}

impl Default for ConfidenceState {
    fn default() -> Self {
        Self {
            direct: 0.5,
            corroborated: 0.0,
            staleness: 0.0,
        }
    }
}

impl ConfidenceState {
    pub fn low() -> Self {
        Self {
            direct: 0.2,
            corroborated: 0.0,
            staleness: 0.0,
        }
    }

    pub fn high() -> Self {
        Self {
            direct: 0.9,
            corroborated: 0.8,
            staleness: 0.0,
        }
    }

    pub fn overall(&self) -> f32 {
        let base = self.direct * 0.6 + self.corroborated * 0.4;
        base * (1.0 - self.staleness)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyState {
    pub epistemic: f32,
    pub aleatoric: f32,
    pub total: f32,
}

impl UncertaintyState {
    pub fn new(epistemic: f32, aleatoric: f32) -> Self {
        let total = (epistemic.powi(2) + aleatoric.powi(2)).sqrt();
        Self {
            epistemic,
            aleatoric,
            total,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
    Disputed,
}
