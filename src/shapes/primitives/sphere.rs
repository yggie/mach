use math::{ TOLERANCE, approx_eq };
use shapes::Shape;

use std::fmt;
use std::f32::consts::PI;

#[cfg(test)]
#[path="../../../tests/shapes/primitives/sphere_test.rs"]
mod tests;

/// A representation of a sphere in 3 dimensions.
#[derive(Clone, Copy, Show)]
pub struct Sphere {
    /// The radius of the sphere.
    pub radius: f32,
}

impl Sphere {
    /// Constructs a new Sphere with the radius provided.
    pub fn new(radius: f32) -> Sphere {
        Sphere{ radius: radius }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle{{ r={} }}", self.radius)
    }
}

/// Implements the `Eq` trait for the Sphere to specify that equality is an
/// equivalence relation.
impl Eq for Sphere { }

/// Implements the `==` operator for the Sphere class.
impl PartialEq for Sphere {
    /// Implements the equality operator for the Sphere class. Compares the
    /// radius of each sphere to determine equality.
    fn eq(&self, other: &Sphere) -> bool {
        approx_eq(self.radius, other.radius)
    }
}

impl Shape for Sphere {
    /// Calculates the volume of the Sphere.
    fn volume(&self) -> f32 {
        4.0*PI*self.radius*self.radius*self.radius/3.0
    }

    /// Returns the _surface radius_ of the Shape. The surface radius is the
    /// tolerance used to determine if a collision has occurred, it is useful to
    /// avoid problems with singularities such as edge-edge collisions.
    /// Overrides the default implementation to return the Sphere’s radius and
    /// the default tolerance.
    #[inline]
    fn surface_radius(&self) -> f32 {
        self.radius + TOLERANCE
    }
}
