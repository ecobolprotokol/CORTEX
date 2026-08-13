use crate::types::*;
use crate::language::syntax::SyntaxAnalysis;

#[derive(Debug, Clone, Default)]
pub struct SemanticAnalysis {
    pub concepts: Vec<ConceptId>,
    pub entities: Vec<EntityId>,
    pub relations: Vec<crate::types::state::Relation>,
    pub graph: SemanticGraph,
    pub intent: Option<Intent>,
    pub key_phrases: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SemanticGraph {
    pub nodes: Vec<SemanticNode>,
    pub edges: Vec<SemanticEdge>,
}

#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: ConceptId,
    pub label: String,
    pub kind: NodeKind,
    pub salience: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Concept,
    Entity,
    Relation,
    Property,
    Action,
}

#[derive(Debug, Clone)]
pub struct SemanticEdge {
    pub source: ConceptId,
    pub target: ConceptId,
    pub relation: EdgeRelation,
    pub weight: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeRelation {
    IsA,
    HasProperty,
    PartOf,
    Causes,
    Requires,
    Enables,
    RelatedTo,
    AgentOf,
    ObjectOf,
}

pub fn extract(symbols: &[Symbol], syntax: &SyntaxAnalysis) -> SemanticAnalysis {
    let mut analysis = SemanticAnalysis::default();
    if symbols.is_empty() {
        return analysis;
    }

    let mut next_concept_id = 1u64;

    for (i, symbol) in symbols.iter().enumerate() {
        if is_stop_word(&symbol.text) {
            continue;
        }
        let concept_id = ConceptId(next_concept_id);
        next_concept_id += 1;
        analysis.concepts.push(concept_id);

        let role = syntax.roles.iter().find(|r| r.position == i);
        let (kind, salience) = match role.map(|r| r.role) {
            Some(crate::language::syntax::SyntacticRole::Subject) => (NodeKind::Entity, 0.9),
            Some(crate::language::syntax::SyntacticRole::Object) => (NodeKind::Entity, 0.8),
            Some(crate::language::syntax::SyntacticRole::Predicate) => (NodeKind::Action, 0.85),
            Some(crate::language::syntax::SyntacticRole::Modifier) => (NodeKind::Property, 0.5),
            _ => (NodeKind::Concept, 0.6),
        };

        analysis.graph.nodes.push(SemanticNode {
            id: concept_id,
            label: symbol.text.clone(),
            kind,
            salience,
        });

        if kind == NodeKind::Entity {
            analysis.entities.push(EntityId(concept_id.0));
        }
    }

    build_edges(&mut analysis);
    detect_intent(symbols, syntax, &mut analysis);
    extract_key_phrases(symbols, &mut analysis);

    analysis
}

fn build_edges(analysis: &mut SemanticAnalysis) {
    let nodes = analysis.graph.nodes.clone();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            let (relation, weight) = if a.kind == NodeKind::Action && b.kind == NodeKind::Entity {
                (EdgeRelation::AgentOf, 0.7)
            } else if a.kind == NodeKind::Entity && b.kind == NodeKind::Action {
                (EdgeRelation::ObjectOf, 0.6)
            } else if a.kind == NodeKind::Entity && b.kind == NodeKind::Property {
                (EdgeRelation::HasProperty, 0.5)
            } else if a.kind == NodeKind::Property && b.kind == NodeKind::Entity {
                (EdgeRelation::HasProperty, 0.5)
            } else if are_synonyms(&a.label, &b.label) {
                (EdgeRelation::RelatedTo, 0.8)
            } else if are_topically_related(&a.label, &b.label) {
                (EdgeRelation::RelatedTo, 0.4)
            } else {
                continue;
            };
            analysis.graph.edges.push(SemanticEdge {
                source: a.id,
                target: b.id,
                relation,
                weight,
            });
        }
    }
}

fn detect_intent(symbols: &[Symbol], syntax: &SyntaxAnalysis, analysis: &mut SemanticAnalysis) {
    let texts: Vec<&str> = symbols.iter().map(|s| s.text.as_str()).collect();
    if syntax.structure.has_question {
        analysis.intent = Some(Intent::Question);
    } else if syntax.structure.has_imperative {
        analysis.intent = Some(Intent::Instruction);
    } else if texts.iter().any(|t| *t == "not" || *t == "no" || *t == "wrong" || *t == "incorrect" || *t == "actually") {
        analysis.intent = Some(Intent::Correction);
    } else {
        analysis.intent = Some(Intent::Statement);
    }
}

fn extract_key_phrases(symbols: &[Symbol], analysis: &mut SemanticAnalysis) {
    let nouns: Vec<&str> = symbols.iter()
        .filter(|s| !is_stop_word(&s.text) && s.kind == SymbolKind::Word)
        .map(|s| s.text.as_str())
        .collect();
    if nouns.len() >= 2 {
        analysis.key_phrases.push(nouns[0..2].join(" "));
    }
    if nouns.len() >= 3 {
        analysis.key_phrases.push(nouns[0..3].join(" "));
    }
}

fn is_stop_word(text: &str) -> bool {
    matches!(text, "a" | "an" | "the" | "is" | "are" | "was" | "were" | "be" | "been" | "being"
        | "have" | "has" | "had" | "do" | "does" | "did" | "will" | "would" | "could" | "should"
        | "may" | "might" | "shall" | "can" | "must" | "i" | "you" | "he" | "she" | "it" | "we" | "they"
        | "my" | "your" | "his" | "her" | "its" | "our" | "their"
        | "me" | "him" | "us" | "them" | "this" | "that" | "these" | "those"
        | "in" | "on" | "at" | "to" | "for" | "with" | "by" | "from" | "of" | "about"
        | "and" | "but" | "or" | "nor" | "not" | "no" | "if" | "then" | "else"
        | "what" | "how" | "why" | "when" | "where" | "who" | "which"
        | "am" | "as" | "so" | "too" | "very" | "just" | "also")
}

fn are_synonyms(a: &str, b: &str) -> bool {
    let synonyms: Vec<(&str, &str)> = vec![
        ("big", "large"), ("small", "little"), ("fast", "quick"),
        ("happy", "glad"), ("sad", "unhappy"), ("good", "better"),
        ("bad", "worse"), ("big", "huge"), ("small", "tiny"),
        ("important", "significant"), ("help", "assist"),
        ("start", "begin"), ("end", "finish"), ("use", "utilize"),
    ];
    synonyms.iter().any(|(x, y)| (x == &a && y == &b) || (y == &a && x == &b))
}

fn are_topically_related(a: &str, b: &str) -> bool {
    let topics: Vec<(&str, Vec<&str>)> = vec![
        ("gravity", vec!["force", "mass", "attraction", "physics", "newton", "fall", "acceleration"]),
        ("water", vec!["liquid", "h2o", "ice", "steam", "ocean", "rain", "boil", "freeze"]),
        ("computer", vec!["software", "hardware", "cpu", "memory", "program", "code", "data"]),
        ("learning", vec!["knowledge", "education", "study", "teach", "understand", "practice"]),
        ("energy", vec!["power", "force", "work", "heat", "light", "electricity"]),
        ("light", vec!["photon", "electromagnetic", "spectrum", "wavelength", "brightness"]),
    ];
    for (topic, related) in &topics {
        let a_match = a == *topic || related.contains(&a);
        let b_match = b == *topic || related.contains(&b);
        if a_match && b_match {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::syntax;

    fn make_symbols(texts: &[&str]) -> Vec<Symbol> {
        texts.iter().enumerate().map(|(i, t)| Symbol {
            id: SymbolId(i as u64),
            text: t.to_string(),
            kind: SymbolKind::Word,
            frequency: 1,
            activation: 1.0,
            confidence: 0.5,
        }).collect()
    }

    #[test]
    fn test_extract_concepts() {
        let symbols = make_symbols(&["what", "is", "gravity"]);
        let syntax_analysis = syntax::analyze(&symbols);
        let analysis = extract(&symbols, &syntax_analysis);
        assert!(!analysis.concepts.is_empty());
        assert!(analysis.intent.is_some());
    }

    #[test]
    fn test_intent_detection() {
        let symbols = make_symbols(&["what", "is", "gravity"]);
        let syntax_analysis = syntax::analyze(&symbols);
        let analysis = extract(&symbols, &syntax_analysis);
        assert_eq!(analysis.intent, Some(Intent::Question));
    }
}
