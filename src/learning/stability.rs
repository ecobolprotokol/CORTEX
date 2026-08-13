pub fn is_safe(signal_magnitude: f32, average_error: f32) -> bool {
    if average_error < 1e-6 {
        return signal_magnitude < 0.5;
    }
    signal_magnitude < average_error * 3.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_small_error() {
        assert!(is_safe(0.1, 0.0));
    }

    #[test]
    fn test_unsafe_large_error() {
        assert!(!is_safe(1.0, 0.1));
    }

    #[test]
    fn test_safe_within_bounds() {
        assert!(is_safe(0.2, 0.1));
    }
}
