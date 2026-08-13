use crate::config::LanguageConfig;
use crate::error::Result;
use crate::types::*;

pub fn predict(state: &LanguageState, config: &LanguageConfig) -> Result<Vec<CandidateContinuation>> {
    let mut candidates = Vec::new();
    for symbol in state.symbols.iter().take(10) {
        candidates.push(CandidateContinuation {
            token: symbol.id,
            score: symbol.activation * symbol.confidence,
        });
    }
    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    candidates.truncate(config.generation_limit as usize);
    Ok(candidates)
}
