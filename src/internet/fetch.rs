use crate::error::{CortexError, Result};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    InvalidUrl(String),
    ContentTooLarge { size: usize, limit: usize },
    RateLimited { retry_after_secs: u64 },
    Timeout,
    NotFound,
    ServerError(u16),
    HashMismatch { expected: String, actual: String },
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            FetchError::ContentTooLarge { size, limit } => {
                write!(f, "Content too large: {} bytes exceeds limit of {} bytes", size, limit)
            }
            FetchError::RateLimited { retry_after_secs } => {
                write!(f, "Rate limited, retry after {} seconds", retry_after_secs)
            }
            FetchError::Timeout => write!(f, "Request timed out"),
            FetchError::NotFound => write!(f, "Resource not found"),
            FetchError::ServerError(code) => write!(f, "Server error: {}", code),
            FetchError::HashMismatch { expected, actual } => {
                write!(f, "Content hash mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for FetchError {}

pub struct RateLimiter {
    max_requests: usize,
    window_secs: u64,
    timestamps: VecDeque<u64>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            max_requests,
            window_secs,
            timestamps: VecDeque::new(),
        }
    }

    pub fn check(&mut self) -> std::result::Result<(), u64> {
        let now = now_secs();
        while let Some(&front) = self.timestamps.front() {
            if now.saturating_sub(front) >= self.window_secs {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
        if self.timestamps.len() >= self.max_requests {
            let oldest = self.timestamps.front().copied().unwrap_or(now);
            return Err(self.window_secs.saturating_sub(now.saturating_sub(oldest)).max(1));
        }
        self.timestamps.push_back(now);
        Ok(())
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn validate_url(url: &str) -> Result<()> {
    if url.is_empty() {
        return Err(CortexError::NetworkError("URL is empty".into()));
    }
    let trimmed = url.trim();
    let has_scheme = trimmed.starts_with("http://") || trimmed.starts_with("https://");
    if !has_scheme {
        return Err(CortexError::NetworkError(format!(
            "URL must start with http:// or https://: {}",
            trimmed
        )));
    }
    let after_scheme = &trimmed[trimmed.find("://").unwrap() + 3..];
    if after_scheme.is_empty() {
        return Err(CortexError::NetworkError(format!(
            "URL has no host: {}",
            trimmed
        )));
    }
    let host_end = after_scheme.find(|c: char| c == '/' || c == '?' || c == '#').unwrap_or(after_scheme.len());
    let host = &after_scheme[..host_end];
    if host.is_empty() {
        return Err(CortexError::NetworkError(format!(
            "URL has empty host: {}",
            trimmed
        )));
    }
    if host.starts_with('.') || host.ends_with('.') || host.contains("..") {
        return Err(CortexError::NetworkError(format!(
            "URL host is invalid: {}",
            host
        )));
    }
    for ch in host.chars() {
        if !ch.is_alphanumeric() && ch != '-' && ch != '.' && ch != ':' {
            if !(ch == '[' && host.ends_with(']')) {
                return Err(CortexError::NetworkError(format!(
                    "URL host contains invalid character '{}': {}",
                    ch, host
                )));
            }
        }
    }
    Ok(())
}

pub fn check_content_size(content: &str, max_response_mb: u32) -> Result<()> {
    let size_bytes = content.len();
    let limit_bytes = max_response_mb as usize * 1024 * 1024;
    if size_bytes > limit_bytes {
        return Err(CortexError::ResourceError(format!(
            "Content size {} bytes exceeds limit of {} bytes ({} MB)",
            size_bytes, limit_bytes, max_response_mb
        )));
    }
    Ok(())
}

pub fn compute_content_hash(content: &str) -> String {
    let hash = blake3::hash(content.as_bytes());
    hash.to_hex().to_string()
}

pub fn verify_content_hash(content: &str, expected_hash: &str) -> Result<()> {
    let actual = compute_content_hash(content);
    if actual != expected_hash {
        return Err(CortexError::NetworkError(format!(
            "Content hash mismatch: expected {}, got {}",
            expected_hash, actual
        )));
    }
    Ok(())
}

pub fn fetch_url(
    url: &str,
    timeout_secs: u32,
    max_response_mb: u32,
    rate_limiter: Option<&mut RateLimiter>,
) -> Result<FetchResponse> {
    validate_url(url)?;

    if let Some(limiter) = rate_limiter {
        if let Err(retry_after) = limiter.check() {
            return Err(CortexError::ResourceError(format!(
                "Rate limited, retry after {} seconds",
                retry_after
            )));
        }
    }

    let content = simulate_fetch(url)?;
    check_content_size(&content, max_response_mb)?;

    let content_hash = compute_content_hash(&content);
    let content_type = detect_content_type(&content);

    let content_length = content.len() as u64;

    Ok(FetchResponse {
        url: url.to_string(),
        status: 200,
        content,
        content_type,
        content_hash,
        content_length,
        timeout_secs,
    })
}

fn simulate_fetch(url: &str) -> Result<String> {
    if url.contains("404") {
        return Err(CortexError::NetworkError("Resource not found (404)".into()));
    }
    if url.contains("500") {
        return Err(CortexError::NetworkError("Internal server error (500)".into()));
    }
    if url.contains("timeout") {
        return Err(CortexError::NetworkError("Request timed out".into()));
    }
    Ok(format!(
        "<html><head><title>Simulated Page</title></head><body>Content from {}</body></html>",
        url
    ))
}

fn detect_content_type(content: &str) -> String {
    if content.starts_with("<!DOCTYPE") || content.starts_with("<html") {
        "text/html".to_string()
    } else if content.starts_with('{') || content.starts_with('[') {
        "application/json".to_string()
    } else {
        "text/plain".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FetchResponse {
    pub url: String,
    pub status: u16,
    pub content: String,
    pub content_type: String,
    pub content_hash: String,
    pub content_length: u64,
    pub timeout_secs: u32,
}

pub fn estimate_content_size(content: &str) -> usize {
    content.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com/path").is_ok());
        assert!(validate_url("https://example.com/path?q=1#frag").is_ok());
    }

    #[test]
    fn test_validate_url_empty() {
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_validate_url_no_scheme() {
        assert!(validate_url("example.com").is_err());
    }

    #[test]
    fn test_validate_url_no_host() {
        assert!(validate_url("http://").is_err());
    }

    #[test]
    fn test_validate_url_invalid_host() {
        assert!(validate_url("http://.example.com").is_err());
        assert!(validate_url("http://example..com").is_err());
        assert!(validate_url("http://example.").is_err());
    }

    #[test]
    fn test_check_content_size_ok() {
        assert!(check_content_size("hello", 1).is_ok());
    }

    #[test]
    fn test_check_content_size_too_large() {
        let content = "x".repeat(2 * 1024 * 1024 + 1);
        assert!(check_content_size(&content, 2).is_err());
    }

    #[test]
    fn test_compute_content_hash_deterministic() {
        let h1 = compute_content_hash("hello world");
        let h2 = compute_content_hash("hello world");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_compute_content_hash_different() {
        let h1 = compute_content_hash("hello");
        let h2 = compute_content_hash("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_verify_content_hash_ok() {
        let content = "test data";
        let hash = compute_content_hash(content);
        assert!(verify_content_hash(content, &hash).is_ok());
    }

    #[test]
    fn test_verify_content_hash_mismatch() {
        assert!(verify_content_hash("data", "wrong_hash").is_err());
    }

    #[test]
    fn test_fetch_url_ok() {
        let result = fetch_url("https://example.com", 15, 4, None);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
        assert!(resp.content.contains("example.com"));
        assert_eq!(resp.content_type, "text/html");
    }

    #[test]
    fn test_fetch_url_404() {
        assert!(fetch_url("https://example.com/404", 15, 4, None).is_err());
    }

    #[test]
    fn test_fetch_url_500() {
        assert!(fetch_url("https://example.com/500", 15, 4, None).is_err());
    }

    #[test]
    fn test_fetch_url_invalid_url() {
        assert!(fetch_url("not-a-url", 15, 4, None).is_err());
    }

    #[test]
    fn test_fetch_url_content_too_large() {
        let big_url = format!("https://example.com/{}", "a".repeat(5 * 1024 * 1024));
        let result = fetch_url(&big_url, 15, 1, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_url_with_rate_limiter() {
        let mut limiter = RateLimiter::new(2, 60);
        assert!(fetch_url("https://example.com", 15, 4, Some(&mut limiter)).is_ok());
        assert!(fetch_url("https://example.com/path1", 15, 4, Some(&mut limiter)).is_ok());
        let result = fetch_url("https://example.com/path2", 15, 4, Some(&mut limiter));
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Rate limited"));
    }

    #[test]
    fn test_rate_limiter_window_expiry() {
        let mut limiter = RateLimiter::new(1, 0);
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_ok());
    }

    #[test]
    fn test_estimate_content_size() {
        assert_eq!(estimate_content_size("hello"), 5);
        assert_eq!(estimate_content_size(""), 0);
    }

    #[test]
    fn test_detect_content_type_html() {
        assert_eq!(detect_content_type("<html>"), "text/html");
        assert_eq!(detect_content_type("<!DOCTYPE html>"), "text/html");
    }

    #[test]
    fn test_detect_content_type_json() {
        assert_eq!(detect_content_type("{\"key\":\"val\"}"), "application/json");
        assert_eq!(detect_content_type("[1,2,3]"), "application/json");
    }

    #[test]
    fn test_detect_content_type_plain() {
        assert_eq!(detect_content_type("hello world"), "text/plain");
    }

    #[test]
    fn test_fetch_response_hash_integrity() {
        let resp = fetch_url("https://example.com", 15, 4, None).unwrap();
        assert!(verify_content_hash(&resp.content, &resp.content_hash).is_ok());
    }

    #[test]
    fn test_fetch_error_display() {
        let err = FetchError::InvalidUrl("bad".into());
        assert!(err.to_string().contains("Invalid URL"));

        let err = FetchError::ContentTooLarge { size: 100, limit: 50 };
        assert!(err.to_string().contains("too large"));

        let err = FetchError::RateLimited { retry_after_secs: 30 };
        assert!(err.to_string().contains("Rate limited"));
    }
}
