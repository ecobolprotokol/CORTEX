use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::common::Timestamp;
use crate::types::ids::{EntityId, TransitionId};
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedState {
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub description: String,
    pub state_changes: Vec<StateChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub entity_id: EntityId,
    pub property: String,
    pub old_value: String,
    pub new_value: String,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule {
    pub id: TransitionId,
    pub precondition: String,
    pub action_pattern: String,
    pub effect: String,
    pub strength: Scalar,
    pub applications: u64,
}

pub struct TransitionModel {
    pub rules: Vec<TransitionRule>,
    pub state_snapshots: Vec<StateSnapshot>,
    pub default_confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub timestamp: Timestamp,
    pub entities: HashMap<EntityId, HashMap<String, String>>,
}

impl TransitionModel {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            state_snapshots: Vec::new(),
            default_confidence: 0.5,
        }
    }

    pub fn add_rule(&mut self, precondition: &str, action_pattern: &str, effect: &str) {
        self.rules.push(TransitionRule {
            id: TransitionId::next(),
            precondition: precondition.to_string(),
            action_pattern: action_pattern.to_string(),
            effect: effect.to_string(),
            strength: 0.1,
            applications: 0,
        });
    }

    pub fn predict(&self, current_state: &str, action: &str) -> PredictedState {
        let mut matched_rules: Vec<&TransitionRule> = self
            .rules
            .iter()
            .filter(|r| action.contains(&r.action_pattern) || r.action_pattern.contains(action))
            .collect();

        matched_rules.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if matched_rules.is_empty() {
            PredictedState {
                confidence: self.default_confidence * 0.5,
                uncertainty: 1.0 - self.default_confidence * 0.5,
                description: format!(
                    "No matching rules for action '{}' in state '{}'",
                    action, current_state
                ),
                state_changes: Vec::new(),
            }
        } else {
            let top_rule = matched_rules[0];
            let confidence = top_rule.strength;
            PredictedState {
                confidence,
                uncertainty: 1.0 - confidence,
                description: format!(
                    "Rule-based prediction: {} (strength: {:.2})",
                    top_rule.effect, top_rule.strength
                ),
                state_changes: Vec::new(),
            }
        }
    }

    pub fn record_application(&mut self, rule_id: TransitionId, success: bool) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            rule.applications += 1;
            if success {
                rule.strength = (rule.strength + 0.05).min(1.0);
            } else {
                rule.strength = (rule.strength - 0.02).max(0.01);
            }
        }
    }

    pub fn save_snapshot(&mut self, entities: HashMap<EntityId, HashMap<String, String>>) {
        self.state_snapshots.push(StateSnapshot {
            timestamp: Timestamp::now(),
            entities,
        });
        if self.state_snapshots.len() > 100 {
            self.state_snapshots.remove(0);
        }
    }

    pub fn get_last_snapshot(&self) -> Option<&StateSnapshot> {
        self.state_snapshots.last()
    }

    pub fn compute_transition_probability(
        &self,
        precondition_met: bool,
        rule_strength: Scalar,
    ) -> Scalar {
        if precondition_met {
            rule_strength
        } else {
            rule_strength * 0.1
        }
    }
}

impl Default for TransitionModel {
    fn default() -> Self {
        Self::new()
    }
}
