#[cfg(test)]
#[path="../../../../tests/collisions/shapes/convex_shapes/sphere_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../../../tests/support/collisions/shapes/convex_shapes/arbitrary_sphere.rs"]
mod arbitrary;

use std::fmt;

use {Scalar, PI, TOLERANCE};
use maths::{Matrix, Vec3D};
use collisions::shapes::{Direction, Shape};
use collisions::shapes::convex_shapes::{ConvexShape, ShapeRef};

/// A representation of a sphere in 3 dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    radius: Scalar,
}

impl Shape for Sphere {}

impl Sphere {
    /// Constructs a new `Sphere` with the radius provided.
    pub fn new(radius: Scalar) -> Sphere {
        Sphere {
            radius: radius,
        }
    }

    /// Returns the radius of the `Sphere`.
    pub fn radius(&self) -> Scalar {
        self.radius
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere{{{}}}", self.radius)
    }
}

impl ConvexShape for Sphere {
    fn downcast(&self) -> ShapeRef {
        ShapeRef::Sphere(self)
    }

    fn volume(&self) -> Scalar {
        PI * self.radius() * self.radius() * self.radius() * 4.0 / 3.0
    }

    fn inertia(&self) -> Matrix {
        let i = self.radius() * self.radius() * 2.0 / 5.0;
        Matrix::diag(i, i, i)
    }

    fn vertex(&self, index: usize) -> Vec3D {
        debug_assert_eq!(index, 0);
        Vec3D::zero()
    }

    fn vertices_len(&self) -> usize {
        1
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a> {
        Box::new(Defer::new(Vec3D::zero()))
    }

    fn support_indices_for(&self, _: Direction) -> Vec<usize> {
        vec!(0)
    }

    #[inline]
    fn surface_radius(&self) -> Scalar {
        self.radius() + TOLERANCE
    }

    fn box_clone(&self) -> Box<ConvexShape> {
        Box::new(self.clone())
    }
}

struct Defer<T> {
    _value: Option<T>,
}

impl<T: Copy> Defer<T> {
    fn new(value: T) -> Defer<T> {
        Defer { _value: Some(value) }
    }
}

impl<T: Copy> Iterator for Defer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self._value {
            Some(value) => {
                self._value = None;

                return Some(value);
            },

            None => return None,
        }
    }
}
