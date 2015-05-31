//! The `utils` module contains various helper functions.

pub use self::surface::Surface;
pub use self::energy::*;
pub use self::compute_surfaces_for_convex_hull::compute_surfaces_for_convex_hull;

mod energy;

mod surface;
mod compute_surfaces_for_convex_hull;
