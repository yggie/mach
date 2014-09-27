use math::approx_eq;
use shapes::Shape;

use std::fmt;

#[cfg(test)]
#[path="../../../tests/shapes/primitives/cube_test.rs"]
mod tests;

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
    fn eq(&self, other: &Cube) -> bool {
        approx_eq(self.width, other.width) &&
            approx_eq(self.height, other.height) &&
            approx_eq(self.depth, other.depth)
    }
}

impl Shape for Cube {

    /// Calculates the volume of the Cube.
    fn volume(&self) -> f32 {
        self.width * self.height * self.depth
    }
}
