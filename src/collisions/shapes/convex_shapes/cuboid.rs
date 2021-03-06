#[cfg(test)]
#[path="../../../../tests/collisions/shapes/convex_shapes/cuboid_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../../../tests/support/collisions/shapes/convex_shapes/arbitrary_cuboid.rs"]
mod arbitrary;

use std::fmt;

use {Scalar, TOLERANCE};
use maths::{ApproxEq, DotProduct, Matrix, Vec3D};
use collisions::shapes::{Direction, Shape};
use collisions::shapes::convex_shapes::{ConvexShape, ShapeRef};

/// A representation of a cuboid in 3 dimensions.
#[derive(Clone, Debug)]
pub struct Cuboid {
    dimensions: Vec3D,
    vertices: [Vec3D; 8],
}

impl Shape for Cuboid {}

impl Cuboid {
    /// Constructs a new `Cuboid` with the given dimensions.
    pub fn new(x_size: Scalar, y_size: Scalar, z_size: Scalar) -> Cuboid {
        let half_x = x_size / 2.0;
        let half_y = y_size / 2.0;
        let half_z = z_size / 2.0;

        Cuboid {
            dimensions: Vec3D::new(x_size, y_size, z_size),
            vertices: [
                // TODO refactor this to something else (z = height??)
                Vec3D::new( half_x,  half_y,  half_z),
                Vec3D::new(-half_x,  half_y,  half_z),
                Vec3D::new(-half_x, -half_y,  half_z),
                Vec3D::new( half_x, -half_y,  half_z),
                Vec3D::new( half_x,  half_y, -half_z),
                Vec3D::new(-half_x,  half_y, -half_z),
                Vec3D::new(-half_x, -half_y, -half_z),
                Vec3D::new( half_x, -half_y, -half_z),
            ],
        }
    }

    /// Constructs a new `Cuboid` with equally sized edges.
    pub fn cube(size: Scalar) -> Cuboid {
        Cuboid::new(size, size, size)
    }

    pub fn dimensions(&self) -> &Vec3D {
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

impl ConvexShape for Cuboid {
    fn downcast(&self) -> ShapeRef {
        ShapeRef::Cuboid(self)
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

    fn vertex(&self, index: usize) -> Vec3D {
        self.vertices[index]
    }

    fn vertices_len(&self) -> usize {
        8
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a> {
        Box::new(self.vertices.iter().map(|&a| a))
    }

    fn support_indices_for(&self, input_direction: Direction) -> Vec<usize> {
        let direction = Vec3D::from(input_direction);

        let new_direction = Vec3D::new(
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

    fn box_clone(&self) -> Box<ConvexShape> {
        Box::new(self.clone())
    }
}
