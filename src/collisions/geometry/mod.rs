///! The `geometry` module defines useful structs and functions for operating on
///! geometry primitives.

mod line;
mod face;
mod plane;
mod polyhedron;
mod convex_hull;
mod support_map;
mod line_projection;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod intersection;

pub mod _2d;
pub mod shapes;

pub use self::line::Line;
pub use self::face::Face;
pub use self::plane::Plane;
pub use self::polyhedron::Polyhedron;
pub use self::convex_hull::ConvexHull3D;
pub use self::support_map::SupportMap;
pub use self::intersection::Intersection;
pub use self::line_projection::LineProjection;
