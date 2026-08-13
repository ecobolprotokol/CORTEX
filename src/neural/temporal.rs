#[derive(Debug, Clone)]
pub struct TemporalBuffer {
    pub history: Vec<Vec<f32>>,
    pub max_size: usize,
}

impl TemporalBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            history: Vec::new(),
            max_size,
        }
    }

    pub fn encode(&mut self, activation: Vec<f32>) {
        self.history.push(activation);
        if self.history.len() > self.max_size {
            self.history.remove(0);
        }
    }

    pub fn last_n(&self, n: usize) -> &[Vec<f32>] {
        let start = self.history.len().saturating_sub(n);
        &self.history[start..]
    }

    pub fn detect_recurrence(&self) -> Option<(usize, usize)> {
        if self.history.len() < 2 {
            return None;
        }

        let len = self.history.len();
        let latest = &self.history[len - 1];

        for i in 0..len - 1 {
            let candidate = &self.history[i];
            if candidate.len() != latest.len() {
                continue;
            }
            let distance: f32 = candidate
                .iter()
                .zip(latest.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f32>()
                .sqrt();
            if distance < 0.05 {
                return Some((i, len - 1));
            }
        }

        None
    }

    pub fn encode_sequence(&self) -> Vec<f32> {
        if self.history.is_empty() {
            return Vec::new();
        }

        let dim = self.history[0].len();
        let mut encoded = vec![0.0f32; dim];

        for (step, snapshot) in self.history.iter().enumerate() {
            let weight = (step + 1) as f32 / self.history.len() as f32;
            for (i, &val) in snapshot.iter().enumerate() {
                if i < dim {
                    encoded[i] += val * weight;
                }
            }
        }

        let len = self.history.len() as f32;
        for val in &mut encoded {
            *val /= len;
        }

        encoded
    }

    pub fn encode_transition(&self) -> Vec<f32> {
        if self.history.len() < 2 {
            return Vec::new();
        }

        let prev = &self.history[self.history.len() - 2];
        let curr = &self.history[self.history.len() - 1];

        let len = prev.len().min(curr.len());
        let mut transition = Vec::with_capacity(len);

        for i in 0..len {
            transition.push(curr[i] - prev[i]);
        }

        transition
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}
