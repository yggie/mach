///! The `geometry` module defines useful structs and functions for operating on
///! geometry primitives.

#[cfg(test)]
#[path="../../../tests/collisions/geometry/support_map_behaviour.rs"]
pub mod behaviour;

mod ray;
mod face;
mod plane;
mod point;
mod geometry;
mod direction;
mod support_map;
mod intersection;
mod line_projection;

pub mod _2d;
pub mod convex_shapes;

pub use self::ray::Ray;
pub use self::face::Face;
pub use self::plane::Plane;
pub use self::point::Point;
pub use self::geometry::Geometry;
pub use self::direction::Direction;
pub use self::support_map::SupportMap;
pub use self::intersection::Intersection;
pub use self::line_projection::LineProjection;
