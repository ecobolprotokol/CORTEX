pub mod fetch;
pub mod parse;

pub use fetch::Fetcher;
pub use parse::ContentParser;

use crate::error::CortexError;

pub trait InternetInterface {
    fn fetch(&self, url: &str) -> Result<String, CortexError>;
    fn parse(&self, content: &str) -> Result<String, CortexError>;
}

pub struct InternetClient {
    pub fetcher: Fetcher,
    pub parser: ContentParser,
}

impl InternetClient {
    pub fn new(timeout_seconds: u32, max_response_mb: u32) -> Self {
        Self {
            fetcher: Fetcher::new(timeout_seconds, max_response_mb),
            parser: ContentParser::new(),
        }
    }

    pub fn fetch_and_parse(&self, url: &str) -> Result<String, CortexError> {
        let raw = self.fetcher.fetch(url)?;
        let parsed = self.parser.extract_text(&raw);
        Ok(parsed)
    }
}
