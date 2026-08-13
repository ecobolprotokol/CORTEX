use std::collections::HashMap;

use crate::types::ids::{ConceptId, EntityId, RelationId, SymbolId};
use crate::types::state::SemanticState;

use super::syntax::{SyntaxNode, SyntacticRole};

#[derive(Debug, Clone)]
pub struct SemanticConcept {
    pub id: ConceptId,
    pub label: String,
    pub kind: ConceptKind,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConceptKind {
    Entity,
    Action,
    Property,
    Relation,
    Event,
    State,
}

#[derive(Debug, Clone)]
pub struct SemanticRelation {
    pub id: RelationId,
    pub source: ConceptId,
    pub target: ConceptId,
    pub kind: String,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct SemanticEntity {
    pub id: EntityId,
    pub name: String,
    pub kind: String,
    pub confidence: f32,
}

pub struct SemanticAnalyzer {
    pub concept_count: u64,
    pub relation_count: u64,
    pub entity_count: u64,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            concept_count: 0,
            relation_count: 0,
            entity_count: 0,
        }
    }

    pub fn extract_semantics(
        &mut self,
        nodes: &[SyntaxNode],
        symbols: &[SymbolId],
    ) -> (Vec<SemanticConcept>, Vec<SemanticRelation>, Vec<SemanticEntity>, SemanticState) {
        let mut concepts = Vec::new();
        let mut relations = Vec::new();
        let mut entities = Vec::new();
        let mut slot_bindings = HashMap::new();
        let mut active_frames = Vec::new();
        let mut coherence_sum = 0.0f32;
        let mut coherence_count = 0u32;

        let subjects: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.role == SyntacticRole::Subject)
            .map(|(i, _)| i)
            .collect();

        let predicates: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.role == SyntacticRole::Predicate)
            .map(|(i, _)| i)
            .collect();

        let objects: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.role == SyntacticRole::Object)
            .map(|(i, _)| i)
            .collect();

        for &si in &subjects {
            let node = &nodes[si];
            let concept_id = ConceptId::from(si as u64 + 1);
            self.concept_count += 1;
            concepts.push(SemanticConcept {
                id: concept_id,
                label: node.token.clone(),
                kind: ConceptKind::Entity,
                confidence: 0.7,
            });
            entities.push(SemanticEntity {
                id: EntityId::from(si as u64 + 1),
                name: node.token.clone(),
                kind: "subject".into(),
                confidence: 0.7,
            });
            slot_bindings.insert("subject".into(), node.token.clone());
            coherence_sum += 0.7;
            coherence_count += 1;
        }

        for &pi in &predicates {
            let node = &nodes[pi];
            let concept_id = ConceptId::from(pi as u64 + 1);
            self.concept_count += 1;
            concepts.push(SemanticConcept {
                id: concept_id,
                label: node.token.clone(),
                kind: ConceptKind::Action,
                confidence: 0.8,
            });
            slot_bindings.insert("predicate".into(), node.token.clone());
            active_frames.insert(0, format!("action:{}", node.token));
            coherence_sum += 0.8;
            coherence_count += 1;
        }

        for &oi in &objects {
            let node = &nodes[oi];
            let concept_id = ConceptId::from(oi as u64 + 1);
            self.concept_count += 1;
            concepts.push(SemanticConcept {
                id: concept_id,
                label: node.token.clone(),
                kind: ConceptKind::Entity,
                confidence: 0.6,
            });
            entities.push(SemanticEntity {
                id: EntityId::from(oi as u64 + 1),
                name: node.token.clone(),
                kind: "object".into(),
                confidence: 0.6,
            });
            slot_bindings.insert("object".into(), node.token.clone());
            coherence_sum += 0.6;
            coherence_count += 1;
        }

        for modifier in nodes.iter().filter(|n| n.role == SyntacticRole::Modifier) {
            let concept_id = ConceptId::from(self.concept_count + 1);
            self.concept_count += 1;
            concepts.push(SemanticConcept {
                id: concept_id,
                label: modifier.token.clone(),
                kind: ConceptKind::Property,
                confidence: 0.5,
            });
            coherence_sum += 0.5;
            coherence_count += 1;
        }

        for (si_idx, &si) in subjects.iter().enumerate() {
            for &pi in &predicates {
                let rel_id = RelationId::from(self.relation_count + 1);
                self.relation_count += 1;
                relations.push(SemanticRelation {
                    id: rel_id,
                    source: ConceptId::from(si as u64 + 1),
                    target: ConceptId::from(pi as u64 + 1),
                    kind: "agent".into(),
                    confidence: 0.7,
                });
                let _ = si_idx;
            }
        }

        for &pi in &predicates {
            for &oi in &objects {
                let rel_id = RelationId::from(self.relation_count + 1);
                self.relation_count += 1;
                relations.push(SemanticRelation {
                    id: rel_id,
                    source: ConceptId::from(pi as u64 + 1),
                    target: ConceptId::from(oi as u64 + 1),
                    kind: "patient".into(),
                    confidence: 0.65,
                });
            }
        }

        let _ = symbols;

        if !active_frames.is_empty() {
            coherence_sum += 0.5;
            coherence_count += 1;
        }

        let coherence = if coherence_count > 0 {
            coherence_sum / coherence_count as f32
        } else {
            0.0
        };

        let semantic_state = SemanticState {
            active_frames,
            slot_bindings,
            coherence,
        };

        (concepts, relations, entities, semantic_state)
    }
}
