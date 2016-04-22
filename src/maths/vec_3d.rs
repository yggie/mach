#[cfg(test)]
#[path="../../tests/maths/vec_3d_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../tests/support/maths/arbitrary_vec_3d.rs"]
mod arbitrary;

use std::fmt;
use std::mem;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};

use {Scalar, TOLERANCE};
use maths::{ApproxEq, CrossProduct, DotProduct, Matrix, UnitVec3D};

/// A representation of a 3-dimensional column vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3D {
    /// The x component of the vector.
    pub x: Scalar,
    /// The y component of the vector.
    pub y: Scalar,
    /// The z component of the vector.
    pub z: Scalar,
}

/// Static methods for the Vec3D struct.
impl Vec3D {
    /// A simple constructor which builds a column vector given three elements.
    #[inline(always)]
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vec3D {
        Vec3D {
            x: x,
            y: y,
            z: z,
        }
    }

    /// Constructs a zero vector.
    pub fn zero() -> Vec3D {
        Vec3D::new(0.0, 0.0, 0.0)
    }

    /// Set the components of the `Vec3D` to the specified values.
    #[inline]
    pub fn set(&mut self, other: &(Scalar, Scalar, Scalar)) {
        self.x = other.0;
        self.y = other.1;
        self.z = other.2;
    }

    /// Computes the direction vector of a Vec3D.
    #[inline]
    pub fn normalize(&self) -> UnitVec3D {
        UnitVec3D::from(self.clone())
    }

    /// Computes the squared length of a Vec3D.
    #[inline(always)]
    pub fn squared_length(&self) -> Scalar {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Computes the length of a Vec3D.
    #[inline(always)]
    pub fn length(&self) -> Scalar {
        self.squared_length().sqrt()
    }

    /// Computes the outer product between two Vectors.
    pub fn outer(&self, other: Vec3D) -> Matrix {
        return Matrix::new(
            self.x*other.x, self.x*other.y, self.x*other.z,
            self.y*other.x, self.y*other.y, self.y*other.z,
            self.z*other.x, self.z*other.y, self.z*other.z,
        );
    }

    /// Computes the distance to the `Vec3D` specified.
    pub fn distance_to(&self, other: Vec3D) -> Scalar {
        (*self - other).length()
    }
}

/// Implements the `Display` trait to allow using `println!` on Vectors. The
/// resulting format is equivalent to:
///
/// ```rust
/// extern crate mach;
///
/// let vec = mach::Vec3D::new(0.1, 0.2, 0.3);
/// println!("[{}, {}, {}]", vec.x, vec.y, vec.z);
/// ```
impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vec3D`s.
impl<'a> Neg for &'a Vec3D {
    type Output = Vec3D;

    #[inline]
    fn neg(self) -> Vec3D {
        Vec3D::new(-self.x, -self.y, -self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vec3D`s.
impl Neg for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn neg(self) -> Vec3D {
        Vec3D::new(-self.x, -self.y, -self.z)
    }
}

/// Implements the `ApproxEq` trait to approximate the equality of two
/// `Vec3D`s. The implementation uses the Euclidean distance between the two
/// `Vec3D`s to perform the comparison.
impl<'a> ApproxEq<&'a Vec3D> for &'a Vec3D {
    fn approx_eq(self, other: &'a Vec3D) -> bool {
        (self - other).squared_length() < TOLERANCE*TOLERANCE
    }
}

/// Reuses the implementation `ApproxEq<&Vec3D> for &Vec3D`.
impl<'a> ApproxEq<Vec3D> for &'a Vec3D {
    fn approx_eq(self, other: Vec3D) -> bool {
        self.approx_eq(&other)
    }
}

/// Reuses the implementation `ApproxEq<&Vec3D> for &Vec3D`.
impl<'a> ApproxEq<&'a Vec3D> for Vec3D {
    fn approx_eq(self, other: &'a Vec3D) -> bool {
        (&self).approx_eq(other)
    }
}

/// Reuses the implementation `ApproxEq<&Vec3D> for &Vec3D`.
impl ApproxEq<Vec3D> for Vec3D {
    fn approx_eq(self, other: Vec3D) -> bool {
        (&self).approx_eq(&other)
    }
}

impl<'a> DotProduct<&'a Vec3D> for Vec3D {
    /// Computes the dot product between two vectors.
    #[inline(always)]
    fn dot(&self, other: &'a Vec3D) -> Scalar {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl DotProduct<Vec3D> for Vec3D {
    /// Computes the dot product between two vectors.
    #[inline(always)]
    fn dot(&self, other: Vec3D) -> Scalar {
        self.dot(&other)
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vec3D`s.
impl<'a, 'b> Add<&'a Vec3D> for &'b Vec3D {
    type Output = Vec3D;

    #[inline]
    fn add(self, other: &'a Vec3D) -> Vec3D {
        Vec3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
implement_op_overload_variants!(Add, add, Vec3D, Vec3D, Vec3D);

/// Implement the `Sub` trait to allow using the `-` operator between `Vec3D`s.
impl<'a, 'b> Sub<&'b Vec3D> for &'a Vec3D {
    type Output = Vec3D;

    #[inline]
    fn sub(self, other: &'b Vec3D) -> Self::Output {
        Vec3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
implement_op_overload_variants!(Sub, sub, Vec3D, Vec3D, Vec3D);

impl<'a, 'b> CrossProduct<&'a Vec3D> for &'b Vec3D {
    type Output = Vec3D;

    /// Computes the cross product between two vectors.
    #[inline]
    fn cross(self, other: &'a Vec3D) -> Self::Output {
        Vec3D::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
}
implement_op_overload_variants!(CrossProduct, cross, Vec3D, Vec3D, Vec3D);

/// Implement the `Mul` trait to allow using the `*` operator for a `Vec3D`
/// with a `Scalar`.
impl<'a> Mul<Scalar> for &'a Vec3D {
    type Output = Vec3D;

    #[inline]
    fn mul(self, s: Scalar) -> Vec3D {
        Vec3D::new(self.x*s, self.y*s, self.z*s)
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Vec3D`
/// with a `Scalar`.
impl Mul<Scalar> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn mul(self, s: Scalar) -> Vec3D {
        &self * s
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vec3D`.
impl Mul<Vec3D> for Scalar {
    type Output = Vec3D;

    #[inline]
    fn mul(self, vector: Vec3D) -> Vec3D {
        &vector * self
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vec3D`.
impl<'a> Mul<&'a Vec3D> for Scalar {
    type Output = Vec3D;

    #[inline]
    fn mul(self, vector: &'a Vec3D) -> Vec3D {
        vector * self
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vec3D`
/// with a `Scalar`.
impl<'a> Div<Scalar> for &'a Vec3D {
    type Output = Vec3D;

    #[inline]
    fn div(self, s: Scalar) -> Vec3D {
        Vec3D::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vec3D`
/// with a `Scalar`.
impl Div<Scalar> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn div(self, s: Scalar) -> Vec3D {
        Vec3D::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vec3D` and a
/// `[Scalar; 3]`.
impl AsRef<[Scalar; 3]> for Vec3D {
    #[inline]
    fn as_ref(&self) -> &[Scalar; 3] {
        unsafe { mem::transmute(self) }
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vec3D` and a
/// `(Scalar, Scalar, Scalar)`.
impl AsRef<(Scalar, Scalar, Scalar)> for Vec3D {
    #[inline]
    fn as_ref(&self) -> &(Scalar, Scalar, Scalar) {
        unsafe { mem::transmute(self) }
    }
}

impl Deref for Vec3D {
    type Target = (Scalar, Scalar, Scalar);

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl DerefMut for Vec3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}
