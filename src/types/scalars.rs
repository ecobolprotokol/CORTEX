use serde::{Deserialize, Serialize};

pub type Scalar = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Precision {
    #[default]
    F32,
    F16,
    BF16,
}


pub const SCALAR_EPSILON: Scalar = 1e-6;

pub fn scalar_eq(a: Scalar, b: Scalar) -> bool {
    (a - b).abs() < SCALAR_EPSILON
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataValidationError {
    NonFiniteValue,
    OutOfRange {
        value: Scalar,
        min: Scalar,
        max: Scalar,
    },
}

impl std::fmt::Display for DataValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValidationError::NonFiniteValue => write!(f, "non-finite scalar value"),
            DataValidationError::OutOfRange { value, min, max } => {
                write!(f, "scalar {} out of range [{}, {}]", value, min, max)
            }
        }
    }
}

impl std::error::Error for DataValidationError {}

pub trait ScalarOps {
    fn is_valid_cognitive_value(&self) -> bool;
    fn validate_range(self, min: Scalar, max: Scalar) -> Result<(), DataValidationError>;
}

impl ScalarOps for Scalar {
    fn is_valid_cognitive_value(&self) -> bool {
        self.is_finite()
    }

    fn validate_range(self, min: Scalar, max: Scalar) -> Result<(), DataValidationError> {
        if !self.is_finite() {
            return Err(DataValidationError::NonFiniteValue);
        }
        if self < min || self > max {
            return Err(DataValidationError::OutOfRange {
                value: self,
                min,
                max,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_eq() {
        assert!(scalar_eq(1.0, 1.0));
        assert!(scalar_eq(1.0, 1.0 + SCALAR_EPSILON / 2.0));
        assert!(!scalar_eq(1.0, 2.0));
    }

    #[test]
    fn test_is_valid_cognitive_value() {
        assert!((1.0_f32).is_valid_cognitive_value());
        assert!((0.0_f32).is_valid_cognitive_value());
        assert!(!f32::NAN.is_valid_cognitive_value());
        assert!(!f32::INFINITY.is_valid_cognitive_value());
    }

    #[test]
    fn test_validate_range() {
        assert!((0.5_f32).validate_range(0.0, 1.0).is_ok());
        assert!((1.5_f32).validate_range(0.0, 1.0).is_err());
        assert!(f32::NAN.validate_range(0.0, 1.0).is_err());
    }
}
