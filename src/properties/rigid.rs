use math::approx_eq;
use shapes::Shape;
use properties::Property;

use std::fmt;

#[cfg(test)]
#[path="../../tests/properties/rigid_test.rs"]
mod tests;

/// Represents a rigid body with a fixed density.
#[deriving(Clone)]
pub struct Rigid {
    /// The density in M/L^3 units.
    pub density: f32,
}

impl Rigid {

    /// Creates a new rigid body property object with the given density.
    pub fn new(density: f32) -> Rigid {
        Rigid{ density: density }
    }
}

impl fmt::Show for Rigid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rigid{{ d={} }}", self.density)
    }
}

impl Eq for Rigid { }

impl PartialEq for Rigid {

    /// Implements the equality operator for the Rigid property. Compares the
    /// density between the two properties with reasonable tolerance.
    fn eq(&self, other: &Rigid) -> bool {
        approx_eq(self.density, other.density)
    }
}

/// Implements the Property trait
impl Property for Rigid {
    /// Computes the mass using the volume of the provided shape.
    fn mass_of(&self, shape: &Shape) -> f32 {
        self.density * shape.volume()
    }

    /// Simple returns the pre-defined density.
    fn density_of(&self, _: &Shape) -> f32 {
        self.density
    }
}
