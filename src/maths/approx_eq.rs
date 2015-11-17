use { Scalar, TOLERANCE };

/// This trait is implemented by types without infinite precision.
pub trait ApproxEq<T = Self> {
    /// Returns true if the other value is approximately equal.
    fn approx_eq(self, T) -> bool;
}

impl ApproxEq for Scalar {
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < TOLERANCE
    }
}
