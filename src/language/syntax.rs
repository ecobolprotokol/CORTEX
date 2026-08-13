use std::collections::HashMap;

use crate::types::ids::{ConceptId, SymbolId};
use crate::types::state::SyntaxState;

#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub token: String,
    pub symbol_id: SymbolId,
    pub role: SyntacticRole,
    pub depth: u32,
    pub dependencies: Vec<usize>,
    pub head: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntacticRole {
    Subject,
    Predicate,
    Object,
    Modifier,
    Determiner,
    Conjunction,
    Punctuation,
    Preposition,
    Adverbial,
    Complement,
}

pub struct SyntaxAnalyzer {
    pub rules_applied: u64,
    pub patterns: HashMap<String, String>,
}

impl Default for SyntaxAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxAnalyzer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert("subject".into(), "noun_phrase".into());
        patterns.insert("predicate".into(), "verb_phrase".into());
        patterns.insert("object".into(), "noun_phrase".into());
        patterns.insert("modifier".into(), "adjunct".into());
        patterns.insert("scope".into(), "embedded_clause".into());

        Self {
            rules_applied: 0,
            patterns,
        }
    }

    pub fn parse_syntax(
        &mut self,
        tokens: &[String],
        symbol_ids: &[SymbolId],
        concepts: &[ConceptId],
    ) -> (Vec<SyntaxNode>, SyntaxState) {
        let mut nodes: Vec<SyntaxNode> = Vec::new();
        let mut depth: u32 = 0;
        let mut nesting_stack: Vec<u32> = Vec::new();
        let mut role_assignments = HashMap::new();

        for (i, token) in tokens.iter().enumerate() {
            let symbol_id = symbol_ids.get(i).copied().unwrap_or(SymbolId::NULL);
            let role = self.classify_token(token, i, tokens);
            let dependencies = self.find_dependencies(i, tokens, &role);

            let mut node = SyntaxNode {
                token: token.clone(),
                symbol_id,
                role: role.clone(),
                depth,
                dependencies,
                head: None,
            };

            match token.as_str() {
                "(" | "{" | "[" => {
                    nesting_stack.push(depth);
                    depth += 1;
                    node.head = Some(i.saturating_sub(1));
                }
                ")" | "}" | "]" => {
                    if let Some(prev) = nesting_stack.pop() {
                        depth = prev;
                    }
                }
                "," | ";" => {
                    node.head = Some(i.saturating_sub(1));
                }
                _ => {
                    if i > 0 {
                        node.head = Some(i - 1);
                    }
                }
            }

            role_assignments.insert(token.clone(), format!("{:?}", role));
            nodes.push(node);
            self.rules_applied += 1;
        }

        let mut active_patterns: Vec<String> = Vec::new();
        for node in &nodes {
            if let Some(pat) = self.patterns.get(&format!("{:?}", node.role).to_lowercase()) {
                if !active_patterns.contains(pat) {
                    active_patterns.push(pat.clone());
                }
            }
        }

        let _ = concepts;

        let syntax_state = SyntaxState {
            rules_applied: self.rules_applied,
            parse_depth: depth,
            active_patterns,
        };

        (nodes, syntax_state)
    }

    fn classify_token(&self, token: &str, position: usize, all_tokens: &[String]) -> SyntacticRole {
        match token {
            "," | "." | ";" | ":" | "!" | "?" => SyntacticRole::Punctuation,
            "and" | "or" | "but" | "nor" => SyntacticRole::Conjunction,
            "the" | "a" | "an" | "this" | "that" | "these" | "those" => SyntacticRole::Determiner,
            "in" | "on" | "at" | "to" | "for" | "with" | "by" | "from" | "of" | "about" => {
                SyntacticRole::Preposition
            }
            "very" | "really" | "quite" | "always" | "never" | "often" | "sometimes" => {
                SyntacticRole::Adverbial
            }
            _ => {
                if is_verb(token) {
                    if position > 0 {
                        return SyntacticRole::Predicate;
                    }
                    SyntacticRole::Predicate
                } else if is_adjective(token) {
                    SyntacticRole::Modifier
                } else if is_noun(token) {
                    if position == 0 || all_tokens.get(position.saturating_sub(1)).is_some_and(|t| is_punctuation(t)) {
                        SyntacticRole::Subject
                    } else {
                        SyntacticRole::Object
                    }
                } else {
                    if position == 0 {
                        SyntacticRole::Subject
                    } else if position == all_tokens.len().saturating_sub(1) {
                        SyntacticRole::Complement
                    } else {
                        SyntacticRole::Object
                    }
                }
            }
        }
    }

    fn find_dependencies(
        &self,
        index: usize,
        tokens: &[String],
        role: &SyntacticRole,
    ) -> Vec<usize> {
        let mut deps = Vec::new();
        match role {
            SyntacticRole::Subject => {
                for (i, token) in tokens.iter().enumerate().take(index) {
                    if matches!(self.quick_classify(token), SyntacticRole::Determiner) {
                        deps.push(i);
                    }
                }
            }
            SyntacticRole::Predicate => {
                for (i, token) in tokens.iter().enumerate().take(index) {
                    if matches!(self.quick_classify(token), SyntacticRole::Adverbial) {
                        deps.push(i);
                    }
                }
            }
            SyntacticRole::Object => {
                for (i, token) in tokens.iter().enumerate().take(index) {
                    if matches!(
                        self.quick_classify(token),
                        SyntacticRole::Preposition | SyntacticRole::Determiner
                    ) {
                        deps.push(i);
                    }
                }
            }
            _ => {}
        }
        deps
    }

    fn quick_classify(&self, token: &str) -> SyntacticRole {
        match token {
            "the" | "a" | "an" => SyntacticRole::Determiner,
            "and" | "or" | "but" => SyntacticRole::Conjunction,
            "in" | "on" | "at" | "to" | "for" => SyntacticRole::Preposition,
            "very" | "always" | "never" => SyntacticRole::Adverbial,
            "," | "." | ";" | ":" | "!" | "?" => SyntacticRole::Punctuation,
            _ => SyntacticRole::Object,
        }
    }
}

fn is_verb(token: &str) -> bool {
    matches!(
        token,
        "is" | "are" | "was" | "were" | "be" | "been" | "being"
            | "have" | "has" | "had"
            | "do" | "does" | "did"
            | "will" | "would" | "shall" | "should"
            | "can" | "could"
            | "may" | "might" | "must"
            | "go" | "goes" | "went" | "going"
            | "make" | "makes" | "made" | "making"
            | "take" | "takes" | "took" | "taken"
            | "come" | "comes" | "came" | "coming"
            | "give" | "gives" | "gave" | "given"
            | "see" | "sees" | "saw" | "seen"
            | "know" | "knows" | "knew" | "known"
            | "think" | "thinks" | "thought"
            | "say" | "says" | "said"
            | "get" | "gets" | "got"
            | "use" | "uses" | "used"
            | "find" | "finds" | "found"
            | "tell" | "tells" | "told"
            | "ask" | "asks" | "asked"
            | "work" | "works" | "worked"
            | "seem" | "seems" | "seemed"
            | "feel" | "feels" | "felt"
            | "try" | "tries" | "tried"
            | "leave" | "leaves" | "left"
            | "call" | "calls" | "called"
            | "keep" | "keeps" | "kept"
            | "let" | "lets"
            | "begin" | "begins" | "began"
            | "show" | "shows" | "showed"
            | "hear" | "hears" | "heard"
            | "play" | "plays" | "played"
            | "run" | "runs" | "ran"
            | "move" | "moves" | "moved"
            | "live" | "lives" | "lived"
            | "believe" | "believes" | "believed"
            | "bring" | "brings" | "brought"
            | "happen" | "happens" | "happened"
            | "write" | "writes" | "wrote" | "written"
            | "provide" | "provides" | "provided"
            | "sit" | "sits" | "sat"
            | "stand" | "stands" | "stood"
            | "lose" | "loses" | "lost"
            | "pay" | "pays" | "paid"
            | "meet" | "meets" | "met"
            | "include" | "includes" | "included"
            | "continue" | "continues" | "continued"
            | "set" | "sets"
            | "learn" | "learns" | "learned"
            | "change" | "changes" | "changed"
            | "lead" | "leads" | "led"
            | "understand" | "understands" | "understood"
            | "watch" | "watches" | "watched"
            | "follow" | "follows" | "followed"
            | "stop" | "stops" | "stopped"
            | "create" | "creates" | "created"
            | "speak" | "speaks" | "spoke" | "spoken"
            | "read" | "reads"
            | "allow" | "allows" | "allowed"
            | "add" | "adds" | "added"
            | "spend" | "spends" | "spent"
            | "grow" | "grows" | "grew" | "grown"
            | "open" | "opens" | "opened"
            | "walk" | "walks" | "walked"
            | "win" | "wins" | "won"
            | "offer" | "offers" | "offered"
            | "remember" | "remembers" | "remembered"
            | "love" | "loves" | "loved"
            | "consider" | "considers" | "considered"
            | "appear" | "appears" | "appeared"
            | "buy" | "buys" | "bought"
            | "wait" | "waits" | "waited"
            | "serve" | "serves" | "served"
            | "die" | "dies" | "died"
            | "send" | "sends" | "sent"
            | "expect" | "expects" | "expected"
            | "build" | "builds" | "built"
            | "stay" | "stays" | "stayed"
            | "fall" | "falls" | "fell"
            | "cut" | "cuts"
            | "reach" | "reaches" | "reached"
            | "kill" | "kills" | "killed"
            | "remain" | "remains" | "remained"
            | "suggest" | "suggests" | "suggested"
            | "raise" | "raises" | "raised"
            | "pass" | "passes" | "passed"
            | "sell" | "sells" | "sold"
            | "require" | "requires" | "required"
            | "report" | "reports" | "reported"
            | "decide" | "decides" | "decided"
            | "pull" | "pulls" | "pulled"
    )
}

fn is_noun(token: &str) -> bool {
    let lower = token.to_lowercase();
    matches!(lower.as_str(),
        "time" | "year" | "people" | "way" | "day" | "man" | "woman" | "child"
        | "world" | "life" | "hand" | "part" | "place" | "case" | "week" | "company"
        | "system" | "program" | "question" | "work" | "government" | "number" | "night"
        | "point" | "home" | "water" | "room" | "mother" | "area" | "money" | "story"
        | "fact" | "month" | "lot" | "right" | "study" | "book" | "eye" | "job"
        | "word" | "business" | "issue" | "side" | "kind" | "head" | "house"
        | "service" | "friend" | "father" | "power" | "hour" | "game" | "line"
        | "end" | "members" | "city" | "community" | "name" | "president" | "team"
        | "minute" | "idea" | "body" | "information" | "back" | "parent" | "face"
        | "others" | "level" | "office" | "door" | "person" | "art" | "car" | "language"
        | "computer" | "result" | "health" | "school" | "music" | "market" | "food"
        | "data" | "model" | "concept" | "entity" | "process" | "state" | "input"
        | "output" | "text" | "token" | "symbol" | "relation" | "meaning" | "context"
    ) || lower.ends_with("tion")
        || lower.ends_with("sion")
        || lower.ends_with("ment")
        || lower.ends_with("ness")
        || lower.ends_with("ity")
        || lower.ends_with("ence")
        || lower.ends_with("ance")
}

fn is_adjective(token: &str) -> bool {
    matches!(
        token,
        "good" | "new" | "first" | "last" | "long" | "great" | "little" | "own"
            | "other" | "old" | "right" | "big" | "high" | "different" | "small"
            | "large" | "next" | "early" | "young" | "important" | "few" | "public"
            | "bad" | "same" | "able" | "clear" | "strong" | "simple" | "complex"
            | "true" | "false" | "valid" | "active" | "full" | "empty" | "current"
            | "possible" | "specific" | "general" | "special" | "significant"
    ) || token.starts_with("un")
        || token.starts_with("re")
}

fn is_punctuation(token: &str) -> bool {
    matches!(token, "," | "." | ";" | ":" | "!" | "?" | "(" | ")" | "{" | "}" | "[" | "]")
}
