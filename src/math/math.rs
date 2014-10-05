//! The math module contains all the logic associated with primitive mathematical
//! operations.

pub use self::vector::Vector;
pub use self::matrix::Matrix;
pub use self::transform::Transform;

mod vector;
mod matrix;
mod transform;

#[cfg(test)]
#[path="../../tests/unit/math/math_test.rs"]
mod tests;

/// The default tolerance used to resolve floating point differences.
pub static TOLERANCE: f32 = 1e-6;

/// Determines if the difference between two floating point numbers are within
/// reasonable tolerance. The tolerance is set to `1e-6`.
pub fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < TOLERANCE
}
