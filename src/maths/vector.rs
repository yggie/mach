use std::fmt;
use std::mem;
use std::ops::{ Add, Deref, DerefMut, Div, Mul, Neg, Sub };

use { Scalar, TOLERANCE };
use maths::{ ApproxEq, Matrix, Quat };

/// A representation of a 3-dimensional column vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    /// The x component of the vector.
    pub x: Scalar,
    /// The y component of the vector.
    pub y: Scalar,
    /// The z component of the vector.
    pub z: Scalar,
}

/// Static methods for the Vector struct.
impl Vector {
    /// A simple constructor which builds a column vector given three elements.
    #[inline(always)]
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }

    /// Constructs a zero vector.
    pub fn new_zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// Set the components of the `Vector` to the specified values.
    #[inline]
    pub fn set(&mut self, other: &(Scalar, Scalar, Scalar)) {
        self.x = other.0;
        self.y = other.1;
        self.z = other.2;
    }

    /// Computes the sum of the `Vector` and three scalars treated as components
    /// of a `Vector`.
    #[inline]
    pub fn add(&self, x: Scalar, y: Scalar, z: Scalar) -> Vector {
        Vector::new(self.x + x, self.y + y, self.z + z)
    }

    /// Computes the difference between a `Vector` and three scalars treated as
    /// components of a `Vector`.
    #[inline]
    pub fn sub(&self, x: Scalar, y: Scalar, z: Scalar) -> Vector {
        Vector::new(self.x - x, self.y - y, self.z - z)
    }

    /// Computes the dot product between two vectors.
    #[inline(always)]
    pub fn dot(&self, other: Vector) -> Scalar {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    /// Computes the cross product between two vectors.
    #[inline]
    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }

    /// Computes the direction vector of a Vector.
    #[inline]
    pub fn normalize(&self) -> Vector {
        *self / self.length()
    }

    /// Computes the squared length of a Vector.
    #[inline(always)]
    pub fn length_sq(&self) -> Scalar {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Computes the length of a Vector.
    #[inline(always)]
    pub fn length(&self) -> Scalar {
        self.length_sq().sqrt()
    }

    /// Computes the outer product between two Vectors.
    pub fn outer(&self, other: Vector) -> Matrix {
        return Matrix::new(
            self.x*other.x, self.x*other.y, self.x*other.z,
            self.y*other.x, self.y*other.y, self.y*other.z,
            self.z*other.x, self.z*other.y, self.z*other.z,
        );
    }

    /// Computes the distance to the `Vector` specified.
    pub fn distance_to(&self, other: Vector) -> Scalar {
        (*self - other).length()
    }

    /// Computes the `Vector` that is the result of being rotated by the input
    /// `Quat`.
    pub fn rotate_by_quaternion(&self, q: Quat) -> Vector {
        let result = q * Quat::new(0.0, self.x, self.y, self.z) * q.inverse();
        return Vector::new(result.i, result.j, result.k);
    }
}

/// Implements the `Display` trait to allow using `println!` on Vectors. The
/// resulting format is equivalent to:
///
/// ```rust
/// extern crate mach;
///
/// let vec = mach::Vector::new(0.1, 0.2, 0.3);
/// println!("[{}, {}, {}]", vec.x, vec.y, vec.z);
/// ```
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vector`s.
impl<'a> Neg for &'a Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// Implement the `Neg` trait to allow using the unary `-` operator for
/// `Vector`s.
impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// Implements the `ApproxEq` trait to approximate the equality of two
/// `Vector`s. The implementation uses the Euclidean distance between the two
/// `Vector`s to perform the comparison.
impl<'a> ApproxEq<&'a Vector> for &'a Vector {
    fn approx_eq(self, other: &'a Vector) -> bool {
        (self - other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Reuses the implementation `ApproxEq<&Vector> for &Vector`.
impl<'a> ApproxEq<Vector> for &'a Vector {
    fn approx_eq(self, other: Vector) -> bool {
        self.approx_eq(&other)
    }
}

/// Reuses the implementation `ApproxEq<&Vector> for &Vector`.
impl<'a> ApproxEq<&'a Vector> for Vector {
    fn approx_eq(self, other: &'a Vector) -> bool {
        (&self).approx_eq(other)
    }
}

/// Reuses the implementation `ApproxEq<&Vector> for &Vector`.
impl ApproxEq<Vector> for Vector {
    fn approx_eq(self, other: Vector) -> bool {
        (&self).approx_eq(&other)
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vector`s.
impl<'a, 'b> Add<&'a Vector> for &'b Vector {
    type Output = Vector;

    #[inline]
    fn add(self, other: &'a Vector) -> Vector {
        Vector::add(self, other.x, other.y, other.z)
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vector`s.
impl<'a> Add<Vector> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn add(self, other: Vector) -> Vector {
        self + &other
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vector`s.
impl<'a> Add<&'a Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, other: &'a Vector) -> Vector {
        &self + other
    }
}

/// Implement the `Add` trait to allow using the `+` operator between `Vector`s.
impl Add<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, other: Vector) -> Vector {
        &self + &other
    }
}

/// Implement the `Sub` trait to allow using the `-` operator between `Vector`s.
impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, other: &'b Vector) -> Self::Output {
        Vector::sub(self, other.x, other.y, other.z)
    }
}

/// Implement the `Sub` trait to allow using the `-` operator between `Vector`s.
impl<'a> Sub<&'a Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, other: &'a Vector) -> Self::Output {
        &self - other
    }
}

/// Implement the `Sub` trait to allow using the `-` operator between `Vector`s.
impl<'a> Sub<Vector> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, other: Vector) -> Self::Output {
        self - &other
    }
}

/// Implement the `Sub` trait to allow using the `-` operator between `Vector`s.
impl Sub<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, other: Vector) -> Self::Output {
        &self - &other
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Vector`
/// with a `Scalar`.
impl<'a> Mul<Scalar> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, s: Scalar) -> Vector {
        Vector::new(self.x*s, self.y*s, self.z*s)
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Vector`
/// with a `Scalar`.
impl Mul<Scalar> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, s: Scalar) -> Vector {
        &self * s
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vector`.
impl Mul<Vector> for Scalar {
    type Output = Vector;

    #[inline]
    fn mul(self, vector: Vector) -> Vector {
        &vector * self
    }
}

/// Implement the `Mul` trait to allow using the `*` operator for a `Scalar`
/// with a `Vector`.
impl<'a> Mul<&'a Vector> for Scalar {
    type Output = Vector;

    #[inline]
    fn mul(self, vector: &'a Vector) -> Vector {
        vector * self
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vector`
/// with a `Scalar`.
impl<'a> Div<Scalar> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn div(self, s: Scalar) -> Vector {
        Vector::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implement the `Div` trait to allow using the `/` operator for a `Vector`
/// with a `Scalar`.
impl Div<Scalar> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, s: Scalar) -> Vector {
        Vector::new(self.x/s, self.y/s, self.z/s)
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vector` and a
/// `[Scalar; 3]`.
impl AsRef<[Scalar; 3]> for Vector {
    #[inline]
    fn as_ref(&self) -> &[Scalar; 3] {
        unsafe { mem::transmute(self) }
    }
}

/// Implements the `AsRef` trait to allow conversion between a `Vector` and a
/// `(Scalar, Scalar, Scalar)`.
impl AsRef<(Scalar, Scalar, Scalar)> for Vector {
    #[inline]
    fn as_ref(&self) -> &(Scalar, Scalar, Scalar) {
        unsafe { mem::transmute(self) }
    }
}

impl Deref for Vector {
    type Target = (Scalar, Scalar, Scalar);

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}
