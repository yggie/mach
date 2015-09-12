//! The `utils` module contains various helper functions.

mod energy;
mod surface;
mod compute_surfaces_for_convex_hull;

pub mod debug;

pub use self::surface::Surface;
pub use self::energy::*;
pub use self::compute_surfaces_for_convex_hull::compute_surfaces_for_convex_hull;

use core::{ Float, TOLERANCE };

/// Determines if the difference between two floating point numbers are within
/// reasonable tolerance. The tolerance is set to `1e-6`.
pub fn approx_eq(a: Float, b: Float) -> bool {
    (a - b).abs() < TOLERANCE
}
