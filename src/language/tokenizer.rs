use crate::error::CortexError;

pub struct Tokenizer {
    pub tokens: Vec<String>,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn normalize(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_whitespace = false;

        for ch in text.chars() {
            if ch.is_control() {
                if !prev_whitespace {
                    result.push(' ');
                    prev_whitespace = true;
                }
                continue;
            }
            if ch.is_whitespace() {
                if !prev_whitespace {
                    result.push(' ');
                    prev_whitespace = true;
                }
            } else {
                let lowered = ch.to_lowercase();
                for lc in lowered {
                    result.push(lc);
                }
                prev_whitespace = false;
            }
        }

        result.trim().to_string()
    }

    pub fn segment(&self, normalized: &str) -> Vec<String> {
        let mut segments = Vec::new();
        let mut current = String::new();

        for ch in normalized.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                current.push(ch);
            } else if ch.is_whitespace() {
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
            } else if ch == '.' || ch == ',' || ch == '!' || ch == '?' || ch == ';' || ch == ':' {
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
                segments.push(ch.to_string());
            } else {
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
                current.push(ch);
            }
        }

        if !current.is_empty() {
            segments.push(current);
        }

        segments
    }

    pub fn tokenize(&mut self, text: &str) -> Result<Vec<String>, CortexError> {
        let normalized = self.normalize(text);
        let segmented = self.segment(&normalized);
        self.tokens = segmented.clone();
        Ok(segmented)
    }
}
