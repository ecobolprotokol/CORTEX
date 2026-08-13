use std::collections::HashMap;

use crate::error::Result;
use crate::types::*;
use crate::config::LanguageConfig;

const BIGRAM_WINDOW: usize = 3;
const POSITIONAL_BEGIN_WEIGHT: Scalar = 0.15;
const POSITIONAL_END_WEIGHT: Scalar = 0.15;
const FREQUENCY_BASE: Scalar = 0.01;
const TFIDF_RARE_BOOST: Scalar = 0.30;
const ACTIVATION_WEIGHT: Scalar = 0.35;
const CONFIDENCE_WEIGHT: Scalar = 0.20;
const BIGRAM_WEIGHT: Scalar = 0.30;

struct BigramModel {
    forward: HashMap<SymbolId, HashMap<SymbolId, u64>>,
    backward: HashMap<SymbolId, HashMap<SymbolId, u64>>,
    global_freq: HashMap<SymbolId, u64>,
    total_tokens: u64,
}

impl BigramModel {
    fn from_tokens(tokens: &[Token]) -> Self {
        let mut forward: HashMap<SymbolId, HashMap<SymbolId, u64>> = HashMap::new();
        let mut backward: HashMap<SymbolId, HashMap<SymbolId, u64>> = HashMap::new();
        let mut global_freq: HashMap<SymbolId, u64> = HashMap::new();
        let total_tokens = tokens.len() as u64;

        for token in tokens {
            *global_freq.entry(token.symbol_id).or_insert(0) += 1;
        }

        for window in tokens.windows(2) {
            let prev = window[0].symbol_id;
            let next = window[1].symbol_id;

            *forward.entry(prev).or_default().entry(next).or_insert(0) += 1;
            *backward.entry(next).or_default().entry(prev).or_insert(0) += 1;
        }

        Self {
            forward,
            backward,
            global_freq,
            total_tokens,
        }
    }

    fn successor_score(&self, context: SymbolId, candidate: SymbolId) -> Scalar {
        let forward_score = self
            .forward
            .get(&context)
            .and_then(|m| m.get(&candidate))
            .copied()
            .unwrap_or(0);
        let backward_score = self
            .backward
            .get(&candidate)
            .and_then(|m| m.get(&context))
            .copied()
            .unwrap_or(0);

        let combined = forward_score + backward_score / 2;
        let context_count = self
            .forward
            .get(&context)
            .map(|m| m.values().sum::<u64>())
            .unwrap_or(1)
            .max(1);

        combined as Scalar / context_count as Scalar
    }

    fn frequency(&self, sym: SymbolId) -> Scalar {
        self.global_freq
            .get(&sym)
            .copied()
            .unwrap_or(0) as Scalar
            / self.total_tokens.max(1) as Scalar
    }
}

pub fn predict(state: &LanguageState, config: &LanguageConfig) -> Result<Vec<CandidateContinuation>> {
    if state.symbols.is_empty() {
        return Ok(Vec::new());
    }

    let bigram = BigramModel::from_tokens(&state.tokens);

    let recent_context: Vec<SymbolId> = state
        .tokens
        .iter()
        .rev()
        .take(BIGRAM_WINDOW)
        .map(|t| t.symbol_id)
        .collect();

    let token_count = state.tokens.len();
    let max_activation = state
        .symbols
        .iter()
        .map(|s| s.activation)
        .fold(0.0f32, Scalar::max)
        .max(SCALAR_EPSILON);

    let mut sorted: Vec<CandidateContinuation> = state
        .symbols
        .iter()
        .map(|symbol| {
            let id = symbol.id;

            let activation_score = symbol.activation / max_activation;
            let confidence_score = symbol.confidence;

            let base_frequency = bigram.frequency(id);
            let base_prob = FREQUENCY_BASE + base_frequency * 0.1;

            let mut bigram_score: Scalar = 0.0;
            for (i, &ctx) in recent_context.iter().enumerate() {
                let recency_weight = 1.0 / (i as Scalar + 1.0);
                bigram_score += bigram.successor_score(ctx, id) * recency_weight;
            }
            bigram_score = bigram_score.min(1.0);

            let mut positional_bias: Scalar = 0.0;
            if token_count > 0 {
                for (pos, token) in state.tokens.iter().enumerate() {
                    if token.symbol_id == id {
                        if pos == 0 {
                            positional_bias += POSITIONAL_BEGIN_WEIGHT;
                        }
                        if pos == token_count - 1 {
                            positional_bias += POSITIONAL_END_WEIGHT;
                        }
                    }
                }
            }

            let total_frequency: u64 = state.symbols.iter().map(|s| s.frequency.max(1)).sum();
            let symbol_frequency = symbol.frequency.max(1);
            let rarity_bonus = TFIDF_RARE_BOOST * (1.0 - symbol_frequency as Scalar / total_frequency as Scalar);

            let score = base_prob
                + ACTIVATION_WEIGHT * activation_score
                + CONFIDENCE_WEIGHT * confidence_score
                + BIGRAM_WEIGHT * bigram_score
                + positional_bias
                + rarity_bonus;

            let kind_penalty = match symbol.kind {
                SymbolKind::Punctuation => 0.3,
                SymbolKind::Special => 0.5,
                SymbolKind::Unknown => 0.6,
                _ => 1.0,
            };

            let final_score = score * kind_penalty;

            CandidateContinuation {
                token: id,
                score: final_score,
            }
        })
        .collect();

    sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    sorted.truncate(config.generation_limit as usize);

    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_symbol(id: u64, text: &str, freq: u64, activation: Scalar, confidence: Scalar) -> Symbol {
        Symbol {
            id: SymbolId(id),
            text: text.to_string(),
            kind: SymbolKind::Word,
            frequency: freq,
            activation,
            confidence,
        }
    }

    fn make_token(id: u64, symbol_id: u64, position: u32, weight: Scalar) -> Token {
        Token {
            id: TokenId(id),
            symbol_id: SymbolId(symbol_id),
            position,
            weight,
        }
    }

    fn default_config() -> LanguageConfig {
        LanguageConfig {
            enabled: true,
            vocabulary_capacity: 65536,
            context_window: 4096,
            generation_limit: 1024,
            learning: true,
        }
    }

    #[test]
    fn test_predict_empty() {
        let state = LanguageState {
            symbols: Vec::new(),
            tokens: Vec::new(),
            vocabulary_size: 0,
            next_symbol_id: SymbolId(1),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_predict_single_symbol() {
        let state = LanguageState {
            symbols: vec![make_symbol(1, "hello", 5, 0.8, 0.9)],
            tokens: vec![make_token(0, 1, 0, 0.8)],
            vocabulary_size: 1,
            next_symbol_id: SymbolId(2),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].score > 0.0);
    }

    #[test]
    fn test_predict_with_symbols() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "hello", 10, 0.9, 0.8),
                make_symbol(2, "world", 5, 0.5, 0.6),
            ],
            tokens: Vec::new(),
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].score >= result[1].score);
    }

    #[test]
    fn test_predict_sorted_by_score() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "low", 1, 0.2, 0.3),
                make_symbol(2, "high", 100, 0.95, 0.95),
                make_symbol(3, "mid", 10, 0.5, 0.5),
            ],
            tokens: Vec::new(),
            vocabulary_size: 3,
            next_symbol_id: SymbolId(4),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result[0].score >= result[1].score);
        assert!(result[1].score >= result[2].score);
    }

    #[test]
    fn test_predict_truncated_to_generation_limit() {
        let symbols: Vec<Symbol> = (1..100)
            .map(|i| make_symbol(i, &format!("w{}", i), i as u64, 0.5, 0.5))
            .collect();
        let state = LanguageState {
            symbols,
            tokens: Vec::new(),
            vocabulary_size: 99,
            next_symbol_id: SymbolId(100),
        };
        let config = LanguageConfig {
            enabled: true,
            vocabulary_capacity: 65536,
            context_window: 4096,
            generation_limit: 5,
            learning: true,
        };
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_bigram_successor_preference() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "a", 50, 0.7, 0.8),
                make_symbol(2, "b", 20, 0.6, 0.7),
                make_symbol(3, "c", 20, 0.6, 0.7),
                make_symbol(4, "z", 1, 0.3, 0.3),
            ],
            tokens: vec![
                make_token(0, 1, 0, 0.7),
                make_token(1, 2, 1, 0.6),
                make_token(2, 1, 2, 0.7),
                make_token(3, 2, 3, 0.6),
            ],
            vocabulary_size: 4,
            next_symbol_id: SymbolId(5),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert!(!result.is_empty());
        let scores: HashMap<SymbolId, Scalar> = result.into_iter().map(|c| (c.token, c.score)).collect();
        let b_score = scores.get(&SymbolId(2)).unwrap();
        let z_score = scores.get(&SymbolId(4)).unwrap();
        assert!(b_score > z_score, "bigram successor b={} should score higher than z={}", b_score, z_score);
    }

    #[test]
    fn test_positional_bias() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "start", 1, 0.5, 0.5),
                make_symbol(2, "middle", 1, 0.5, 0.5),
                make_symbol(3, "end", 1, 0.5, 0.5),
            ],
            tokens: vec![
                make_token(0, 1, 0, 0.5),
                make_token(1, 2, 1, 0.5),
                make_token(2, 3, 2, 0.5),
            ],
            vocabulary_size: 3,
            next_symbol_id: SymbolId(4),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 3);
        let start_score = result.iter().find(|c| c.token == SymbolId(1)).unwrap().score;
        let end_score = result.iter().find(|c| c.token == SymbolId(3)).unwrap().score;
        let middle_score = result.iter().find(|c| c.token == SymbolId(2)).unwrap().score;
        assert!(start_score > middle_score);
        assert!(end_score > middle_score);
    }

    #[test]
    fn test_activation_and_confidence_matter() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "strong", 1, 0.99, 0.99),
                make_symbol(2, "weak", 1, 0.1, 0.1),
            ],
            tokens: Vec::new(),
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].token == SymbolId(1));
    }

    #[test]
    fn test_tf_idf_rare_word_boosted() {
        let state = LanguageState {
            symbols: vec![
                Symbol {
                    id: SymbolId(1),
                    text: "common".into(),
                    kind: SymbolKind::Word,
                    frequency: 100,
                    activation: 0.5,
                    confidence: 0.5,
                },
                Symbol {
                    id: SymbolId(2),
                    text: "rare".into(),
                    kind: SymbolKind::Word,
                    frequency: 1,
                    activation: 0.5,
                    confidence: 0.5,
                },
            ],
            tokens: vec![
                make_token(0, 1, 0, 0.5),
                make_token(1, 2, 1, 0.5),
            ],
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        let rare = result.iter().find(|c| c.token == SymbolId(2)).unwrap();
        let common = result.iter().find(|c| c.token == SymbolId(1)).unwrap();
        assert!(rare.score > common.score,
            "rare ({}) should score higher than common ({})", rare.score, common.score);
    }

    #[test]
    fn test_punctuation_penalized() {
        let state = LanguageState {
            symbols: vec![
                Symbol {
                    id: SymbolId(1),
                    text: "word".into(),
                    kind: SymbolKind::Word,
                    frequency: 10,
                    activation: 0.8,
                    confidence: 0.8,
                },
                Symbol {
                    id: SymbolId(2),
                    text: ",".into(),
                    kind: SymbolKind::Punctuation,
                    frequency: 10,
                    activation: 0.8,
                    confidence: 0.8,
                },
            ],
            tokens: Vec::new(),
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].token == SymbolId(1));
    }

    #[test]
    fn test_special_symbol_heavily_penalized() {
        let state = LanguageState {
            symbols: vec![
                Symbol {
                    id: SymbolId(1),
                    text: "word".into(),
                    kind: SymbolKind::Word,
                    frequency: 10,
                    activation: 0.8,
                    confidence: 0.8,
                },
                Symbol {
                    id: SymbolId(2),
                    text: "<eos>".into(),
                    kind: SymbolKind::Special,
                    frequency: 10,
                    activation: 0.8,
                    confidence: 0.8,
                },
            ],
            tokens: Vec::new(),
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].token == SymbolId(1));
        assert!(result[0].score > result[1].score);
    }

    #[test]
    fn test_bigram_window_context() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "I", 5, 0.7, 0.8),
                make_symbol(2, "love", 5, 0.7, 0.8),
                make_symbol(3, "rust", 5, 0.7, 0.8),
                make_symbol(4, "dogs", 5, 0.7, 0.8),
            ],
            tokens: vec![
                make_token(0, 1, 0, 0.7),
                make_token(1, 2, 1, 0.7),
                make_token(2, 3, 2, 0.7),
                make_token(3, 1, 3, 0.7),
                make_token(4, 2, 4, 0.7),
            ],
            vocabulary_size: 4,
            next_symbol_id: SymbolId(5),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert!(!result.is_empty());
        let top_ids: Vec<SymbolId> = result.iter().take(2).map(|c| c.token).collect();
        assert!(top_ids.contains(&SymbolId(3)));
    }

    #[test]
    fn test_generation_limit_enforced() {
        let symbols: Vec<Symbol> = (1..50)
            .map(|i| make_symbol(i, &format!("t{}", i), 1, 0.5, 0.5))
            .collect();
        let state = LanguageState {
            symbols,
            tokens: Vec::new(),
            vocabulary_size: 49,
            next_symbol_id: SymbolId(50),
        };
        let config = LanguageConfig {
            enabled: true,
            vocabulary_capacity: 65536,
            context_window: 4096,
            generation_limit: 10,
            learning: true,
        };
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_all_scores_positive() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "alpha", 1, 0.1, 0.1),
                make_symbol(2, "beta", 1, 0.1, 0.1),
                Symbol {
                    id: SymbolId(3),
                    text: "!".into(),
                    kind: SymbolKind::Punctuation,
                    frequency: 1,
                    activation: 0.1,
                    confidence: 0.1,
                },
            ],
            tokens: Vec::new(),
            vocabulary_size: 3,
            next_symbol_id: SymbolId(4),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        for c in &result {
            assert!(c.score >= 0.0, "score {} should be non-negative", c.score);
        }
    }

    #[test]
    fn test_multiple_bigram_contexts() {
        let state = LanguageState {
            symbols: vec![
                make_symbol(1, "a", 10, 0.6, 0.7),
                make_symbol(2, "b", 10, 0.6, 0.7),
                make_symbol(3, "c", 10, 0.6, 0.7),
                make_symbol(4, "d", 1, 0.6, 0.7),
            ],
            tokens: vec![
                make_token(0, 1, 0, 0.6),
                make_token(1, 2, 1, 0.6),
                make_token(2, 1, 2, 0.6),
                make_token(3, 2, 3, 0.6),
                make_token(4, 3, 4, 0.6),
            ],
            vocabulary_size: 4,
            next_symbol_id: SymbolId(5),
        };
        let config = default_config();
        let result = predict(&state, &config).unwrap();
        assert!(!result.is_empty());
        let scores: HashMap<SymbolId, Scalar> = result.into_iter().map(|c| (c.token, c.score)).collect();
        assert!(scores.get(&SymbolId(3)).unwrap() > scores.get(&SymbolId(4)).unwrap());
    }
}
