use crate::error::Result;
use crate::types::*;

pub fn predict(state: &LanguageState, config: &crate::config::LanguageConfig) -> Result<Vec<CandidateContinuation>> {
    let mut candidates = Vec::new();
    for symbol in state.symbols.iter().take(10) {
        let score = symbol.activation * symbol.confidence;
        candidates.push(CandidateContinuation {
            token: symbol.id,
            score,
        });
    }
    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    candidates.truncate(config.generation_limit as usize);
    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_empty() {
        let state = LanguageState {
            symbols: Vec::new(),
            tokens: Vec::new(),
            vocabulary_size: 0,
            next_symbol_id: SymbolId(1),
        };
        let config = crate::config::LanguageConfig {
            enabled: true,
            vocabulary_capacity: 65536,
            context_window: 4096,
            generation_limit: 1024,
            learning: true,
        };
        let result = predict(&state, &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_predict_with_symbols() {
        let state = LanguageState {
            symbols: vec![
                Symbol { id: SymbolId(1), text: "hello".into(), kind: SymbolKind::Word, frequency: 1, activation: 0.9, confidence: 0.8 },
                Symbol { id: SymbolId(2), text: "world".into(), kind: SymbolKind::Word, frequency: 1, activation: 0.5, confidence: 0.6 },
            ],
            tokens: Vec::new(),
            vocabulary_size: 2,
            next_symbol_id: SymbolId(3),
        };
        let config = crate::config::LanguageConfig {
            enabled: true,
            vocabulary_capacity: 65536,
            context_window: 4096,
            generation_limit: 1024,
            learning: true,
        };
        let result = predict(&state, &config).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].score >= result[1].score);
    }
}
