pub fn authenticate(request_key: Option<&str>, expected_key: Option<&str>) -> bool {
    match (request_key, expected_key) {
        (Some(req), Some(exp)) => {
            if req.len() != exp.len() {
                return false;
            }
            let mut result = 0u8;
            for (a, b) in req.bytes().zip(exp.bytes()) {
                result |= a ^ b;
            }
            result == 0
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_keys_required() {
        assert!(authenticate(None, None));
    }

    #[test]
    fn test_valid_key() {
        assert!(authenticate(Some("secret123"), Some("secret123")));
    }

    #[test]
    fn test_invalid_key() {
        assert!(!authenticate(Some("wrong"), Some("secret123")));
    }

    #[test]
    fn test_missing_request_key() {
        assert!(!authenticate(None, Some("secret123")));
    }

    #[test]
    fn test_missing_expected_key() {
        assert!(!authenticate(Some("secret123"), None));
    }
}
