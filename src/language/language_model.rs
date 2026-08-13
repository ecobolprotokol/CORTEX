use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScoredToken {
    pub token: String,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct ScoreFactors {
    pub language: f32,
    pub context: f32,
    pub semantic: f32,
    pub memory: f32,
    pub world: f32,
    pub verification: f32,
}

pub struct LanguageModel {
    pub vocab_frequencies: HashMap<String, u64>,
    pub bigram_counts: HashMap<(String, String), u64>,
    pub context_window: Vec<String>,
    pub weights: ScoreWeights,
}

#[derive(Debug, Clone)]
pub struct ScoreWeights {
    pub language: f32,
    pub context: f32,
    pub semantic: f32,
    pub memory: f32,
    pub world: f32,
    pub verification: f32,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            language: 0.30,
            context: 0.20,
            semantic: 0.15,
            memory: 0.15,
            world: 0.10,
            verification: 0.10,
        }
    }
}

impl Default for LanguageModel {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageModel {
    pub fn new() -> Self {
        Self {
            vocab_frequencies: HashMap::new(),
            bigram_counts: HashMap::new(),
            context_window: Vec::new(),
            weights: ScoreWeights::default(),
        }
    }

    pub fn update_from_tokens(&mut self, tokens: &[String]) {
        for token in tokens {
            *self.vocab_frequencies
                .entry(token.clone())
                .or_insert(0) += 1;
        }

        for window in tokens.windows(2) {
            *self.bigram_counts
                .entry((window[0].clone(), window[1].clone()))
                .or_insert(0) += 1;
        }

        for token in tokens {
            if self.context_window.len() >= 32 {
                self.context_window.remove(0);
            }
            self.context_window.push(token.clone());
        }
    }

    pub fn predict(&self, top_k: usize) -> Vec<ScoredToken> {
        let context_len = self.context_window.len();
        let last_token = self.context_window.last().map(|s| s.as_str());

        let mut candidates: HashMap<String, ScoreFactors> = HashMap::new();

        for token in self.vocab_frequencies.keys() {
            let lang_score = self.language_score(token);
            let ctx_score = self.context_score(token, last_token);
            let sem_score = self.semantic_score(token);
            let mem_score = self.memory_score(token);
            let world_score = self.world_score(token);
            let ver_score = self.verification_score(token);

            candidates.insert(
                token.clone(),
                ScoreFactors {
                    language: lang_score,
                    context: ctx_score,
                    semantic: sem_score,
                    memory: mem_score,
                    world: world_score,
                    verification: ver_score,
                },
            );
        }

        let mut scored: Vec<ScoredToken> = candidates
            .into_iter()
            .map(|(token, factors)| {
                let score = factors.language * self.weights.language
                    + factors.context * self.weights.context
                    + factors.semantic * self.weights.semantic
                    + factors.memory * self.weights.memory
                    + factors.world * self.weights.world
                    + factors.verification * self.weights.verification;
                ScoredToken { token, score }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        let _ = context_len;
        scored
    }

    fn language_score(&self, token: &str) -> f32 {
        let total: u64 = self.vocab_frequencies.values().sum();
        if total == 0 {
            return 0.0;
        }
        let freq = self.vocab_frequencies.get(token).copied().unwrap_or(0);
        let base = freq as f32 / total as f32;

        if let Some(last) = self.context_window.last() {
            let bigram = (last.clone(), token.to_string());
            let bigram_count = self.bigram_counts.get(&bigram).copied().unwrap_or(0);
            let unigram_count = self
                .vocab_frequencies
                .get(last)
                .copied()
                .unwrap_or(1);
            let conditional = bigram_count as f32 / unigram_count as f32;
            return (base * 0.4 + conditional * 0.6).min(1.0);
        }

        base.min(1.0)
    }

    fn context_score(&self, token: &str, last_token: Option<&str>) -> f32 {
        if let Some(last) = last_token {
            if token == last {
                return 0.1;
            }
        }

        let mut overlap = 0u32;
        for ctx_token in &self.context_window {
            if *ctx_token == token {
                overlap += 1;
            }
        }

        let recency_bonus = if let Some(last) = last_token {
            if token.starts_with(&last[..last.len().min(3)]) {
                0.2
            } else {
                0.0
            }
        } else {
            0.0
        };

        let frequency_bonus = (overlap as f32 / self.context_window.len().max(1) as f32) * 0.3;
        (frequency_bonus + recency_bonus).min(1.0)
    }

    fn semantic_score(&self, token: &str) -> f32 {
        let noun_markers = [
            "the", "a", "an", "this", "that", "is", "are", "was", "were",
        ];
        let verb_markers = [
            "to", "can", "will", "would", "could", "should", "may", "might",
        ];

        for marker in &noun_markers {
            if let Some(last) = self.context_window.last() {
                if *marker == *last && is_content_word(token) {
                    return 0.8;
                }
            }
        }

        for marker in &verb_markers {
            if let Some(last) = self.context_window.last() {
                if *marker == *last && is_content_word(token) {
                    return 0.7;
                }
            }
        }

        if is_content_word(token) {
            0.6
        } else {
            0.4
        }
    }

    fn memory_score(&self, token: &str) -> f32 {
        let freq = self.vocab_frequencies.get(token).copied().unwrap_or(0);
        if freq == 0 {
            return 0.1;
        }
        let log_freq = (freq as f32).ln();
        (log_freq / 10.0).min(1.0)
    }

    fn world_score(&self, token: &str) -> f32 {
        if is_content_word(token) {
            0.6
        } else {
            0.3
        }
    }

    fn verification_score(&self, token: &str) -> f32 {
        let freq = self.vocab_frequencies.get(token).copied().unwrap_or(0);
        if freq > 10 {
            0.8
        } else if freq > 5 {
            0.6
        } else if freq > 0 {
            0.4
        } else {
            0.2
        }
    }
}

fn is_content_word(token: &str) -> bool {
    matches!(
        token,
        "time" | "year" | "people" | "day" | "man" | "child" | "world"
            | "life" | "hand" | "place" | "work" | "number" | "night"
            | "home" | "room" | "story" | "book" | "word" | "car"
            | "computer" | "data" | "model" | "concept" | "system"
            | "process" | "state" | "input" | "text" | "meaning"
            | "language" | "function" | "result" | "method" | "type"
            | "value" | "error" | "file" | "code" | "test" | "user"
            | "application" | "network" | "memory" | "algorithm"
    )
}
