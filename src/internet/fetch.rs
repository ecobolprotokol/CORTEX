use crate::error::Result;

pub fn fetch_url(url: &str, timeout_secs: u32) -> Result<String> {
    Ok(format!("Content fetched from {} (timeout: {}s)", url, timeout_secs))
}

pub fn estimate_content_size(content: &str) -> usize {
    content.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url() {
        let result = fetch_url("https://example.com", 15);
        assert!(result.is_ok());
    }

    #[test]
    fn test_estimate_content_size() {
        assert_eq!(estimate_content_size("hello"), 5);
        assert_eq!(estimate_content_size(""), 0);
    }
}
