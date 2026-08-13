use crate::error::CortexError;

pub struct RequestHandler;

impl RequestHandler {
    pub fn new() -> Self { Self }

    pub fn handle_inference(&self, input: &str) -> Result<String, CortexError> {
        Ok(format!("Response to: {}", input))
    }

    pub fn handle_observe(&self, observation: &str) -> Result<String, CortexError> {
        Ok(format!("Observed: {}", observation))
    }

    pub fn handle_query(&self, query: &str) -> Result<String, CortexError> {
        Ok(format!("Query result for: {}", query))
    }

    pub fn handle_status(&self) -> Result<String, CortexError> {
        Ok("Status: Ready".into())
    }
}
