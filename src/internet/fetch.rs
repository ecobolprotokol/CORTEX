use crate::error::CortexError;

pub struct Fetcher {
    pub timeout_seconds: u32,
    pub max_response_mb: u32,
}

impl Fetcher {
    pub fn new(timeout_seconds: u32, max_response_mb: u32) -> Self {
        Self { timeout_seconds, max_response_mb }
    }

    pub fn fetch(&self, url: &str) -> Result<String, CortexError> {
        Ok(format!("Content from {}", url))
    }
}
