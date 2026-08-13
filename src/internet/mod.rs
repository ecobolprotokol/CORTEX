pub mod fetch;
pub mod parse;

use crate::error::CortexError;

pub trait InternetInterface {
    fn fetch(&self, url: &str) -> Result<String, CortexError>;
    fn parse(&self, content: &str) -> Result<String, CortexError>;
}
