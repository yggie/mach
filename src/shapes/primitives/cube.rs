use math::{ approx_eq, Vector };
use shapes::Shape;

use std::fmt;

#[cfg(test)]
#[path="../../../tests/shapes/primitives/cube_test.rs"]
mod tests;

/// A representation of a cube in 3 dimensions.
#[derive(Clone, Show)]
pub struct Cube {
    width: f32,
    height: f32,
    depth: f32,
    vertices: Vec<Vector>,
}

impl Cube {
    /// Constructs a new Cube given the width, height and depth dimensions.
    pub fn new(width: f32, height: f32, depth: f32) -> Cube {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let half_depth = depth / 2.0;

        let mut vertices = Vec::new();
        vertices.push(Vector::new( half_width,  half_height,  half_depth));
        vertices.push(Vector::new(-half_width,  half_height,  half_depth));
        vertices.push(Vector::new(-half_width, -half_height,  half_depth));
        vertices.push(Vector::new( half_width, -half_height,  half_depth));

        vertices.push(Vector::new( half_width,  half_height, -half_depth));
        vertices.push(Vector::new(-half_width,  half_height, -half_depth));
        vertices.push(Vector::new(-half_width, -half_height, -half_depth));
        vertices.push(Vector::new( half_width, -half_height, -half_depth));

        Cube{
            width: width,
            height: height,
            depth: depth,
            vertices: vertices,
        }
    }

    /// Returns the width of the `Cube`.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Returns the height of the `Cube`.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Returns the depth of the `Cube`.
    pub fn depth(&self) -> f32 {
        self.depth
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cube{{ w={}, h={}, d={} }}", self.width, self.height, self.depth)
    }
}

/// Implements the `Eq` trait for the Cube to specify that equality is an
/// equivalence relation.
impl Eq for Cube {}

impl PartialEq for Cube {
    /// Implements the `==` operator for the `Cube` class. Compares the
    /// dimensions of the `Cube` to be within reasonable tolerance.
    fn eq(&self, other: &Cube) -> bool {
        approx_eq(self.width, other.width) &&
            approx_eq(self.height, other.height) &&
            approx_eq(self.depth, other.depth)
    }
}

impl Shape for Cube {
    /// Calculates the volume of the `Cube`.
    fn volume(&self) -> f32 {
        self.width * self.height * self.depth
    }

    fn vertex(&self, index: usize) -> Vector {
        self.vertices[index]
    }

    fn vertices_len(&self) -> usize {
        8
    }

    fn vertices_iter(&self) -> Box<Iterator<Item=&Vector>> {
        Box::new(self.vertices.iter())
    }

    fn farthest_index_in_direction(&self, direction: Vector) -> usize {
        let new_direction = Vector::new(
            direction[0]/self.width,
            direction[1]/self.height,
            direction[2]/self.depth,
        );
        let mut max_value = 0.0;
        let mut max_index = 0us;

        for (index, vertex) in self.vertices.iter().enumerate() {
            let value = vertex.dot(new_direction);
            if value > max_value {
                max_value = value;
                max_index = index;
            }
        }

        return max_index;
    }
}
