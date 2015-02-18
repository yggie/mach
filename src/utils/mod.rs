//! The `utils` module contains various helper functions.

#![unstable]

pub use self::surface::Surface;
pub use self::compute_surfaces_for_convex_hull::compute_surfaces_for_convex_hull;

mod surface;
mod compute_surfaces_for_convex_hull;
