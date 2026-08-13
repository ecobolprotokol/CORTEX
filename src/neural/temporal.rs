use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct TemporalBuffer {
    pub history: Vec<Vec<Scalar>>,
    pub max_size: usize,
}

impl TemporalBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            history: Vec::new(),
            max_size,
        }
    }

    pub fn encode(&mut self, activation: Vec<Scalar>) {
        self.history.push(activation);
        if self.history.len() > self.max_size {
            self.history.remove(0);
        }
    }

    pub fn last_n(&self, n: usize) -> &[Vec<Scalar>] {
        let start = self.history.len().saturating_sub(n);
        &self.history[start..]
    }
}
