use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct SyntaxAnalysis {
    pub dependencies: Vec<Dependency>,
    pub roles: Vec<RoleAssignment>,
    pub structure: SyntacticStructure,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub governor: usize,
    pub dependent: usize,
    pub relation: DependencyRelation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyRelation {
    NominalSubject,
    DirectObject,
    IndirectObject,
    PrepositionalObject,
    Modifier,
    Predicate,
    Conjunction,
    Determiner,
    Compound,
    Root,
}

#[derive(Debug, Clone)]
pub struct RoleAssignment {
    pub position: usize,
    pub role: SyntacticRole,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntacticRole {
    Subject,
    Predicate,
    Object,
    IndirectObject,
    Modifier,
    Complement,
    Adjunct,
}

#[derive(Debug, Clone, Default)]
pub struct SyntacticStructure {
    pub clause_count: usize,
    pub has_question: bool,
    pub has_imperative: bool,
    pub has_negation: bool,
    pub tense: Tense,
    pub mood: Mood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tense {
    #[default]
    Present,
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mood {
    #[default]
    Indicative,
    Interrogative,
    Imperative,
    Subjunctive,
}

pub fn analyze(symbols: &[Symbol]) -> SyntaxAnalysis {
    let mut analysis = SyntaxAnalysis::default();
    if symbols.is_empty() {
        return analysis;
    }

    let structure = analyze_structure(symbols);
    analysis.structure = structure;

    let roles = assign_roles(symbols, &analysis.structure);
    analysis.roles = roles;

    let dependencies = build_dependencies(symbols, &analysis);
    analysis.dependencies = dependencies;

    analysis
}

fn analyze_structure(symbols: &[Symbol]) -> SyntacticStructure {
    let mut structure = SyntacticStructure::default();
    let texts: Vec<&str> = symbols.iter().map(|s| s.text.as_str()).collect();

    structure.has_question = texts.iter().any(|t| *t == "?" || *t == "how" || *t == "what" || *t == "why" || *t == "when" || *t == "where" || *t == "who" || *t == "which" || *t == "is" || *t == "are" || *t == "do" || *t == "does" || *t == "can" || *t == "could" || *t == "would" || *t == "should");
    structure.has_imperative = texts.first().map_or(false, |t| {
        matches!(*t, "please" | "tell" | "give" | "show" | "explain" | "describe" | "list" | "create" | "make" | "set" | "run" | "start" | "stop")
    });
    structure.has_negation = texts.iter().any(|t| *t == "not" || *t == "no" || *t == "never" || *t == "don't" || *t == "doesn't" || *t == "didn" || *t == "won't" || *t == "can't" || *t == "cannot");

    if structure.has_question {
        structure.mood = Mood::Interrogative;
    } else if structure.has_imperative {
        structure.mood = Mood::Imperative;
    }

    let question_words = ["what", "how", "why", "when", "where", "who", "which"];
    if texts.iter().any(|t| question_words.contains(t)) {
        structure.tense = Tense::Present;
    }

    structure.clause_count = texts.iter().filter(|t| **t == "," || **t == ";" || **t == "and" || **t == "but" || **t == "or").count() + 1;

    structure
}

fn assign_roles(symbols: &[Symbol], structure: &SyntacticStructure) -> Vec<RoleAssignment> {
    let mut roles = Vec::new();
    let texts: Vec<&str> = symbols.iter().map(|s| s.text.as_str()).collect();

    if texts.is_empty() {
        return roles;
    }

    if structure.has_question {
        if let Some(pos) = texts.iter().position(|t| *t == "is" || *t == "are" || *t == "was" || *t == "were" || *t == "do" || *t == "does" || *t == "did") {
            roles.push(RoleAssignment { position: pos, role: SyntacticRole::Predicate, confidence: 0.8 });
            if pos > 0 {
                roles.push(RoleAssignment { position: 0, role: SyntacticRole::Subject, confidence: 0.7 });
            }
            if pos + 1 < texts.len() {
                roles.push(RoleAssignment { position: pos + 1, role: SyntacticRole::Object, confidence: 0.6 });
            }
        } else if texts.len() >= 2 {
            roles.push(RoleAssignment { position: 0, role: SyntacticRole::Subject, confidence: 0.6 });
            roles.push(RoleAssignment { position: 1, role: SyntacticRole::Predicate, confidence: 0.7 });
        }
    } else {
        roles.push(RoleAssignment { position: 0, role: SyntacticRole::Subject, confidence: 0.5 });
        if texts.len() > 1 {
            let verb_pos = texts.iter().position(|t| is_verb_like(t)).unwrap_or(1);
            roles.push(RoleAssignment { position: verb_pos, role: SyntacticRole::Predicate, confidence: 0.6 });
            for i in (verb_pos + 1)..texts.len() {
                if !is_function_word(texts[i]) {
                    roles.push(RoleAssignment { position: i, role: SyntacticRole::Object, confidence: 0.5 });
                    break;
                }
            }
        }
    }

    for (i, text) in texts.iter().enumerate() {
        if roles.iter().any(|r| r.position == i) {
            continue;
        }
        if is_modifier(text) {
            roles.push(RoleAssignment { position: i, role: SyntacticRole::Modifier, confidence: 0.4 });
        }
    }

    roles
}

fn build_dependencies(symbols: &[Symbol], analysis: &SyntaxAnalysis) -> Vec<Dependency> {
    let mut deps = Vec::new();
    for role in &analysis.roles {
        if role.role == SyntacticRole::Subject {
            if let Some(pred) = analysis.roles.iter().find(|r| r.role == SyntacticRole::Predicate) {
                deps.push(Dependency {
                    governor: pred.position,
                    dependent: role.position,
                    relation: DependencyRelation::NominalSubject,
                });
            }
        }
        if role.role == SyntacticRole::Object {
            if let Some(pred) = analysis.roles.iter().find(|r| r.role == SyntacticRole::Predicate) {
                deps.push(Dependency {
                    governor: pred.position,
                    dependent: role.position,
                    relation: DependencyRelation::DirectObject,
                });
            }
        }
    }
    deps
}

fn is_verb_like(text: &str) -> bool {
    matches!(text, "is" | "are" | "was" | "were" | "be" | "been" | "being"
        | "have" | "has" | "had" | "do" | "does" | "did"
        | "will" | "would" | "shall" | "should" | "may" | "might" | "can" | "could"
        | "go" | "goes" | "went" | "come" | "comes" | "came"
        | "make" | "makes" | "made" | "take" | "takes" | "took"
        | "give" | "gives" | "gave" | "get" | "gets" | "got"
        | "say" | "says" | "said" | "tell" | "tells" | "told"
        | "know" | "knows" | "knew" | "think" | "thinks" | "thought"
        | "see" | "sees" | "saw" | "look" | "looks" | "looked"
        | "use" | "uses" | "used" | "find" | "finds" | "found"
        | "want" | "wants" | "wanted" | "need" | "needs" | "needed"
        | "work" | "works" | "worked" | "call" | "calls" | "called"
        | "try" | "tries" | "tried" | "ask" | "asks" | "asked"
        | "mean" | "means" | "meant" | "keep" | "keeps" | "kept"
        | "let" | "lets" | "begin" | "begins" | "began"
        | "seem" | "seems" | "seemed" | "help" | "helps" | "helped"
        | "show" | "shows" | "showed" | "hear" | "hears" | "heard"
        | "play" | "plays" | "played" | "run" | "runs" | "ran"
        | "move" | "moves" | "moved" | "live" | "lives" | "lived"
        | "believe" | "believed" | "hold" | "holds" | "held"
        | "bring" | "brings" | "brought" | "happen" | "happens" | "happened"
        | "write" | "writes" | "wrote" | "provide" | "provides" | "provided"
        | "sit" | "sits" | "sat" | "stand" | "stands" | "stood"
        | "lose" | "loses" | "lost" | "pay" | "pays" | "paid"
        | "meet" | "meets" | "met" | "include" | "includes" | "included"
        | "continue" | "continues" | "continued" | "set" | "learn" | "learns" | "learned"
        | "change" | "changes" | "changed" | "lead" | "leads" | "led"
        | "understand" | "understands" | "understood" | "watch" | "watches" | "watched"
        | "follow" | "follows" | "followed" | "stop" | "stops" | "stopped"
        | "create" | "creates" | "created" | "speak" | "speaks" | "spoke"
        | "read" | "reads" | "allow" | "allows" | "allowed"
        | "add" | "adds" | "added" | "spend" | "spends" | "spent"
        | "grow" | "grows" | "grew" | "open" | "opens" | "opened"
        | "walk" | "walks" | "walked" | "win" | "wins" | "won"
        | "offer" | "offers" | "offered" | "remember" | "remembers" | "remembered"
        | "love" | "loves" | "loved" | "consider" | "considers" | "considered"
        | "appear" | "appears" | "appeared" | "buy" | "buys" | "bought"
        | "wait" | "waits" | "waited" | "serve" | "serves" | "served"
        | "die" | "dies" | "died" | "send" | "sends" | "sent"
        | "build" | "builds" | "built" | "stay" | "stays" | "stayed"
        | "fall" | "falls" | "fell" | "cut" | "reach" | "reaches" | "reached"
        | "kill" | "kills" | "killed" | "remain" | "remains" | "remained"
        | "suggest" | "suggests" | "suggested" | "raise" | "raises" | "raised"
        | "pass" | "passes" | "passed" | "sell" | "sells" | "sold"
        | "require" | "requires" | "required" | "report" | "reports" | "reported"
        | "decide" | "decides" | "decided" | "pull" | "pulls" | "pulled")
}

fn is_function_word(text: &str) -> bool {
    matches!(text, "a" | "an" | "the" | "this" | "that" | "these" | "those"
        | "i" | "you" | "he" | "she" | "it" | "we" | "they"
        | "my" | "your" | "his" | "her" | "its" | "our" | "their"
        | "me" | "him" | "us" | "them"
        | "in" | "on" | "at" | "to" | "for" | "with" | "by" | "from"
        | "of" | "about" | "into" | "through" | "during" | "before" | "after"
        | "above" | "below" | "between" | "under" | "over"
        | "and" | "but" | "or" | "nor" | "for" | "yet" | "so"
        | "if" | "then" | "else" | "when" | "where" | "how" | "what"
        | "who" | "whom" | "whose" | "which" | "why"
        | "not" | "no" | "never" | "neither" | "either"
        | "is" | "are" | "was" | "were" | "be" | "been" | "being"
        | "have" | "has" | "had" | "do" | "does" | "did"
        | "will" | "would" | "shall" | "should" | "may" | "might" | "can" | "could" | "must")
}

fn is_modifier(text: &str) -> bool {
    text.ends_with("ly") || text.ends_with("ful") || text.ends_with("less")
        || text.ends_with("ous") || text.ends_with("ive") || text.ends_with("able")
        || text.ends_with("ible") || text.ends_with("al") || text.ends_with("ial")
        || text.ends_with("ic") || text.ends_with("ical")
        || matches!(text, "very" | "really" | "quite" | "rather" | "somewhat" | "extremely" | "highly" | "most" | "more" | "less" | "least" | "best" | "worst" | "better" | "worse")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Symbol;

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
    fn test_question_detection() {
        let symbols = make_symbols(&["what", "is", "gravity"]);
        let analysis = analyze(&symbols);
        assert!(analysis.structure.has_question);
        assert_eq!(analysis.structure.mood, Mood::Interrogative);
    }

    #[test]
    fn test_subject_predicate() {
        let symbols = make_symbols(&["gravity", "is", "a", "force"]);
        let analysis = analyze(&symbols);
        assert!(analysis.roles.iter().any(|r| r.role == SyntacticRole::Subject));
        assert!(analysis.roles.iter().any(|r| r.role == SyntacticRole::Predicate));
    }
}
