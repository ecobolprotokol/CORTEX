use crate::error::CortexError;

pub struct Authenticator {
    pub api_key: String,
    pub valid_tokens: Vec<String>,
    pub revoked_tokens: Vec<String>,
}

impl Authenticator {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            valid_tokens: vec![api_key.to_string()],
            revoked_tokens: Vec::new(),
        }
    }

    pub fn validate(&self, token: &str) -> Result<(), CortexError> {
        if self.revoked_tokens.contains(&token.to_string()) {
            return Err(CortexError::PolicyError("Token has been revoked".into()));
        }

        if token == self.api_key || self.valid_tokens.contains(&token.to_string()) {
            Ok(())
        } else {
            Err(CortexError::PolicyError("Invalid API key".into()))
        }
    }

    pub fn add_token(&mut self, token: &str) {
        if !self.valid_tokens.contains(&token.to_string()) {
            self.valid_tokens.push(token.to_string());
        }
    }

    pub fn revoke_token(&mut self, token: &str) {
        if !self.revoked_tokens.contains(&token.to_string()) {
            self.revoked_tokens.push(token.to_string());
        }
        self.valid_tokens.retain(|t| t != token);
    }

    pub fn is_valid(&self, token: &str) -> bool {
        !self.revoked_tokens.contains(&token.to_string())
            && (token == self.api_key || self.valid_tokens.contains(&token.to_string()))
    }

    pub fn token_count(&self) -> usize {
        self.valid_tokens.len()
    }

    pub fn validate_bearer(&self, authorization_header: &str) -> Result<(), CortexError> {
        let token = authorization_header
            .strip_prefix("Bearer ")
            .unwrap_or(authorization_header);
        self.validate(token)
    }

    pub fn extract_token(authorization_header: &str) -> Option<&str> {
        authorization_header.strip_prefix("Bearer ")
    }
}
