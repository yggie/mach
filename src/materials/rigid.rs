use math::{ approx_eq, Matrix };
use shapes::Shape;
use materials::Material;

use std::fmt;

/// Represents a rigid body with a fixed density.
#[derive(Clone, Copy, Debug)]
pub struct Rigid {
    /// The density in M/L^3 units.
    pub density: f32,
}

impl Rigid {
    /// Creates a new rigid material object with the given density.
    pub fn new(density: f32) -> Rigid {
        Rigid{ density: density }
    }
}

impl fmt::Display for Rigid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rigid{{ d={} }}", self.density)
    }
}

impl Eq for Rigid { }

impl PartialEq for Rigid {
    /// Implements the equality operator for the `Rigid` property. Compares the
    /// density between the two properties with reasonable tolerance.
    fn eq(&self, other: &Rigid) -> bool {
        approx_eq(self.density, other.density)
    }
}

/// Implements the `Material` trait
impl Material for Rigid {
    fn mass_of(&self, shape: &Shape) -> f32 {
        self.density * shape.volume()
    }

    fn inertia_for(&self, shape: &Shape) -> Matrix {
        shape.inertia_tensor() * self.mass_of(shape)
    }

    /// Returns the pre-defined density.
    fn density_of(&self, _: &Shape) -> f32 {
        self.density
    }
}
