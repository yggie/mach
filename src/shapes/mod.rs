//! The `shapes` module defines the shared traits for all geometric models.

mod cuboid;
mod sphere;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod shape;

pub use self::shape::Shape;
pub use self::sphere::Sphere;
pub use self::cuboid::Cuboid;
