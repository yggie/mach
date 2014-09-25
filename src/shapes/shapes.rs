//! The shapes module defines the shared traits for all geometric models.
use std::fmt::Show;

pub use self::primitives::{ Sphere, Cube };

/// Defines the traits for all geometric property descriptions.
pub trait Shape: Clone + Eq + Show {
    /// Computes the volume for the shape.
    fn volume(&self) -> f32;
}

mod primitives {
    pub use self::sphere::Sphere;
    pub use self::cube::Cube;

    mod sphere;
    mod cube;
}
