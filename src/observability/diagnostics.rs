use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DiagnosticEntry {
    pub kind: String,
    pub message: String,
    pub timestamp: u64,
    pub severity: String,
}

pub struct Diagnostics {
    entries: VecDeque<DiagnosticEntry>,
    max_entries: usize,
}

impl Diagnostics {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
        }
    }

    pub fn record(&mut self, kind: &str, message: &str, severity: &str) {
        let entry = DiagnosticEntry {
            kind: kind.to_string(),
            message: message.to_string(),
            timestamp: crate::types::Timestamp::now().0,
            severity: severity.to_string(),
        };
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn recent(&self, count: usize) -> Vec<&DiagnosticEntry> {
        self.entries.iter().rev().take(count).collect()
    }
}
