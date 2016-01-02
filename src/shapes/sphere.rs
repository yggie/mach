use std::fmt;

use {Scalar, PI, TOLERANCE};
use maths::{Matrix, Vect};
use shapes::{Shape, ShapeSpec};

/// A representation of a sphere in 3 dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    radius: Scalar,
}

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

impl Shape for Sphere {
    fn spec(&self) -> ShapeSpec {
        ShapeSpec::Sphere(self.radius)
    }

    fn volume(&self) -> Scalar {
        PI * self.radius() * self.radius() * self.radius() * 4.0 / 3.0
    }

    fn inertia(&self) -> Matrix {
        let i = self.radius() * self.radius() * 2.0 / 5.0;
        Matrix::diag(i, i, i)
    }

    fn vertex(&self, index: usize) -> Vect {
        debug_assert_eq!(index, 0);
        Vect::zero()
    }

    fn vertices_len(&self) -> usize {
        1
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
        Box::new(Defer::new(Vect::zero()))
    }

    fn support_indices_for(&self, _: Vect) -> Vec<usize> {
        vec!(0)
    }

    #[inline]
    fn surface_radius(&self) -> Scalar {
        self.radius() + TOLERANCE
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
