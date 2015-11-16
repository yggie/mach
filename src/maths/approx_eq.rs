use { Float, TOLERANCE };

/// This trait is implemented by types without infinite precision.
pub trait ApproxEq<T = Self> {
    /// Returns true if the other value is approximately equal.
    fn approx_eq(self, T) -> bool;
}

impl<'a> ApproxEq<&'a Float> for &'a Float {
    fn approx_eq(self, other: &'a Float) -> bool {
        (self - other).abs() < TOLERANCE
    }
}

impl ApproxEq for Float {
    fn approx_eq(self, other: Self) -> bool {
        (&self).approx_eq(&other)
    }
}
