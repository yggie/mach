mod simplex;
mod minkowski_difference;

#[cfg(test)]
#[path="../../../tests/private/collisions/gjk/minkowski_difference_test.rs"]
mod minkowski_difference_test;

pub use self::simplex::{Simplex, SimplexContainingOrigin};
pub use self::minkowski_difference::{MinkowskiDifference, SupportPoint};
