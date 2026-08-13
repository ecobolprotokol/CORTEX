pub mod confidence;

use crate::config::VerificationConfig;
use crate::error::Result;
use crate::types::*;

pub trait VerificationEngine {
    fn evaluate(&mut self, reasoning: &ReasoningResult) -> Result<VerifiedResult>;
    fn state(&self) -> &VerificationState;
}

pub struct VerificationEngineImpl {
    config: VerificationConfig,
    state: VerificationState,
}

impl VerificationEngineImpl {
    pub fn new(config: &VerificationConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: VerificationState {
                pending_claims: Vec::new(),
                verified_claims: 0,
                contradicted_claims: 0,
                confidence_threshold: config.minimum_confidence,
            },
        })
    }
}

impl VerificationEngine for VerificationEngineImpl {
    fn evaluate(&mut self, reasoning: &ReasoningResult) -> Result<VerifiedResult> {
        let claims: Vec<Claim> = reasoning.hypotheses.iter().map(|h| {
            let status = if h.confidence >= self.config.minimum_confidence {
                VerificationStatus::Verified
            } else if h.confidence >= 0.5 {
                VerificationStatus::Supported
            } else {
                VerificationStatus::Provisional
            };
            Claim {
                id: ClaimId(h.id.0),
                text: h.proposition.predicate.clone(),
                status,
                confidence: ConfidenceState {
                    belief: h.confidence,
                    evidence_strength: h.evidence.total_strength(),
                    source_quality: 0.5,
                    consistency: 0.5,
                    uncertainty: 1.0 - h.confidence,
                    prediction_reliability: 0.0,
                    verification_status: status,
                },
                evidence: h.evidence.clone(),
                created_at: Timestamp::now(),
            }
        }).collect();

        let overall = confidence::compute_overall(&claims);
        let status = if claims.iter().any(|c| c.status == VerificationStatus::Verified) {
            VerificationStatus::Verified
        } else if claims.iter().any(|c| c.status == VerificationStatus::Supported) {
            VerificationStatus::Supported
        } else {
            VerificationStatus::Provisional
        };

        Ok(VerifiedResult {
            claims,
            overall_confidence: overall,
            verification_status: status,
            reasoning_result: Some(reasoning.clone()),
        })
    }

    fn state(&self) -> &VerificationState {
        &self.state
    }
}
