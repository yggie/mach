use math::approx_eq;
use shapes::Shape;
use properties::Property;

use std::fmt;

/// Represents a rigid body with a fixed density
#[deriving(Clone)]
pub struct Rigid {
    pub density: f32,
}

impl Rigid {

    /// Creates a new rigid body property object with the given density.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::properties::Rigid;
    /// let r = Rigid::new(1.5);
    ///
    /// assert_eq!(r.density, 1.5)
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::properties::Rigid;
    /// let a = Rigid::new(1.0);
    /// let b = Rigid::new(1.0);
    ///
    /// assert_eq!(a, b)
    /// ```
    fn eq(&self, other: &Rigid) -> bool {
        approx_eq(self.density, other.density)
    }
}

/// Implements the Property trait
impl Property for Rigid {
    /// Computes the mass using the volume of the provided shape.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::properties::{ Rigid, Property };
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(1.0, 2.0, 3.0);
    /// let p = Rigid::new(2.0);
    ///
    /// assert_eq!(p.mass_of(&c), 12.0)
    /// ```
    fn mass_of(&self, shape: &Shape) -> f32 {
        self.density * shape.volume()
    }

    /// Simple returns the pre-defined density.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::properties::{ Rigid, Property };
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(1.0, 2.0, 3.0);
    /// let p = Rigid::new(1.5);
    ///
    /// assert_eq!(p.density_of(&c), 1.5)
    /// ```
    fn density_of(&self, _: &Shape) -> f32 {
        self.density
    }
}
