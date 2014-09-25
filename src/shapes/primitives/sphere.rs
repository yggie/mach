use math::approx_eq;
use shapes::Shape;

use std::fmt;
use std::f32::consts::PI;

/// A representation of a sphere in 3 dimensions.
#[deriving(Clone)]
pub struct Sphere {
    /// The radius of the sphere.
    pub radius: f32,
}

impl Sphere {

    /// Constructs a new Sphere with the radius provided.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::Sphere;
    /// let s = Sphere::new(5.0);
    ///
    /// assert_eq!(s.radius, 5.0)
    /// ```
    pub fn new(radius: f32) -> Sphere {
        Sphere{ radius: radius }
    }
}

impl fmt::Show for Sphere {
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
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::Sphere;
    /// let a = Sphere::new(8.8);
    /// let b = Sphere::new(8.8);
    ///
    /// assert_eq!(a, b)
    /// ```
    fn eq(&self, other: &Sphere) -> bool {
        approx_eq(self.radius, other.radius)
    }
}

impl Shape for Sphere {

    /// Calculates the volume of the Sphere.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::{ Shape, Sphere };
    /// # use mithril::math::approx_eq;
    /// # use std::f32::consts::PI;
    /// let s = Sphere::new(0.75);
    ///
    /// assert!(approx_eq(s.volume(), 0.5625*PI));
    /// ```
    fn volume(&self) -> f32 {
        4.0*PI*self.radius*self.radius*self.radius/3.0
    }
}
