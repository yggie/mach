use std::fmt;
use std::ops::{ Add, Div, Mul, Neg, Sub };

use { Float, TOLERANCE };
use maths::{ Matrix, Quat };

/// A representation of a 3-dimensional column vector.
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    /// The x component of the vector.
    pub x: Float,
    /// The y component of the vector.
    pub y: Float,
    /// The z component of the vector.
    pub z: Float,
}

/// Static methods for the Vector struct.
impl Vector {
    /// A simple constructor which builds a column vector given three elements.
    #[inline(always)]
    pub fn new(x: Float, y: Float, z: Float) -> Vector {
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
    pub fn set(&mut self, x: Float, y: Float, z: Float) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Computes the sum of the `Vector` and three scalars treated as components
    /// of a `Vector`.
    #[inline]
    pub fn add(self, x: Float, y: Float, z: Float) -> Vector {
        Vector::new(self.x + x, self.y + y, self.z + z)
    }

    /// Computes the difference between a `Vector` and three scalars treated as
    /// components of a `Vector`.
    #[inline]
    pub fn sub(self, x: Float, y: Float, z: Float) -> Vector {
        Vector::new(self.x - x, self.y - y, self.z - z)
    }

    /// Computes the dot product between two vectors.
    #[inline(always)]
    pub fn dot(&self, other: Vector) -> Float {
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
    pub fn length_sq(&self) -> Float {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Computes the length of a Vector.
    #[inline(always)]
    pub fn length(&self) -> Float {
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
    pub fn distance_to(&self, other: Vector) -> Float {
        (*self - other).length()
    }

    /// Computes the `Vector` that is the result of being rotated by the input
    /// `Quat`.
    pub fn rotate_by_quaternion(&self, q: Quat) -> Vector {
        let result = q * Quat::new(0.0, self.x, self.y, self.z) * q.inverse();
        return Vector::new(result[1], result[2], result[3]);
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on Vectors.
impl fmt::Display for Vector {
    /// Implements the fmt operation for `Vector`s. The resulting format is
    /// equivalent to:
    ///
    /// ```rust,ignore
    /// println!("[{}, {}, {}]", vec.x, vec.y, vec.z);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

/// Implementation for the equality operations, allows the use of `==` and `!=`
/// operators on `Vector`s.
impl PartialEq for Vector {
    /// Implements the equality operator for Vectors. Returns true if the
    /// Euclidean distance between the two vectors is below an allowed
    /// tolerance.
    fn eq(&self, other: &Vector) -> bool {
        (*self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Implement the unary negation operator.
impl Neg for Vector {
    type Output = Vector;

    /// Reverses the direction of the vector.
    #[inline]
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// Implement the addition operator between Vectors.
impl Add<Vector> for Vector {
    type Output = Vector;

    /// Calculates the sum of two vectors.
    #[inline]
    fn add(self, other: Vector) -> Vector {
        self.add(other.x, other.y, other.z)
    }
}

/// Implement the subtraction operator between Vectors.
impl Sub<Vector> for Vector {
    type Output = Vector;

    /// Calculates the difference between two vectors.
    #[inline]
    fn sub(self, other: Vector) -> Vector {
        self.sub(other.x, other.y, other.z)
    }
}

/// Implements the multiplication operator between a `Vector` and a scalar.
impl Mul<Float> for Vector {
    type Output = Vector;

    /// Multiplies a `Vector` by a scalar.
    #[inline]
    fn mul(self, s: Float) -> Vector {
        Vector::new(self.x*s, self.y*s, self.z*s)
    }
}

/// Implements the division operator between a `Vector` and a scalar.
impl Div<Float> for Vector {
    type Output = Vector;

    /// Divides the `Vector` by a scalar.
    #[inline]
    fn div(self, s: Float) -> Vector {
        Vector::new(self.x/s, self.y/s, self.z/s)
    }
}
