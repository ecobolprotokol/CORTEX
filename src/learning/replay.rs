use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct ReplayEntry {
    pub experience: String,
    pub priority: Scalar,
    pub timestamp: u64,
    pub access_count: u32,
    pub last_accessed: Option<u64>,
}

impl ReplayEntry {
    pub fn new(experience: &str, priority: Scalar) -> Self {
        Self {
            experience: experience.to_string(),
            priority,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            access_count: 0,
            last_accessed: None,
        }
    }

    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        );
    }

    pub fn effective_priority(&self) -> Scalar {
        let recency_bonus = if let Some(last) = self.last_accessed {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            let age_ms = now.saturating_sub(last);
            (1.0 - (age_ms as Scalar / 3600000.0).min(1.0)) * 0.2
        } else {
            0.1
        };

        let frequency_bonus = (self.access_count as Scalar * 0.05).min(0.3);

        self.priority + recency_bonus + frequency_bonus
    }
}

pub struct ReplayBuffer {
    pub entries: Vec<ReplayEntry>,
    pub max_size: usize,
    pub total_sampled: u64,
}

impl ReplayBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
            total_sampled: 0,
        }
    }

    pub fn add(&mut self, experience: &str, priority: Scalar) {
        if self.entries.len() >= self.max_size {
            if let Some(pos) = self
                .entries
                .iter()
                .enumerate()
                .min_by(|a, b| {
                    a.1.priority
                        .partial_cmp(&b.1.priority)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
            {
                if self.entries[pos].priority < priority {
                    self.entries.remove(pos);
                } else {
                    return;
                }
            }
        }

        self.entries
            .push(ReplayEntry::new(experience, priority));
    }

    pub fn sample(&mut self, n: usize) -> Vec<ReplayEntry> {
        let sample_size = n.min(self.entries.len());
        if sample_size == 0 {
            return Vec::new();
        }

        let mut indexed: Vec<(usize, Scalar)> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.effective_priority()))
            .collect();

        indexed.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let selected: Vec<ReplayEntry> = indexed
            .iter()
            .take(sample_size)
            .map(|(idx, _)| {
                let mut entry = self.entries[*idx].clone();
                entry.access();
                self.entries[*idx].access();
                entry
            })
            .collect();

        self.total_sampled += sample_size as u64;
        selected
    }

    pub fn sample_uniform(&self, n: usize) -> Vec<&ReplayEntry> {
        if self.entries.is_empty() {
            return Vec::new();
        }

        let sample_size = n.min(self.entries.len());
        let step = if sample_size == 0 {
            1
        } else {
            self.entries.len() / sample_size
        };

        self.entries
            .iter()
            .step_by(step.max(1))
            .take(sample_size)
            .collect()
    }

    pub fn update_priority(&mut self, experience: &str, new_priority: Scalar) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.experience == experience)
        {
            entry.priority = new_priority;
        }
    }

    pub fn remove_low_priority(&mut self, threshold: Scalar) {
        self.entries.retain(|e| e.priority >= threshold);
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn capacity_remaining(&self) -> usize {
        self.max_size.saturating_sub(self.entries.len())
    }

    pub fn average_priority(&self) -> Scalar {
        if self.entries.is_empty() {
            return 0.0;
        }
        self.entries.iter().map(|e| e.priority).sum::<Scalar>() / self.entries.len() as Scalar
    }

    pub fn highest_priority(&self) -> Scalar {
        self.entries
            .iter()
            .map(|e| e.priority)
            .fold(0.0f32, Scalar::max)
    }

    pub fn lowest_priority(&self) -> Scalar {
        self.entries
            .iter()
            .map(|e| e.priority)
            .fold(1.0f32, Scalar::min)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.total_sampled = 0;
    }
}
