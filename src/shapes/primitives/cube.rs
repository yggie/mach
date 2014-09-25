use math::approx_eq;
use shapes::Shape;

use std::fmt;

/// A representation of a cube in 3 dimensions.
#[deriving(Clone)]
pub struct Cube {
    /// The width of the cube
    pub width: f32,
    /// The height of the cube
    pub height: f32,
    /// The depth of the cube
    pub depth: f32,
}

impl Cube {

    /// Constructs a new Cube given the width, height and depth dimensions.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(5.0, 3.0, 7.5);
    ///
    /// assert_eq!((c.width, c.height, c.depth), (5.0, 3.0, 7.5))
    /// ```
    pub fn new(width: f32, height: f32, depth: f32) -> Cube {
        Cube{ width: width, height: height, depth: depth }
    }
}

impl fmt::Show for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cube{{ w={}, h={}, d={} }}", self.width, self.height, self.depth)
    }
}

/// Implements the `Eq` trait for the Cube to specify that equality is an
/// equivalence relation.
impl Eq for Cube {}

impl PartialEq for Cube {

    /// Implements the `==` operator for the Cube class. Compares the dimensions
    /// of the cube to be within reasonable tolerance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::Cube;
    /// let a = Cube::new(1.0, 2.0, 3.0);
    /// let b = Cube::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(a, b)
    /// ```
    fn eq(&self, other: &Cube) -> bool {
        approx_eq(self.width, other.width) &&
            approx_eq(self.height, other.height) &&
            approx_eq(self.depth, other.depth)
    }
}

impl Shape for Cube {

    /// Calculates the volume of the Cube.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::shapes::Shape;
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(c.volume(), 24.0)
    /// ```
    fn volume(&self) -> f32 {
        self.width * self.height * self.depth
    }
}
