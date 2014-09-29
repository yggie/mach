//! The shapes module defines the shared traits for all geometric models.
use math;

use std::fmt::Show;

pub use self::primitives::{ Sphere, Cube };

/// Defines the traits for all geometric property descriptions.
pub trait Shape: Clone + Eq + Show {
    /// Computes the volume for the shape.
    fn volume(&self) -> f32;
    /// Returns the _surface radius_ of the Shape. The surface radius is the
    /// tolerance used to determine if a collision has occurred, it is useful to
    /// avoid problems with singularities such as edge-edge collisions. By
    /// default it returns the Engine’s tolerance, _i.e._
    /// `mithril::math::TOLERANCE`.
    #[inline]
    fn surface_radius(&self) -> f32 {
        return math::TOLERANCE;
    }
}

mod primitives {
    pub use self::sphere::Sphere;
    pub use self::cube::Cube;

    mod sphere;
    mod cube;
}
