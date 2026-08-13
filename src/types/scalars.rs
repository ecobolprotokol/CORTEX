pub type Scalar = f32;

pub const SCALAR_EPSILON: Scalar = 1e-6;

pub fn scalar_eq(a: Scalar, b: Scalar) -> bool {
    (a - b).abs() < SCALAR_EPSILON
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Precision {
    F32,
    F16,
    BF16,
}

impl Default for Precision {
    fn default() -> Self {
        Precision::F32
    }
}

impl Precision {
    pub fn bits(&self) -> u32 {
        match self {
            Precision::F32 => 32,
            Precision::F16 => 16,
            Precision::BF16 => 16,
        }
    }
}

pub trait ScalarExt {
    fn is_valid_cognitive_value(self) -> bool;
    fn validate_range(self, min: Scalar, max: Scalar) -> Result<(), String>;
    fn clamp_valid(self) -> Self;
}

impl ScalarExt for Scalar {
    fn is_valid_cognitive_value(self) -> bool {
        self.is_finite()
    }

    fn validate_range(self, min: Scalar, max: Scalar) -> Result<(), String> {
        if !self.is_finite() {
            return Err("Non-finite value".into());
        }
        if self < min || self > max {
            return Err(format!("Value {} out of range [{}, {}]", self, min, max));
        }
        Ok(())
    }

    fn clamp_valid(self) -> Self {
        if self.is_finite() { self } else { 0.0 }
    }
}
