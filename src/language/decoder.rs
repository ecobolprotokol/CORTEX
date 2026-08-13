use std::collections::HashMap;

use crate::types::state::SemanticState;

use super::language_model::LanguageModel;
use super::semantics::{ConceptKind, SemanticConcept, SemanticEntity, SemanticRelation};


pub struct Decoder {
    pub lexical_cache: HashMap<String, Vec<String>>,
}

impl Decoder {
    pub fn new() -> Self {
        let mut lexical_cache = HashMap::new();

        lexical_cache.insert(
            "entity".into(),
            vec!["the".into(), "this".into(), "a".into()],
        );
        lexical_cache.insert(
            "action".into(),
            vec!["is".into(), "does".into(), "has".into()],
        );
        lexical_cache.insert(
            "property".into(),
            vec!["great".into(), "small".into(), "new".into()],
        );
        lexical_cache.insert(
            "relation".into(),
            vec!["of".into(), "in".into(), "with".into()],
        );

        Self { lexical_cache }
    }

    pub fn realize(
        &self,
        concepts: &[SemanticConcept],
        relations: &[SemanticRelation],
        entities: &[SemanticEntity],
        semantic_state: &SemanticState,
        model: &LanguageModel,
    ) -> String {
        let mut output_tokens = Vec::new();
        let mut slot_fills: HashMap<String, String> = HashMap::new();

        for concept in concepts {
            match concept.kind {
                ConceptKind::Entity => {
                    slot_fills.insert("subject".into(), concept.label.clone());
                }
                ConceptKind::Action => {
                    slot_fills.insert("predicate".into(), concept.label.clone());
                }
                ConceptKind::Property => {
                    slot_fills.insert("modifier".into(), concept.label.clone());
                }
                ConceptKind::State => {
                    slot_fills.insert("complement".into(), concept.label.clone());
                }
                _ => {}
            }
        }

        if let Some(subj) = slot_fills.get("subject") {
            output_tokens.push(self.select_determiner(subj));
            output_tokens.push(subj.clone());
        }

        if let Some(modifier) = slot_fills.get("modifier") {
            output_tokens.push(modifier.clone());
        }

        if let Some(pred) = slot_fills.get("predicate") {
            output_tokens.push(pred.clone());
        }

        if !relations.is_empty() {
            for rel in relations.iter().take(3) {
                let rel_label = &rel.kind;
                if let Some(preposition) = self.map_relation_to_preposition(rel_label) {
                    output_tokens.push(preposition);
                }
            }
        }

        if let Some(obj) = slot_fills.get("object") {
            output_tokens.push(self.select_determiner(obj));
            output_tokens.push(obj.clone());
        }

        if let Some(comp) = slot_fills.get("complement") {
            output_tokens.push("is".into());
            output_tokens.push(comp.clone());
        }

        for entity in entities.iter().take(2) {
            if !output_tokens.contains(&entity.name) {
                output_tokens.push(self.select_determiner(&entity.name));
                output_tokens.push(entity.name.clone());
            }
        }

        if output_tokens.is_empty() {
            let predictions = model.predict(3);
            for pred in predictions {
                output_tokens.push(pred.token);
            }
        }

        let _ = semantic_state;

        let mut result = output_tokens.join(" ");
        if !result.is_empty() {
            let first_char = result.remove(0);
            result.insert(0, first_char.to_uppercase().next().unwrap_or(first_char));
            result.push('.');
        }

        result
    }

    fn select_determiner(&self, noun: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let first = noun.chars().next().unwrap_or('x');
        if vowels.contains(&first.to_ascii_lowercase()) {
            "an".into()
        } else {
            "the".into()
        }
    }

    fn map_relation_to_preposition(&self, kind: &str) -> Option<String> {
        match kind {
            "agent" => Some("by".into()),
            "patient" => Some("of".into()),
            "location" => Some("in".into()),
            "time" => Some("at".into()),
            "purpose" => Some("for".into()),
            "instrument" => Some("with".into()),
            "cause" => Some("due to".into()),
            "source" => Some("from".into()),
            _ => Some("of".into()),
        }
    }
}
