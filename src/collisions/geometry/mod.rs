///! The `geometry` module defines useful structs and functions for operating on
///! geometry primitives.

#[cfg(test)]
#[path="../../../tests/collisions/geometry/support_map_behaviour.rs"]
pub mod behaviour;

mod line;
mod face;
mod plane;
mod support_map;
mod line_projection;
mod convex_polyhedron;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod intersection;

pub mod _2d;
pub mod shapes;

pub use self::line::Line;
pub use self::face::Face;
pub use self::plane::Plane;
pub use self::support_map::SupportMap;
pub use self::intersection::Intersection;
pub use self::line_projection::LineProjection;
pub use self::convex_polyhedron::{ConvexPolyhedron, ConvexPolyhedronError};
