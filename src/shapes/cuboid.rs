use std::fmt;

use { Scalar, TOLERANCE };
use maths::{ ApproxEq, Matrix, Vector };
use shapes::Shape;

/// A representation of a cuboid in 3 dimensions.
#[derive(Clone, Debug)]
pub struct Cuboid {
    width: Scalar,
    height: Scalar,
    depth: Scalar,
    vertices: [Vector; 8],
}

impl Cuboid {
    /// Constructs a new `Cuboid` given the width, height and depth dimensions.
    pub fn new(width: Scalar, height: Scalar, depth: Scalar) -> Cuboid {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let half_depth = depth / 2.0;

        Cuboid {
            width: width,
            height: height,
            depth: depth,
            vertices: [
                // TODO refactor this to something else (z = height??)
                Vector::new( half_width,  half_height,  half_depth),
                Vector::new(-half_width,  half_height,  half_depth),
                Vector::new(-half_width, -half_height,  half_depth),
                Vector::new( half_width, -half_height,  half_depth),
                Vector::new( half_width,  half_height, -half_depth),
                Vector::new(-half_width,  half_height, -half_depth),
                Vector::new(-half_width, -half_height, -half_depth),
                Vector::new( half_width, -half_height, -half_depth),
            ],
        }
    }

    /// Constructs a new `Cuboid` with equally sized edges.
    pub fn new_cube(size: Scalar) -> Cuboid {
        Cuboid::new(size, size, size)
    }

    /// Returns the width of the `Cuboid`.
    pub fn width(&self) -> Scalar {
        self.width
    }

    /// Returns the height of the `Cuboid`.
    pub fn height(&self) -> Scalar {
        self.height
    }

    /// Returns the depth of the `Cuboid`.
    pub fn depth(&self) -> Scalar {
        self.depth
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cuboid{{ w={}, h={}, d={} }}", self.width, self.height, self.depth)
    }
}

/// Implements the `Eq` trait for the Cuboid to specify that equality is an
/// equivalence relation.
impl Eq for Cuboid {}

impl PartialEq for Cuboid {
    /// Implements the `==` operator for the `Cuboid` class. Compares the
    /// dimensions of the `Cuboid` to be within reasonable tolerance.
    fn eq(&self, other: &Cuboid) -> bool {
        ApproxEq::approx_eq(self.width, other.width) &&
            ApproxEq::approx_eq(self.height, other.height) &&
            ApproxEq::approx_eq(self.depth, other.depth)
    }
}

impl Shape for Cuboid {
    /// Calculates the volume of the `Cuboid`.
    fn volume(&self) -> Scalar {
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

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vector> + 'a> {
        Box::new(self.vertices.iter().map(|&a| a))
    }

    fn support_indices_for(&self, direction: Vector) -> Vec<usize> {
        let new_direction = Vector::new(
            direction.x/self.width,
            direction.y/self.height,
            direction.z/self.depth,
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
