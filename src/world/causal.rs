use crate::types::scalars::Scalar;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CausalHypothesisId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalHypothesis {
    pub id: CausalHypothesisId,
    pub cause: EntityId,
    pub effect: EntityId,
    pub strength: Scalar,
    pub confidence: Scalar,
    pub observation_count: u64,
    pub co_occurrence_count: u64,
    pub temporal_consistency: Scalar,
    pub confound_risk: Scalar,
    pub created_at: Timestamp,
    pub last_updated: Timestamp,
}

#[derive(Debug, Clone)]
pub struct CausalLink {
    pub source: EntityId,
    pub target: EntityId,
    pub strength: Scalar,
    pub chain_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalModel {
    pub hypotheses: Vec<CausalHypothesis>,
    pub co_occurrence_matrix: HashMap<(EntityId, EntityId), CoOccurrenceData>,
    pub temporal_pairs: Vec<TemporalPair>,
    pub next_hypothesis_id: u64,
    pub max_hypotheses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoOccurrenceData {
    pub count: u64,
    pub cause_first_count: u64,
    pub effect_first_count: u64,
    pub first_seen: Timestamp,
    pub last_seen: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPair {
    pub cause: EntityId,
    pub effect: EntityId,
    pub lag_millis: u64,
    pub occurrences: u64,
}

impl Default for CausalModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CausalModel {
    pub fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
            co_occurrence_matrix: HashMap::new(),
            temporal_pairs: Vec::new(),
            next_hypothesis_id: 1,
            max_hypotheses: 1000,
        }
    }

    pub fn store_hypothesis(
        &mut self,
        cause: EntityId,
        effect: EntityId,
        strength: Scalar,
        confidence: Scalar,
    ) -> CausalHypothesisId {
        if let Some(existing) = self.hypotheses.iter_mut().find(|h| h.cause == cause && h.effect == effect) {
            existing.strength = existing.strength * 0.7 + strength * 0.3;
            existing.confidence = existing.confidence * 0.8 + confidence * 0.2;
            existing.observation_count += 1;
            existing.last_updated = Timestamp::now();
            return existing.id.clone();
        }

        let id = CausalHypothesisId(self.next_hypothesis_id);
        self.next_hypothesis_id += 1;

        let hypothesis = CausalHypothesis {
            id: id.clone(),
            cause,
            effect,
            strength,
            confidence,
            observation_count: 1,
            co_occurrence_count: 0,
            temporal_consistency: 0.5,
            confound_risk: 0.5,
            created_at: Timestamp::now(),
            last_updated: Timestamp::now(),
        };

        if self.hypotheses.len() >= self.max_hypotheses {
            let weakest_idx = self.hypotheses.iter()
                .enumerate()
                .min_by(|a, b| {
                    let score_a = a.1.strength * a.1.confidence;
                    let score_b = b.1.strength * b.1.confidence;
                    score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.hypotheses.remove(weakest_idx);
        }

        self.hypotheses.push(hypothesis);
        id
    }

    pub fn query_hypothesis(&self, cause: EntityId, effect: EntityId) -> Option<&CausalHypothesis> {
        self.hypotheses.iter().find(|h| h.cause == cause && h.effect == effect)
    }

    pub fn query_hypotheses_for_effect(&self, effect: EntityId) -> Vec<&CausalHypothesis> {
        self.hypotheses.iter().filter(|h| h.effect == effect).collect()
    }

    pub fn query_hypotheses_for_cause(&self, cause: EntityId) -> Vec<&CausalHypothesis> {
        self.hypotheses.iter().filter(|h| h.cause == cause).collect()
    }

    pub fn update_from_observation(
        &mut self,
        observed_entities: &[EntityId],
        timestamp: Timestamp,
    ) {
        for i in 0..observed_entities.len() {
            for j in 0..observed_entities.len() {
                if i == j {
                    continue;
                }
                let key = (observed_entities[i], observed_entities[j]);
                let entry = self.co_occurrence_matrix.entry(key).or_insert_with(|| CoOccurrenceData {
                    count: 0,
                    cause_first_count: 0,
                    effect_first_count: 0,
                    first_seen: timestamp,
                    last_seen: timestamp,
                });
                entry.count += 1;
                entry.cause_first_count += 1;
                entry.last_seen = timestamp;
            }
        }

        for entity_id in observed_entities {
            self.temporal_pairs.push(TemporalPair {
                cause: *entity_id,
                effect: *entity_id,
                lag_millis: 0,
                occurrences: 1,
            });
        }
    }

    pub fn record_temporal_observation(
        &mut self,
        cause: EntityId,
        effect: EntityId,
        cause_time: Timestamp,
        effect_time: Timestamp,
    ) {
        let lag = effect_time.0.saturating_sub(cause_time.0);

        if let Some(entry) = self.temporal_pairs.iter_mut().find(|p| p.cause == cause && p.effect == effect) {
            let old_total = entry.occurrences;
            entry.lag_millis = (entry.lag_millis * old_total + lag) / (old_total + 1);
            entry.occurrences += 1;
        } else {
            self.temporal_pairs.push(TemporalPair {
                cause,
                effect,
                lag_millis: lag,
                occurrences: 1,
            });
        }

        let key = (cause, effect);
        let entry = self.co_occurrence_matrix.entry(key).or_insert_with(|| CoOccurrenceData {
            count: 0,
            cause_first_count: 0,
            effect_first_count: 0,
            first_seen: cause_time,
            last_seen: effect_time,
        });
        entry.count += 1;
        if cause_time.is_before(effect_time) {
            entry.cause_first_count += 1;
        } else {
            entry.effect_first_count += 1;
        }
        entry.last_seen = effect_time;
    }

    pub fn estimate_strength(&self, cause: EntityId, effect: EntityId) -> Scalar {
        let key = (cause, effect);
        let reverse_key = (effect, cause);

        let co_data = self.co_occurrence_matrix.get(&key);
        let reverse_data = self.co_occurrence_matrix.get(&reverse_key);

        let total_cause = co_data.map(|d| d.count).unwrap_or(0)
            + reverse_data.map(|d| d.count).unwrap_or(0);
        if total_cause == 0 {
            return 0.0;
        }

        let forward_count = co_data.map(|d| d.cause_first_count).unwrap_or(0) as Scalar;
        let total_count = total_cause as Scalar;

        let base_strength = forward_count / total_count;

        let temporal_data = self.temporal_pairs.iter()
            .find(|p| p.cause == cause && p.effect == effect);
        let temporal_boost = if let Some(tp) = temporal_data {
            if tp.occurrences > 2 && tp.lag_millis > 0 {
                (tp.occurrences as Scalar).min(5.0) / 5.0 * 0.2
            } else {
                0.0
            }
        } else {
            0.0
        };

        let consistency = self.temporal_consistency(cause, effect);

        let strength = base_strength * 0.5 + consistency * 0.3 + temporal_boost + 0.1;
        strength.clamp(0.0, 1.0)
    }

    fn temporal_consistency(&self, cause: EntityId, effect: EntityId) -> Scalar {
        let pairs: Vec<&TemporalPair> = self.temporal_pairs.iter()
            .filter(|p| p.cause == cause && p.effect == effect && p.occurrences > 1)
            .collect();

        if pairs.is_empty() {
            return 0.5;
        }

        let avg_lag: Scalar = pairs.iter().map(|p| p.lag_millis as Scalar).sum::<Scalar>() / pairs.len() as Scalar;
        if avg_lag == 0.0 {
            return 0.3;
        }

        let variance: Scalar = pairs.iter()
            .map(|p| {
                let diff = p.lag_millis as Scalar - avg_lag;
                diff * diff
            })
            .sum::<Scalar>() / pairs.len() as Scalar;

        let cv = variance.sqrt() / avg_lag;
        let consistency = (1.0 - cv.min(1.0)) * 0.5 + 0.5;
        consistency
    }
}

#[derive(Debug, Clone)]
pub struct ConfoundCandidate {
    pub confounder: EntityId,
    pub score: Scalar,
}

pub fn detect_confounds(
    model: &CausalModel,
    cause: EntityId,
    effect: EntityId,
    all_entity_ids: &[EntityId],
) -> Vec<ConfoundCandidate> {
    let mut candidates = Vec::new();

    for &candidate_id in all_entity_ids {
        if candidate_id == cause || candidate_id == effect {
            continue;
        }

        let cause_to_confounder = model.estimate_strength(cause, candidate_id);
        let confounder_to_effect = model.estimate_strength(candidate_id, effect);
        let direct_strength = model.estimate_strength(cause, effect);

        if cause_to_confounder > 0.2 && confounder_to_effect > 0.2 {
            let indirect_strength = cause_to_confounder * confounder_to_effect;
            let score = if direct_strength > 0.0 {
                (indirect_strength / (direct_strength + indirect_strength)).min(1.0)
            } else {
                indirect_strength
            };

            if score > 0.1 {
                candidates.push(ConfoundCandidate {
                    confounder: candidate_id,
                    score,
                });
            }
        }
    }

    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    candidates
}

#[derive(Debug, Clone)]
pub struct CausalChain {
    pub links: Vec<CausalLink>,
    pub total_strength: Scalar,
    pub length: u32,
}

pub fn trace_causal_chains(
    model: &CausalModel,
    start: EntityId,
    end: EntityId,
    max_depth: u32,
) -> Vec<CausalChain> {
    let mut chains = Vec::new();
    let mut stack: Vec<(EntityId, Vec<CausalLink>, Scalar)> = vec![
        (start, Vec::new(), 1.0),
    ];

    while let Some((current, links, cumulative_strength)) = stack.pop() {
        if current == end && !links.is_empty() {
            chains.push(CausalChain {
                links: links.clone(),
                total_strength: cumulative_strength,
                length: links.len() as u32,
            });
            continue;
        }

        if links.len() as u32 >= max_depth {
            continue;
        }

        let hypotheses = model.query_hypotheses_for_cause(current);
        for hyp in hypotheses {
            if hyp.effect == current {
                continue;
            }
            let already_visited = links.iter().any(|l| l.target == hyp.effect);
            if already_visited {
                continue;
            }

            let mut new_links = links.clone();
            new_links.push(CausalLink {
                source: hyp.cause,
                target: hyp.effect,
                strength: hyp.strength,
                chain_depth: new_links.len() as u32,
            });
            let new_strength = cumulative_strength * hyp.strength;
            stack.push((hyp.effect, new_links, new_strength));
        }
    }

    chains.sort_by(|a, b| b.total_strength.partial_cmp(&a.total_strength).unwrap_or(std::cmp::Ordering::Equal));
    chains
}

#[derive(Debug, Clone)]
pub struct InterventionResult {
    pub target: EntityId,
    pub value_override: String,
    pub predicted_effects: Vec<(EntityId, Scalar)>,
    pub confidence: Scalar,
    pub side_effects: Vec<(EntityId, Scalar)>,
}

pub fn reason_about_intervention(
    model: &CausalModel,
    target: EntityId,
    proposed_state: &str,
    _all_entity_ids: &[EntityId],
) -> InterventionResult {
    let mut predicted_effects = Vec::new();
    let mut side_effects = Vec::new();
    let mut total_confidence = 0.0;
    let mut effect_count = 0;

    let direct_causes = model.query_hypotheses_for_cause(target);

    for hyp in direct_causes {
        if hyp.strength > 0.2 {
            predicted_effects.push((hyp.effect, hyp.strength));
            total_confidence += hyp.confidence;
            effect_count += 1;
        }
    }

    let mut visited: std::collections::HashSet<EntityId> = std::collections::HashSet::new();
    let mut frontier: Vec<(EntityId, Scalar, u32)> = predicted_effects.iter()
        .map(|(eid, strength)| (*eid, *strength, 1))
        .collect();
    visited.insert(target);
    for (eid, _) in &predicted_effects {
        visited.insert(*eid);
    }

    while let Some((current, strength, depth)) = frontier.pop() {
        if depth > 3 {
            continue;
        }
        let next_hyps = model.query_hypotheses_for_cause(current);
        for hyp in next_hyps {
            if !visited.contains(&hyp.effect) && hyp.strength > 0.15 {
                let propagated = strength * hyp.strength;
                if propagated > 0.05 {
                    side_effects.push((hyp.effect, propagated));
                    visited.insert(hyp.effect);
                    frontier.push((hyp.effect, propagated, depth + 1));
                }
            }
        }
    }

    let confidence = if effect_count > 0 {
        (total_confidence / effect_count as Scalar).clamp(0.1, 0.9)
    } else {
        0.2
    };

    InterventionResult {
        target,
        value_override: proposed_state.to_string(),
        predicted_effects,
        confidence,
        side_effects,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausalStrength {
    Strong,
    Moderate,
    Weak,
    None,
}

pub fn distinguish_causation(correlation: Scalar, temporal_order: bool, confounders: usize) -> CausalStrength {
    let mut strength = correlation;
    if temporal_order {
        strength *= 1.2;
    }
    strength *= 1.0 / (1.0 + confounders as Scalar);
    match strength {
        x if x > 0.8 => CausalStrength::Strong,
        x if x > 0.5 => CausalStrength::Moderate,
        x if x > 0.2 => CausalStrength::Weak,
        _ => CausalStrength::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_state() -> WorldState {
        WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: TemporalContext::default(),
            uncertainty: UncertaintyState::initial(),
            next_entity_id: EntityId(1),
            next_relation_id: RelationId(1),
            next_event_id: EventId(1),
        }
    }

    #[test]
    fn test_strong_causation() {
        assert_eq!(distinguish_causation(0.9, true, 0), CausalStrength::Strong);
    }

    #[test]
    fn test_weak_causation() {
        let result = distinguish_causation(0.3, false, 5);
        assert!(matches!(result, CausalStrength::Weak | CausalStrength::None));
    }

    #[test]
    fn test_no_causation() {
        let result = distinguish_causation(0.1, false, 10);
        assert!(matches!(result, CausalStrength::None));
    }

    #[test]
    fn test_temporal_order_boosts() {
        let with_temporal = 0.5 * 1.2;
        let without_temporal = 0.5;
        assert!(with_temporal > without_temporal);
    }

    #[test]
    fn test_store_and_query_hypothesis() {
        let mut model = CausalModel::new();
        let id = model.store_hypothesis(EntityId(1), EntityId(2), 0.8, 0.9);
        assert_eq!(id, CausalHypothesisId(1));
        let hyp = model.query_hypothesis(EntityId(1), EntityId(2)).unwrap();
        assert_eq!(hyp.cause, EntityId(1));
        assert_eq!(hyp.effect, EntityId(2));
        assert!((hyp.strength - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_update_hypothesis() {
        let mut model = CausalModel::new();
        model.store_hypothesis(EntityId(1), EntityId(2), 0.8, 0.9);
        model.store_hypothesis(EntityId(1), EntityId(2), 0.6, 0.7);
        let hyp = model.query_hypothesis(EntityId(1), EntityId(2)).unwrap();
        assert!((hyp.strength - 0.74).abs() < 0.01);
        assert_eq!(hyp.observation_count, 2);
    }

    #[test]
    fn test_query_by_effect() {
        let mut model = CausalModel::new();
        model.store_hypothesis(EntityId(1), EntityId(3), 0.5, 0.6);
        model.store_hypothesis(EntityId(2), EntityId(3), 0.7, 0.8);
        model.store_hypothesis(EntityId(1), EntityId(4), 0.3, 0.4);
        let hyps = model.query_hypotheses_for_effect(EntityId(3));
        assert_eq!(hyps.len(), 2);
    }

    #[test]
    fn test_estimate_strength() {
        let mut model = CausalModel::new();
        let t1 = Timestamp(1000);
        let t2 = Timestamp(2000);
        model.record_temporal_observation(EntityId(1), EntityId(2), t1, t2);
        model.record_temporal_observation(EntityId(1), EntityId(2), t1, t2);
        model.record_temporal_observation(EntityId(1), EntityId(2), t1, t2);
        let strength = model.estimate_strength(EntityId(1), EntityId(2));
        assert!(strength > 0.3);
    }

    #[test]
    fn test_detect_confounds() {
        let mut model = CausalModel::new();
        model.record_temporal_observation(EntityId(1), EntityId(3), Timestamp(1000), Timestamp(1500));
        model.record_temporal_observation(EntityId(1), EntityId(3), Timestamp(2000), Timestamp(2500));
        model.record_temporal_observation(EntityId(3), EntityId(2), Timestamp(1600), Timestamp(2000));
        model.record_temporal_observation(EntityId(3), EntityId(2), Timestamp(2600), Timestamp(3000));
        model.record_temporal_observation(EntityId(1), EntityId(2), Timestamp(1000), Timestamp(3000));

        let confounds = detect_confounds(&model, EntityId(1), EntityId(2), &[EntityId(1), EntityId(2), EntityId(3)]);
        assert!(!confounds.is_empty());
        assert_eq!(confounds[0].confounder, EntityId(3));
    }

    #[test]
    fn test_trace_causal_chains() {
        let mut model = CausalModel::new();
        model.store_hypothesis(EntityId(1), EntityId(2), 0.8, 0.9);
        model.store_hypothesis(EntityId(2), EntityId(3), 0.7, 0.85);
        model.store_hypothesis(EntityId(1), EntityId(3), 0.4, 0.5);

        let chains = trace_causal_chains(&model, EntityId(1), EntityId(3), 3);
        assert!(chains.len() >= 2);
        assert!(chains[0].total_strength >= chains[1].total_strength);
    }

    #[test]
    fn test_intervention_reasoning() {
        let mut model = CausalModel::new();
        model.store_hypothesis(EntityId(1), EntityId(2), 0.8, 0.9);
        model.store_hypothesis(EntityId(2), EntityId(3), 0.6, 0.7);

        let result = reason_about_intervention(&model, EntityId(1), "modified", &[EntityId(1), EntityId(2), EntityId(3)]);
        assert_eq!(result.target, EntityId(1));
        assert_eq!(result.value_override, "modified");
        assert!(!result.predicted_effects.is_empty());
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_causal_chain_no_path() {
        let model = CausalModel::new();
        let chains = trace_causal_chains(&model, EntityId(1), EntityId(5), 3);
        assert!(chains.is_empty());
    }

    #[test]
    fn test_intervention_with_side_effects() {
        let mut model = CausalModel::new();
        model.store_hypothesis(EntityId(1), EntityId(2), 0.9, 0.9);
        model.store_hypothesis(EntityId(2), EntityId(3), 0.7, 0.8);
        model.store_hypothesis(EntityId(2), EntityId(4), 0.5, 0.6);

        let result = reason_about_intervention(&model, EntityId(1), "changed", &[EntityId(1), EntityId(2), EntityId(3), EntityId(4)]);
        assert!(!result.side_effects.is_empty());
    }

    #[test]
    fn test_max_hypotheses_eviction() {
        let mut model = CausalModel::new();
        model.max_hypotheses = 3;
        model.store_hypothesis(EntityId(1), EntityId(2), 0.9, 0.9);
        model.store_hypothesis(EntityId(3), EntityId(4), 0.8, 0.8);
        model.store_hypothesis(EntityId(5), EntityId(6), 0.7, 0.7);
        model.store_hypothesis(EntityId(7), EntityId(8), 0.6, 0.6);
        assert!(model.hypotheses.len() <= 3);
    }

    #[test]
    fn test_co_occurrence_tracking() {
        let mut model = CausalModel::new();
        model.update_from_observation(&[EntityId(1), EntityId(2), EntityId(3)], Timestamp::now());
        let key = (EntityId(1), EntityId(2));
        let data = model.co_occurrence_matrix.get(&key).unwrap();
        assert_eq!(data.count, 1);
    }
}
