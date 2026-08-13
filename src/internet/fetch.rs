use crate::error::CortexError;

pub struct Fetcher {
    pub timeout_seconds: u32,
    pub max_response_mb: u32,
    pub max_retries: u32,
    pub allowed_schemes: Vec<String>,
    pub blocked_domains: Vec<String>,
}

impl Fetcher {
    pub fn new(timeout_seconds: u32, max_response_mb: u32) -> Self {
        Self {
            timeout_seconds,
            max_response_mb,
            max_retries: 3,
            allowed_schemes: vec!["https".into(), "http".into()],
            blocked_domains: Vec::new(),
        }
    }

    pub fn with_timeout(mut self, seconds: u32) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    pub fn with_max_size(mut self, mb: u32) -> Self {
        self.max_response_mb = mb;
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn fetch(&self, url: &str) -> Result<String, CortexError> {
        self.validate_url(url)?;

        let max_bytes = self.max_response_mb as usize * 1024 * 1024;
        let _ = max_bytes;

        tracing::info!("Fetching URL: {}", url);

        for attempt in 0..=self.max_retries {
            match self.attempt_fetch(url) {
                Ok(content) => {
                    tracing::debug!("Fetch succeeded on attempt {} for {}", attempt + 1, url);
                    return Ok(content);
                }
                Err(e) => {
                    if attempt == self.max_retries {
                        return Err(CortexError::NetworkError(format!(
                            "Failed after {} retries: {}",
                            self.max_retries, e
                        )));
                    }
                    tracing::warn!("Fetch attempt {} failed for {}: {}", attempt + 1, url, e);
                }
            }
        }

        Err(CortexError::NetworkError("Exhausted retries".into()))
    }

    fn attempt_fetch(&self, url: &str) -> Result<String, CortexError> {
        Ok(format!(
            "<html><body>Simulated content from {}</body></html>",
            url
        ))
    }

    fn validate_url(&self, url: &str) -> Result<(), CortexError> {
        if url.is_empty() {
            return Err(CortexError::NetworkError("URL cannot be empty".into()));
        }

        let has_valid_scheme = self
            .allowed_schemes
            .iter()
            .any(|scheme| url.starts_with(&format!("{}://", scheme)));

        if !has_valid_scheme {
            return Err(CortexError::NetworkError(format!(
                "URL scheme not allowed: {}",
                url
            )));
        }

        for domain in &self.blocked_domains {
            if url.contains(domain.as_str()) {
                return Err(CortexError::NetworkError(format!(
                    "Domain blocked: {}",
                    domain
                )));
            }
        }

        let max_url_length = 2048;
        if url.len() > max_url_length {
            return Err(CortexError::NetworkError("URL too long".into()));
        }

        Ok(())
    }

    pub fn check_url_allowed(&self, url: &str) -> bool {
        self.validate_url(url).is_ok()
    }
}
