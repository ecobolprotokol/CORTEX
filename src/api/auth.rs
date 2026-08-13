use crate::error::CortexError;

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

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
        if self
            .revoked_tokens
            .iter()
            .any(|t| constant_time_eq(t.as_bytes(), token.as_bytes()))
        {
            return Err(CortexError::PolicyError("Token has been revoked".into()));
        }

        if constant_time_eq(token.as_bytes(), self.api_key.as_bytes())
            || self
                .valid_tokens
                .iter()
                .any(|t| constant_time_eq(t.as_bytes(), token.as_bytes()))
        {
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
        !self
            .revoked_tokens
            .iter()
            .any(|t| constant_time_eq(t.as_bytes(), token.as_bytes()))
            && (constant_time_eq(token.as_bytes(), self.api_key.as_bytes())
                || self
                    .valid_tokens
                    .iter()
                    .any(|t| constant_time_eq(t.as_bytes(), token.as_bytes())))
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
