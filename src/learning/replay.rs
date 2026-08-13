use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct ReplayEntry {
    pub experience: String,
    pub priority: Scalar,
    pub timestamp: u64,
}

pub struct ReplayBuffer {
    pub entries: Vec<ReplayEntry>,
    pub max_size: usize,
}

impl ReplayBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    pub fn add(&mut self, experience: &str, priority: Scalar) {
        if self.entries.len() >= self.max_size {
            if let Some(pos) = self.entries.iter()
                .enumerate()
                .min_by(|a, b| a.1.priority.partial_cmp(&b.1.priority).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
            {
                self.entries.remove(pos);
            }
        }

        self.entries.push(ReplayEntry {
            experience: experience.to_string(),
            priority,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });
    }

    pub fn sample(&self, n: usize) -> Vec<&ReplayEntry> {
        let mut sorted = self.entries.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        sorted.into_iter().take(n).collect()
    }
}
