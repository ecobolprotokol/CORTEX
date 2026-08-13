use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct RetrievalResult {
    pub score: Scalar,
    pub item_type: String,
    pub item_id: u64,
}

pub struct RetrievalEngine;

impl RetrievalEngine {
    pub fn new() -> Self { Self }

    pub fn score_relevance(query: &str, item: &str) -> Scalar {
        let query_words: Vec<&str> = query.split_whitespace().collect();
        let item_words: Vec<&str> = item.split_whitespace().collect();

        let overlap = query_words.iter()
            .filter(|w| item_words.contains(w))
            .count() as Scalar;

        let max_len = query_words.len().max(item_words.len()) as Scalar;
        if max_len > 0.0 { overlap / max_len } else { 0.0 }
    }
}
