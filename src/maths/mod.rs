//! The `math` module contains all the logic associated with primitive
//! mathematical operations.

mod quat;
mod vect;
mod matrix;
mod motion;
mod approx_eq;
mod transform;

pub use self::quat::Quat;
pub use self::vect::Vect;
pub use self::matrix::Matrix;
pub use self::motion::Motion;
pub use self::approx_eq::ApproxEq;
pub use self::transform::Transform;
