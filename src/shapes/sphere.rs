use std::fmt;

use core::{ Float, PI, TOLERANCE };
use maths::{ Matrix, Vector };
use shapes::Shape;

/// A representation of a sphere in 3 dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    _radius: Float,
}

impl Sphere {
    /// Constructs a new `Sphere` with the radius provided.
    pub fn new(radius: Float) -> Sphere {
        Sphere {
            _radius: radius,
        }
    }

    /// Returns the radius of the `Sphere`.
    pub fn radius(&self) -> Float {
        self._radius
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere{{{}}}", self._radius)
    }
}

impl Shape for Sphere {
    fn volume(&self) -> Float {
        PI * self.radius() * self.radius() * self.radius() * 4.0 / 3.0
    }

    fn inertia(&self) -> Matrix {
        let i = self.radius() * self.radius() * 2.0 / 5.0;
        Matrix::new_diag(i, i, i)
    }

    fn vertex(&self, index: usize) -> Vector {
        debug_assert_eq!(index, 0);
        Vector::new_zero()
    }

    fn vertices_len(&self) -> usize {
        1
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vector> + 'a> {
        Box::new(Defer::new(Vector::new_zero()))
    }

    fn support_indices_for(&self, _: Vector) -> Vec<usize> {
        vec!(0)
    }

    #[inline]
    fn surface_radius(&self) -> Float {
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
