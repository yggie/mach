use std::fmt;

use maths::{ approx_eq, Matrix, Vector, TOLERANCE };
use shapes::Shape;

/// A representation of a cube in 3 dimensions.
#[derive(Clone, Debug)]
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

        // TODO refactor this to something else (z = height??)
        let mut vertices = Vec::new();
        vertices.push(Vector::new( half_width,  half_height,  half_depth));
        vertices.push(Vector::new(-half_width,  half_height,  half_depth));
        vertices.push(Vector::new(-half_width, -half_height,  half_depth));
        vertices.push(Vector::new( half_width, -half_height,  half_depth));

        vertices.push(Vector::new( half_width,  half_height, -half_depth));
        vertices.push(Vector::new(-half_width,  half_height, -half_depth));
        vertices.push(Vector::new(-half_width, -half_height, -half_depth));
        vertices.push(Vector::new( half_width, -half_height, -half_depth));

        Cube {
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

    fn inertia(&self) -> Matrix {
        let w2 = self.width*self.width;
        let h2 = self.height*self.height;
        let d2 = self.depth*self.depth;

        return Matrix::new(
            (h2 + d2)/12.0,            0.0,            0.0,
                       0.0, (w2 + d2)/12.0,            0.0,
                       0.0,            0.0, (h2 + w2)/12.0,
        );
    }

    fn vertex(&self, index: usize) -> Vector {
        self.vertices[index]
    }

    fn vertices_len(&self) -> usize {
        8
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=&Vector> + 'a> {
        Box::new(self.vertices.iter())
    }

    fn support_indices_for(&self, direction: Vector) -> Vec<usize> {
        let new_direction = Vector::new(
            direction[0]/self.width,
            direction[1]/self.height,
            direction[2]/self.depth,
        );
        let mut max_value = 0.0;
        let mut max_indices = Vec::new();

        for (index, vertex) in self.vertices.iter().enumerate() {
            let value = vertex.dot(new_direction);

            let diff = value - max_value;
            if diff > TOLERANCE {
                max_value = value;
                max_indices = Vec::new();
                max_indices.push(index)
            } else if diff.abs() < TOLERANCE {
                max_indices.push(index)
            }
        }

        return max_indices;
    }
}
