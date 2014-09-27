//! The math module contains all the logic associated with primitive mathematical
//! operations.

pub use self::vector::Vector;
pub use self::matrix::Matrix;
pub use self::transform::Transform;

#[stable]
mod vector;
#[stable]
mod matrix;
mod transform;

#[cfg(test)]
#[path="../../tests/math/math_test.rs"]
mod tests;

/// Determines if the difference between two floating point numbers are within
/// reasonable tolerance. The tolerance is set to `1e-6`.
pub fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-6
}
