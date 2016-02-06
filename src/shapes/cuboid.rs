#[cfg(test)]
#[path="../../tests/shapes/cuboid_test.rs"]
mod cuboid_test;

use std::fmt;

use {Scalar, TOLERANCE};
use maths::{ApproxEq, Matrix, Vect};
use shapes::{Shape, ShapeSpec};

/// A representation of a cuboid in 3 dimensions.
#[derive(Clone, Debug)]
pub struct Cuboid {
    dimensions: Vect,
    vertices: [Vect; 8],
}

impl Cuboid {
    /// Constructs a new `Cuboid` with the given dimensions.
    pub fn new(x_size: Scalar, y_size: Scalar, z_size: Scalar) -> Cuboid {
        let half_x = x_size / 2.0;
        let half_y = y_size / 2.0;
        let half_z = z_size / 2.0;

        Cuboid {
            dimensions: Vect::new(x_size, y_size, z_size),
            vertices: [
                // TODO refactor this to something else (z = height??)
                Vect::new( half_x,  half_y,  half_z),
                Vect::new(-half_x,  half_y,  half_z),
                Vect::new(-half_x, -half_y,  half_z),
                Vect::new( half_x, -half_y,  half_z),
                Vect::new( half_x,  half_y, -half_z),
                Vect::new(-half_x,  half_y, -half_z),
                Vect::new(-half_x, -half_y, -half_z),
                Vect::new( half_x, -half_y, -half_z),
            ],
        }
    }

    /// Constructs a new `Cuboid` with equally sized edges.
    pub fn cube(size: Scalar) -> Cuboid {
        Cuboid::new(size, size, size)
    }

    pub fn dimensions(&self) -> &Vect {
        &self.dimensions
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cuboid({}, {}, {})", self.dimensions.x, self.dimensions.y, self.dimensions.z)
    }
}

/// Implements the `Eq` trait for the Cuboid to specify that equality is an
/// equivalence relation.
impl Eq for Cuboid {}

impl PartialEq for Cuboid {
    /// Implements the `==` operator for the `Cuboid` class. Compares the
    /// dimensions of the `Cuboid` to be within reasonable tolerance.
    fn eq(&self, other: &Cuboid) -> bool {
        ApproxEq::approx_eq(self.dimensions, other.dimensions)
    }
}

impl Shape for Cuboid {
    fn spec(&self) -> ShapeSpec {
        ShapeSpec::Cuboid(
            self.dimensions.x,
            self.dimensions.y,
            self.dimensions.z,
        )
    }

    /// Calculates the volume of the `Cuboid`.
    fn volume(&self) -> Scalar {
        self.dimensions.x * self.dimensions.y * self.dimensions.z
    }

    fn inertia(&self) -> Matrix {
        let xx = self.dimensions.x*self.dimensions.x;
        let yy = self.dimensions.y*self.dimensions.y;
        let zz = self.dimensions.z*self.dimensions.z;

        return Matrix::new(
            (yy + zz)/12.0,            0.0,            0.0,
                       0.0, (xx + zz)/12.0,            0.0,
                       0.0,            0.0, (xx + yy)/12.0,
        );
    }

    fn vertex(&self, index: usize) -> Vect {
        self.vertices[index]
    }

    fn vertices_len(&self) -> usize {
        8
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
        Box::new(self.vertices.iter().map(|&a| a))
    }

    fn support_indices_for(&self, direction: Vect) -> Vec<usize> {
        let new_direction = Vect::new(
            direction.x/self.dimensions.x,
            direction.y/self.dimensions.y,
            direction.z/self.dimensions.z,
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
