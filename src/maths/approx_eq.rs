use { Float, TOLERANCE };
use maths::Quaternion;

/// This trait is implemented by types without infinite precision.
pub trait ApproxEq {
    /// Returns true if the other value is approximately equal.
    fn approx_eq(&self, &Self) -> bool;
}

impl ApproxEq for Float {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < TOLERANCE
    }
}

impl ApproxEq for Quaternion {
    fn approx_eq(&self, other: &Self) -> bool {
        (*self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}
