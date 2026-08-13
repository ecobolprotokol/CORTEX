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

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_record() {
        let mut diag = Diagnostics::new(10);
        diag.record("TestError", "test message", "warning");
        assert_eq!(diag.count(), 1);
    }

    #[test]
    fn test_diagnostics_max_entries() {
        let mut diag = Diagnostics::new(2);
        diag.record("Error1", "msg1", "error");
        diag.record("Error2", "msg2", "error");
        diag.record("Error3", "msg3", "error");
        assert_eq!(diag.count(), 2);
    }

    #[test]
    fn test_diagnostics_recent() {
        let mut diag = Diagnostics::new(10);
        diag.record("Error1", "msg1", "error");
        diag.record("Error2", "msg2", "error");
        let recent = diag.recent(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].kind, "Error2");
    }

    #[test]
    fn test_diagnostics_clear() {
        let mut diag = Diagnostics::new(10);
        diag.record("Error", "msg", "error");
        diag.clear();
        assert_eq!(diag.count(), 0);
    }
}
