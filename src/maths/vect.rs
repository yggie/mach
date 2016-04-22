#[cfg(test)]
#[path="../../tests/maths/vect_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../tests/support/maths/arbitrary_vect.rs"]
mod arbitrary;

use std::fmt;
use std::mem;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};

use {Scalar, TOLERANCE};
use maths::{ApproxEq, Matrix};

/// A representation of a 3-dimensional column vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vect {
    /// The x component of the vector.
    pub x: Scalar,
    /// The y component of the vector.
    pub y: Scalar,
    /// The z component of the vector.
    pub z: Scalar,
}

/// Static methods for the Vect struct.
impl Vect {
    /// A simple constructor which builds a column vector given three elements.
    #[inline(always)]
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vect {
        Vect {
            x: x,
            y: y,
            z: z,
        }
    }

    /// Constructs a zero vector.
    pub fn zero() -> Vect {
        Vect::new(0.0, 0.0, 0.0)
    }

    /// Set the components of the `Vect` to the specified values.
    #[inline]
    pub fn set(&mut self, other: &(Scalar, Scalar, Scalar)) {
        self.x = other.0;
        self.y = other.1;
        self.z = other.2;
    }

    /// Computes the dot product between two vectors.
    #[inline(always)]
    pub fn dot(&self, other: Vect) -> Scalar {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    /// Computes the cross product between two vectors.
    #[inline]
    pub fn cross(&self, other: Vect) -> Vect {
        Vect::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }

    /// Computes the direction vector of a Vect.
    #[inline]
    pub fn normalize(&self) -> Vect {
        *self / self.length()
    }

    /// Computes the squared length of a Vect.
    #[inline(always)]
    pub fn length_sq(&self) -> Scalar {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Computes the length of a Vect.
    #[inline(always)]
    pub fn length(&self) -> Scalar {
        self.length_sq().sqrt()
    }

    /// Computes the outer product between two Vectors.
    pub fn outer(&self, other: Vect) -> Matrix {
        return Matrix::new(
            self.x*other.x, self.x*other.y, self.x*other.z,
            self.y*other.x, self.y*other.y, self.y*other.z,
            self.z*other.x, self.z*other.y, self.z*other.z,
        );
    }

    /// Computes the distance to the `Vect` specified.
    pub fn distance_to(&self, other: Vect) -> Scalar {
        (*self - other).length()
    }
}

/// Implements the `Display` trait to allow using `println!` on Vectors. The
/// resulting format is equivalent to:
///
/// ```rust
/// extern crate mach;
///
/// let vec = mach::Vect::new(0.1, 0.2, 0.3);
/// println!("[{}, {}, {}]", vec.x, vec.y, vec.z);
/// ```
impl fmt::Display for Vect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vect`s.
impl<'a> Neg for &'a Vect {
    type Output = Vect;

    #[inline]
    fn neg(self) -> Vect {
        Vect::new(-self.x, -self.y, -self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vect`s.
impl Neg for Vect {
    type Output = Vect;

    #[inline]
    fn neg(self) -> Vect {
        Vect::new(-self.x, -self.y, -self.z)
    }
}

/// Implements the `ApproxEq` trait to approximate the equality of two
/// `Vect`s. The implementation uses the Euclidean distance between the two
/// `Vect`s to perform the comparison.
impl<'a> ApproxEq<&'a Vect> for &'a Vect {
    fn approx_eq(self, other: &'a Vect) -> bool {
        (self - other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Reuses the implementation `ApproxEq<&Vect> for &Vect`.
impl<'a> ApproxEq<Vect> for &'a Vect {
    fn approx_eq(self, other: Vect) -> bool {
        self.approx_eq(&other)
    }
}

/// Reuses the implementation `ApproxEq<&Vect> for &Vect`.
impl<'a> ApproxEq<&'a Vect> for Vect {
    fn approx_eq(self, other: &'a Vect) -> bool {
        (&self).approx_eq(other)
    }
}

/// Reuses the implementation `ApproxEq<&Vect> for &Vect`.
impl ApproxEq<Vect> for Vect {
    fn approx_eq(self, other: Vect) -> bool {
        (&self).approx_eq(&other)
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vect`s.
impl<'a, 'b> Add<&'a Vect> for &'b Vect {
    type Output = Vect;

    #[inline]
    fn add(self, other: &'a Vect) -> Vect {
        Vect::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
implement_op_overload_variants!(Add, add, Vect, Vect, Vect);

/// Implement the `Sub` trait to allow using the `-` operator between `Vect`s.
impl<'a, 'b> Sub<&'b Vect> for &'a Vect {
    type Output = Vect;

    #[inline]
    fn sub(self, other: &'b Vect) -> Self::Output {
        Vect::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
implement_op_overload_variants!(Sub, sub, Vect, Vect, Vect);

/// Implement the `Mul` trait to allow using the `*` operator for a `Vect`
/// with a `Scalar`.
impl<'a> Mul<Scalar> for &'a Vect {
    type Output = Vect;

    #[inline]
    fn mul(self, s: Scalar) -> Vect {
        Vect::new(self.x*s, self.y*s, self.z*s)
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Vect`
/// with a `Scalar`.
impl Mul<Scalar> for Vect {
    type Output = Vect;

    #[inline]
    fn mul(self, s: Scalar) -> Vect {
        &self * s
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vect`.
impl Mul<Vect> for Scalar {
    type Output = Vect;

    #[inline]
    fn mul(self, vector: Vect) -> Vect {
        &vector * self
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vect`.
impl<'a> Mul<&'a Vect> for Scalar {
    type Output = Vect;

    #[inline]
    fn mul(self, vector: &'a Vect) -> Vect {
        vector * self
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vect`
/// with a `Scalar`.
impl<'a> Div<Scalar> for &'a Vect {
    type Output = Vect;

    #[inline]
    fn div(self, s: Scalar) -> Vect {
        Vect::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vect`
/// with a `Scalar`.
impl Div<Scalar> for Vect {
    type Output = Vect;

    #[inline]
    fn div(self, s: Scalar) -> Vect {
        Vect::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vect` and a
/// `[Scalar; 3]`.
impl AsRef<[Scalar; 3]> for Vect {
    #[inline]
    fn as_ref(&self) -> &[Scalar; 3] {
        unsafe { mem::transmute(self) }
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vect` and a
/// `(Scalar, Scalar, Scalar)`.
impl AsRef<(Scalar, Scalar, Scalar)> for Vect {
    #[inline]
    fn as_ref(&self) -> &(Scalar, Scalar, Scalar) {
        unsafe { mem::transmute(self) }
    }
}

impl Deref for Vect {
    type Target = (Scalar, Scalar, Scalar);

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl DerefMut for Vect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}
