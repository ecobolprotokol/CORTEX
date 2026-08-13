use serde::{Deserialize, Serialize};

pub type Scalar = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

pub const SCALAR_EPSILON: Scalar = 1e-6;

pub fn scalar_eq(a: Scalar, b: Scalar) -> bool {
    (a - b).abs() < SCALAR_EPSILON
}
