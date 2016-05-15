#[macro_use]
mod behaviours;
#[macro_use]
mod assert_approx_eq;
#[macro_use]
mod assert_approx_matching;

mod arbitrary_radians;
mod arbitrary_positive_scalar;

pub use self::arbitrary_radians::Radians;
pub use self::arbitrary_positive_scalar::PositiveScalar;

use Scalar;

pub static TEST_SCALAR_BOUNDS: Scalar = 1e2;
