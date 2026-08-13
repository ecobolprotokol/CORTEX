use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::common::Timestamp;
use crate::types::ids::{EpisodeId, EvidenceId, KnowledgeId, ProvenanceId, SourceId, SessionId};
use crate::types::observation::Observation;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: EvidenceId,
    pub source: ProvenanceId,
    pub content: EvidenceContent,
    pub strength: Scalar,
    pub polarity: EvidencePolarity,
    pub timestamp: Timestamp,
    pub related: Vec<EvidenceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceContent {
    Text(String),
    Observation(Box<Observation>),
    KnowledgeRef(KnowledgeId),
    EpisodeRef(EpisodeId),
    Numeric(Scalar),
    Composite(Vec<EvidenceContent>),
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
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, evidence: Evidence) {
        self.items.push(evidence);
    }

    pub fn total_strength(&self) -> Scalar {
        self.items.iter().map(|e| e.strength).sum::<Scalar>()
            / self.items.len().max(1) as Scalar
    }

    pub fn supporting(&self) -> Vec<&Evidence> {
        self.items
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Supports)
            .collect()
    }

    pub fn contradicting(&self) -> Vec<&Evidence> {
        self.items
            .iter()
            .filter(|e| e.polarity == EvidencePolarity::Contradicts)
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn merge(&self, other: &EvidenceSet) -> EvidenceSet {
        let mut merged = self.clone();
        merged.items.extend(other.items.clone());
        merged
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub category: ProvenanceCategory,
    pub source: Source,
    pub source_identity: SourceIdentity,
    pub timestamp: Timestamp,
    pub retrieval_context: Option<RetrievalContext>,
    pub content_hash: [u8; 32],
    pub evidence: Vec<EvidenceId>,
    pub verification_status: VerificationStatus,
    pub confidence: ConfidenceState,
}

impl Provenance {
    pub fn user_provided() -> Self {
        Self {
            category: ProvenanceCategory::UserProvided,
            source: Source {
                id: SourceId::new(1),
                name: "user".into(),
                kind: SourceKind::User,
            },
            source_identity: SourceIdentity {
                identifier: "user".into(),
                reliability: 0.8,
                verification_count: 0,
            },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: Vec::new(),
            verification_status: VerificationStatus::Observed,
            confidence: ConfidenceState::default(),
        }
    }

    pub fn internet(url: &str) -> Self {
        Self {
            category: ProvenanceCategory::Internet,
            source: Source {
                id: SourceId::new(2),
                name: url.to_string(),
                kind: SourceKind::Internet,
            },
            source_identity: SourceIdentity {
                identifier: url.to_string(),
                reliability: 0.3,
                verification_count: 0,
            },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: Vec::new(),
            verification_status: VerificationStatus::Unknown,
            confidence: ConfidenceState::low(),
        }
    }

    pub fn derived(parents: &[Provenance]) -> Self {
        Self {
            category: ProvenanceCategory::Derived,
            source: Source {
                id: SourceId::new(3),
                name: "derived".into(),
                kind: SourceKind::Derived,
            },
            source_identity: SourceIdentity {
                identifier: "derived".into(),
                reliability: 0.5,
                verification_count: 0,
            },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: parents.iter().flat_map(|p| p.evidence.clone()).collect(),
            verification_status: VerificationStatus::Inferred,
            confidence: ConfidenceState::default(),
        }
    }
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
    pub name: String,
    pub kind: SourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceKind {
    User,
    System,
    Internet,
    Derived,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub identifier: String,
    pub reliability: Scalar,
    pub verification_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalContext {
    pub query: String,
    pub timestamp: Timestamp,
    pub session_id: SessionId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceState {
    pub belief: Scalar,
    pub evidence_strength: Scalar,
    pub source_quality: Scalar,
    pub consistency: Scalar,
    pub uncertainty: Scalar,
    pub prediction_reliability: Scalar,
    pub verification_status: VerificationStatus,
}

impl Default for ConfidenceState {
    fn default() -> Self {
        Self {
            belief: 0.5,
            evidence_strength: 0.0,
            source_quality: 0.5,
            consistency: 0.5,
            uncertainty: 0.5,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Unknown,
        }
    }
}

impl ConfidenceState {
    pub fn low() -> Self {
        Self {
            belief: 0.1,
            evidence_strength: 0.0,
            source_quality: 0.1,
            consistency: 0.1,
            uncertainty: 0.9,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Unknown,
        }
    }

    pub fn high() -> Self {
        Self {
            belief: 0.9,
            evidence_strength: 0.8,
            source_quality: 0.9,
            consistency: 0.9,
            uncertainty: 0.1,
            prediction_reliability: 0.8,
            verification_status: VerificationStatus::Supported,
        }
    }

    pub fn overall(&self) -> Scalar {
        let raw = (self.belief * 0.3)
            + (self.evidence_strength * 0.25)
            + (self.source_quality * 0.15)
            + (self.consistency * 0.2)
            + ((1.0 - self.uncertainty) * 0.1);
        raw.clamp(0.0, 1.0)
    }

    pub fn is_verified(&self) -> bool {
        self.verification_status == VerificationStatus::Verified
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyState {
    pub level: Scalar,
    pub dimensions: HashMap<String, Scalar>,
    pub reducible: bool,
    pub updated_at: Timestamp,
}

impl Default for UncertaintyState {
    fn default() -> Self {
        Self {
            level: 1.0,
            dimensions: HashMap::new(),
            reducible: true,
            updated_at: Timestamp::now(),
        }
    }
}

impl UncertaintyState {
    pub fn initial() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationStatus {
    Observed,
    Inferred,
    Supported,
    Provisional,
    Verified,
    Unknown,
    Contradicted,
}
