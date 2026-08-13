use crate::error::Result;

pub fn fetch_url(url: &str, timeout_secs: u32) -> Result<String> {
    Ok(format!("Content from {}", url))
}
