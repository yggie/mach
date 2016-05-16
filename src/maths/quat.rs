#[cfg(test)]
#[path="../../tests/maths/quat_test.rs"]
mod quat_test;

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use {Scalar, TOLERANCE};
use maths::{ApproxEq, UnitQuat, Vec3D};

/// A representation of a quaternion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    /// The real component of the `Quat`.
    pub r: Scalar,
    /// The first imaginary component of the `Quat`.
    pub i: Scalar,
    /// The second imaginary component of the `Quat`.
    pub j: Scalar,
    /// The third imaginary component of the `Quat`.
    pub k: Scalar,
}

impl Quat {
    /// Creates a new `Quat` with the coordinates provided.
    #[inline(always)]
    pub fn new(r: Scalar, i: Scalar, j: Scalar, k: Scalar) -> Quat {
        Quat {
            r: r,
            i: i,
            j: j,
            k: k,
        }
    }

    /// Creates a new `Quat` representing an identity transformation.
    #[inline(always)]
    pub fn identity() -> Quat {
        Quat::new(1.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new `Quat` taking the input `Vec3D` as the components
    /// of the complex part of the `Quat`.
    #[inline]
    pub fn from_vector(vector: Vec3D) -> Quat {
        Quat::new(0.0, vector.x, vector.y, vector.z)
    }

    /// Computes the squared length of the `Quat`.
    #[inline(always)]
    pub fn squared_length(&self) -> Scalar {
        self.r*self.r + self.i*self.i + self.j*self.j + self.k*self.k
    }

    /// Computes the length of the `Quat`.
    #[inline]
    pub fn length(&self) -> Scalar {
        self.squared_length().sqrt()
    }

    /// Computes a unit `Quat` with the same direction as the current
    /// `Quat`.
    #[inline]
    pub fn normalize(&self) -> UnitQuat {
        UnitQuat::from_quat(self.clone())
    }

    /// Computes the inverse of the `Quat`.
    #[inline]
    pub fn inverse(&self) -> Quat {
        let denom = self.squared_length();
        Quat::new(self.r/denom, -self.i/denom, -self.j/denom, -self.k/denom)
    }

    /// Multiples each component of the `Quat` by the scalar.
    #[inline]
    pub fn mult_scalar(&self, s: Scalar) -> Quat {
        Quat::new(self.r * s, self.i * s, self.j * s, self.k * s)
    }

    /// Divides each component of the `Quat` by the scalar.
    #[inline]
    pub fn div_scalar(&self, s: Scalar) -> Quat {
        Quat::new(self.r / s, self.i / s, self.j / s, self.k / s)
    }

    /// Computes the `Quat` multiplication with the input scalars as
    /// components of a `Quat`.
    #[inline]
    pub fn mult_quat(&self, r: Scalar, i: Scalar, j: Scalar, k: Scalar) -> Quat {
        Quat::new(
            self.r*r - self.i*i - self.j*j - self.k*k,
            self.r*i + self.i*r + self.j*k - self.k*j,
            self.r*j - self.i*k + self.j*r + self.k*i,
            self.r*k + self.i*j - self.j*i + self.k*r,
        )
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on
/// `Quat`s
impl fmt::Display for Quat {
    /// Implements the fmt operation for `Quat`s. The resulting format is
    /// equivalent to:
    ///
    /// ```rust,ignore
    /// println!("[{}, {}, {}, {}]", quat.r, quat.i, quat.j, quat.k);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.r, self.i, self.j, self.k)
    }
}

/// Guarantees that equality satisfies the equivalence relations.
impl Eq for Quat { }

/// Implements the unary negation operator.
impl Neg for Quat {
    type Output = Quat;

    /// Reverses the direction of the quaternion.
    #[inline]
    fn neg(self) -> Quat {
        Quat::new(-self.r, -self.i, -self.j, -self.k)
    }
}

/// Implements the addition operator.
impl<'a, 'b> Add<&'a Quat> for &'b Quat {
    type Output = Quat;

    #[inline]
    fn add(self, other: &'a Quat) -> Quat {
        Quat::new(
            self.r + other.r,
            self.i + other.i,
            self.j + other.j,
            self.k + other.k,
        )
    }
}
implement_op_overload_variants!(Add, add, Quat, Quat, Quat);

/// Implements the subtraction operator.
impl<'a, 'b> Sub<&'a Quat> for &'b Quat {
    type Output = Quat;

    #[inline]
    fn sub(self, other: &'a Quat) -> Quat {
        Quat::new(
            self.r - other.r,
            self.i - other.i,
            self.j - other.j,
            self.k - other.k,
        )
    }
}
implement_op_overload_variants!(Sub, sub, Quat, Quat, Quat);

/// Implements the multiplication operator between a `Quat` and a scalar.
impl<'a> Mul<Scalar> for &'a Quat {
    type Output = Quat;

    fn mul(self, s: Scalar) -> Quat {
        Quat::mult_scalar(self, s)
    }
}

/// Implements the multiplication operator between a `Quat` and a scalar.
impl Mul<Scalar> for Quat {
    type Output = Quat;

    fn mul(self, s: Scalar) -> Quat {
        &self * s
    }
}

/// Implements the multiplication operator between two `Quat`s.
impl<'a, 'b> Mul<&'a Quat> for &'b Quat {
    type Output = Quat;

    /// Multiplies two quaternions and returns the result.
    #[inline]
    fn mul(self, other: &'a Quat) -> Quat {
        Quat::mult_quat(self, other.r, other.i, other.j, other.k)
    }
}
implement_op_overload_variants!(Mul, mul, Quat, Quat, Quat);

/// Implements the division operator between a `Quat` and a scalar.
impl<'a> Div<Scalar> for &'a Quat {
    type Output = Quat;

    /// Divides the `Quat` by a scalar.
    #[inline]
    fn div(self, s: Scalar) -> Quat {
        Quat::div_scalar(self, s)
    }
}

/// Implements the division operator between a `Quat` and a scalar.
impl Div<Scalar> for Quat {
    type Output = Quat;

    /// Divides the `Quat` by a scalar.
    #[inline]
    fn div(self, s: Scalar) -> Quat {
        &self / s
    }
}

/// Implements the `ApproxEq` trait to approximate the equality of two
/// `Quat`s. The implementation uses the Euclidean distance between the two
/// `Quat`s to perform the comparison.
impl<'a> ApproxEq<&'a Quat> for &'a Quat {
    fn approx_eq(self, other: &'a Quat) -> bool {
        (self - other).squared_length() < TOLERANCE*TOLERANCE
    }
}

/// Reuses the implementation `ApproxEq<&Quat> for &Quat`.
impl<'a> ApproxEq<Quat> for &'a Quat {
    fn approx_eq(self, other: Quat) -> bool {
        self.approx_eq(&other)
    }
}

/// Reuses the implementation `ApproxEq<&Quat> for &Quat`.
impl<'a> ApproxEq<&'a Quat> for Quat {
    fn approx_eq(self, other: &'a Quat) -> bool {
        (&self).approx_eq(other)
    }
}

/// Reuses the implementation `ApproxEq<&Quat> for &Quat`.
impl ApproxEq<Quat> for Quat {
    fn approx_eq(self, other: Quat) -> bool {
        (&self).approx_eq(&other)
    }
}
