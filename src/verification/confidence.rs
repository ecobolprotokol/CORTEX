use crate::types::scalars::Scalar;
use crate::types::evidence::{VerificationStatus, ConfidenceState};

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub claim: String,
    pub status: VerificationStatus,
    pub confidence: Scalar,
    pub evidence_count: u32,
}

pub struct ConfidenceModel {
    pub minimum_confidence: Scalar,
    pub belief_weight: Scalar,
    pub evidence_weight: Scalar,
    pub source_weight: Scalar,
    pub consistency_weight: Scalar,
    pub uncertainty_penalty: Scalar,
}

impl ConfidenceModel {
    pub fn new(minimum_confidence: Scalar) -> Self {
        Self {
            minimum_confidence,
            belief_weight: 0.3,
            evidence_weight: 0.25,
            source_weight: 0.15,
            consistency_weight: 0.2,
            uncertainty_penalty: 0.1,
        }
    }

    pub fn verify(&self, claim: &str, evidence_strength: Scalar) -> VerificationResult {
        let status = self.classify_from_strength(evidence_strength);

        VerificationResult {
            claim: claim.to_string(),
            status,
            confidence: evidence_strength,
            evidence_count: 0,
        }
    }

    fn classify_from_strength(&self, strength: Scalar) -> VerificationStatus {
        if strength >= self.minimum_confidence {
            VerificationStatus::Verified
        } else if strength >= 0.6 {
            VerificationStatus::Supported
        } else if strength >= 0.3 {
            VerificationStatus::Provisional
        } else if strength > 0.0 {
            VerificationStatus::Unknown
        } else {
            VerificationStatus::Observed
        }
    }

    pub fn compute_overall_confidence(
        &self,
        direct: Scalar,
        corroborated: Scalar,
        staleness: Scalar,
    ) -> Scalar {
        let base = direct * 0.6 + corroborated * 0.4;
        base * (1.0 - staleness)
    }

    pub fn compute_multidimensional(&self, state: &ConfidenceState) -> Scalar {
        let weighted_belief = state.belief * self.belief_weight;
        let weighted_evidence = state.evidence_strength * self.evidence_weight;
        let weighted_source = state.source_quality * self.source_weight;
        let weighted_consistency = state.consistency * self.consistency_weight;
        let uncertainty_penalty = state.uncertainty * self.uncertainty_penalty;

        let raw = weighted_belief
            + weighted_evidence
            + weighted_source
            + weighted_consistency
            - uncertainty_penalty;

        raw.clamp(0.0, 1.0)
    }

    pub fn compute_with_prediction_reliability(
        &self,
        state: &ConfidenceState,
        prediction_reliability: Scalar,
    ) -> Scalar {
        let base = self.compute_multidimensional(state);
        let prediction_bonus = prediction_reliability * 0.1;
        (base + prediction_bonus).min(1.0)
    }

    pub fn should_verify(&self, confidence: Scalar) -> bool {
        confidence >= self.minimum_confidence
    }

    pub fn confidence_level(&self, confidence: Scalar) -> &'static str {
        if confidence >= 0.9 {
            "Very High"
        } else if confidence >= 0.75 {
            "High"
        } else if confidence >= 0.5 {
            "Moderate"
        } else if confidence >= 0.25 {
            "Low"
        } else {
            "Very Low"
        }
    }

    pub fn merge_confidence_states(
        &self,
        states: &[ConfidenceState],
    ) -> ConfidenceState {
        if states.is_empty() {
            return ConfidenceState::default();
        }

        let n = states.len() as Scalar;
        ConfidenceState {
            belief: states.iter().map(|s| s.belief).sum::<Scalar>() / n,
            evidence_strength: states.iter().map(|s| s.evidence_strength).sum::<Scalar>() / n,
            source_quality: states.iter().map(|s| s.source_quality).sum::<Scalar>() / n,
            consistency: states.iter().map(|s| s.consistency).sum::<Scalar>() / n,
            uncertainty: states.iter().map(|s| s.uncertainty).sum::<Scalar>() / n,
            prediction_reliability: states
                .iter()
                .map(|s| s.prediction_reliability)
                .sum::<Scalar>()
                / n,
            verification_status: VerificationStatus::Provisional,
        }
    }
}
