//! The `shapes` module defines the shared traits for all geometric models.

mod cuboid;
mod sphere;
mod shape_ref;
mod intersections;
mod triangle_mesh;
mod convex_polyhedron;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod convex_shape;

pub use self::convex_shape::ConvexShape;
pub use self::sphere::Sphere;
pub use self::cuboid::Cuboid;
pub use self::shape_ref::ShapeRef;
pub use self::triangle_mesh::TriangleMesh;
pub use self::convex_polyhedron::{ConvexPolyhedron, ConvexPolyhedronError};
