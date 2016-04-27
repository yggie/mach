///! The `geometry` module defines useful structs and functions for operating on
///! geometry primitives.
mod line;
mod plane;
mod line_projection;
mod plane_normal_projection;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod intersection;

pub mod _2d;

pub use self::line::Line;
pub use self::plane::Plane;
pub use self::intersection::Intersection;
pub use self::line_projection::LineProjection;
pub use self::plane_normal_projection::PlaneNormalProjection;
