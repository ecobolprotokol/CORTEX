use crate::error::CortexError;

pub struct Authenticator {
    pub api_key: String,
}

impl Authenticator {
    pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string() }
    }

    pub fn validate(&self, token: &str) -> Result<(), CortexError> {
        if token == self.api_key {
            Ok(())
        } else {
            Err(CortexError::PolicyError("Invalid API key".into()))
        }
    }
}
