use crate::types::*;
use std::collections::BinaryHeap;

pub fn select_replay_candidates(experiences: &[Experience], max_count: usize) -> Vec<&Experience> {
    let mut scored: Vec<(usize, Scalar)> = experiences.iter().enumerate()
        .map(|(i, e)| (i, e.error.magnitude * 0.4 + e.observation.importance * 0.3 + 0.3))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.iter().take(max_count).map(|(i, _)| &experiences[*i]).collect()
}
