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

/// A confidence value clamped to [0.0, 1.0].
/// This newtype enforces the invariant that confidence values
/// are always within the valid range.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(f32);

impl Confidence {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);
    pub const HALF: Self = Self(0.5);

    pub fn new(value: f32) -> Self {
        if value.is_nan() {
            Self(0.0)
        } else {
            Self(value.clamp(0.0, 1.0))
        }
    }

    pub fn raw(self) -> f32 {
        self.0
    }

    pub fn is_zero(self) -> bool {
        self.0 <= SCALAR_EPSILON
    }

    pub fn is_one(self) -> bool {
        (self.0 - 1.0).abs() <= SCALAR_EPSILON
    }

    pub fn is_above(self, threshold: f32) -> bool {
        self.0 > threshold
    }

    pub fn is_above_threshold(self, threshold: Confidence) -> bool {
        self.0 > threshold.0
    }

    pub fn weighted_average(self, other: Confidence, self_weight: f32) -> Self {
        let w = self_weight.clamp(0.0, 1.0);
        Self(self.0 * w + other.0 * (1.0 - w))
    }

    pub fn combine_support(self, support: Confidence) -> Confidence {
        let combined = self.0 * 0.6 + support.0 * 0.4;
        Self(combined.clamp(0.0, 1.0))
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

impl From<f32> for Confidence {
    fn from(v: f32) -> Self {
        Self::new(v)
    }
}

impl From<Confidence> for f32 {
    fn from(c: Confidence) -> Self {
        c.0
    }
}

#[cfg(test)]
mod confidence_tests {
    use super::*;

    #[test]
    fn test_confidence_clamped_above() {
        let c = Confidence::new(1.5);
        assert_eq!(c.raw(), 1.0);
    }

    #[test]
    fn test_confidence_clamped_below() {
        let c = Confidence::new(-0.5);
        assert_eq!(c.raw(), 0.0);
    }

    #[test]
    fn test_confidence_in_range() {
        let c = Confidence::new(0.7);
        assert_eq!(c.raw(), 0.7);
    }

    #[test]
    fn test_confidence_constants() {
        assert_eq!(Confidence::ZERO.raw(), 0.0);
        assert_eq!(Confidence::ONE.raw(), 1.0);
        assert_eq!(Confidence::HALF.raw(), 0.5);
    }

    #[test]
    fn test_confidence_default() {
        let c = Confidence::default();
        assert_eq!(c, Confidence::ZERO);
    }

    #[test]
    fn test_confidence_weighted_average() {
        let a = Confidence::new(0.8);
        let b = Confidence::new(0.4);
        let result = a.weighted_average(b, 0.5);
        assert!((result.raw() - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_confidence_combine_support() {
        let base = Confidence::new(0.5);
        let support = Confidence::new(0.8);
        let combined = base.combine_support(support);
        assert!((combined.raw() - 0.62).abs() < 0.001);
    }

    #[test]
    fn test_confidence_is_zero() {
        assert!(Confidence::ZERO.is_zero());
        assert!(Confidence::new(0.0000001).is_zero());
        assert!(!Confidence::new(0.1).is_zero());
    }

    #[test]
    fn test_confidence_display() {
        let c = Confidence::new(0.75);
        assert_eq!(format!("{}", c), "0.7500");
    }

    #[test]
    fn test_confidence_from_f32() {
        let c: Confidence = 0.5.into();
        assert_eq!(c, Confidence::HALF);
    }

    #[test]
    fn test_confidence_into_f32() {
        let c = Confidence::new(0.5);
        let v: f32 = c.into();
        assert!((v - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_confidence_serialization_roundtrip() {
        let c = Confidence::new(0.75);
        let json = serde_json::to_string(&c).unwrap();
        let deserialized: Confidence = serde_json::from_str(&json).unwrap();
        assert_eq!(c, deserialized);
    }

    #[test]
    fn test_confidence_nan_becomes_zero() {
        let c = Confidence::new(f32::NAN);
        assert_eq!(c.raw(), 0.0);
    }

    #[test]
    fn test_confidence_infinity_becomes_one() {
        let c = Confidence::new(f32::INFINITY);
        assert_eq!(c.raw(), 1.0);
    }
}
