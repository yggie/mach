use std::fmt;
use std::ops::{ Add, Div, Index, IndexMut, Mul, Neg, Sub };

use { Float, TOLERANCE };
use maths::Vector;

/// A representation of a quaternion.
#[derive(Clone, Copy, Debug)]
pub struct Quat {
    elements: [Float; 4]
}

impl Quat {
    /// Creates a new `Quat` with the coordinates provided.
    #[inline(always)]
    pub fn new(r: Float, i: Float, j: Float, k: Float) -> Quat {
        Quat { elements: [r, i, j, k] }
    }

    /// Creates a new `Quat` representing an identity transformation.
    #[inline(always)]
    pub fn new_identity() -> Quat {
        Quat { elements: [1.0, 0.0, 0.0, 0.0] }
    }

    /// Creates a new `Quat` taking the input `Vector` as the components
    /// of the complex part of the `Quat`.
    #[inline]
    pub fn new_from_vector(vector: Vector) -> Quat {
        Quat::new(0.0, vector.x, vector.y, vector.z)
    }

    /// Creates a new `Quat` representing a rotation about an axis.
    pub fn new_from_axis_angle(axis: Vector, angle_in_radians: Float) -> Quat {
        let length = axis.length();
        let half_radians = angle_in_radians / 2.0;
        let sl = half_radians.sin() / length;
        let c = half_radians.cos();

        return Quat::new(c, sl*axis.x, sl*axis.y, sl*axis.z);
    }

    /// Computes the squared length of the `Quat`.
    #[inline(always)]
    pub fn length_sq(&self) -> Float {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3]
    }

    /// Computes the length of the `Quat`.
    #[inline]
    pub fn length(&self) -> Float {
        self.length_sq().sqrt()
    }

    /// Computes a unit `Quat` with the same direction as the current
    /// `Quat`.
    #[inline]
    pub fn normalize(&self) -> Quat {
        *self / self.length()
    }

    /// Computes the inverse of the `Quat`.
    #[inline]
    pub fn inverse(&self) -> Quat {
        let denom = self.length_sq();
        Quat::new(self[0]/denom, -self[1]/denom, -self[2]/denom, -self[3]/denom)
    }

    /// Sets the components of the `Quat` to the specified values.
    #[inline]
    pub fn set(&mut self, r: Float, i: Float, j: Float, k: Float) {
        self[0] = r;
        self[1] = i;
        self[2] = j;
        self[3] = k;
    }

    /// Copies the contents of the provided `Quat`.
    #[inline]
    pub fn copy(&mut self, q: Quat) {
        self.set(q[0], q[1], q[2], q[3]);
    }

    /// Computes the sum between the `Quat` and the input scalars treated
    /// as components of a `Quat`.
    #[inline]
    pub fn add(self, r: Float, i: Float, j: Float, k: Float) -> Quat {
        Quat{ elements: [
            self[0] + r,
            self[1] + i,
            self[2] + j,
            self[3] + k,
        ] }

    }

    /// Computes the difference between the `Quat` and the input scalars
    /// treated as components of a `Quat`.
    #[inline]
    pub fn sub(self, r: Float, i: Float, j: Float, k: Float) -> Quat {
        Quat{ elements: [
            self[0] - r,
            self[1] - i,
            self[2] - j,
            self[3] - k,
        ] }
    }

    /// Computes the `Quat` multiplication with the input scalars as
    /// components of a `Quat`.
    #[inline]
    pub fn mult(&self, r: Float, i: Float, j: Float, k: Float) -> Quat {
        Quat{ elements: [
            self[0]*r - self[1]*i - self[2]*j - self[3]*k,
            self[0]*i + self[1]*r + self[2]*k - self[3]*j,
            self[0]*j - self[1]*k + self[2]*r + self[3]*i,
            self[0]*k + self[1]*j - self[2]*i + self[3]*r,
        ] }
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on
/// `Quat`s
impl fmt::Display for Quat {
    /// Implements the fmt operation for `Quat`s. The resulting format is
    /// equivalent to:
    ///
    /// ```rust,ignore
    /// println!("[{}, {}, {}, {}]", quat[0], quat[1], quat[2], quat[3]);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}

/// Guarantees that equality satisfies the equivalence relations.
impl Eq for Quat { }

/// Implements the equality operators: `==` and `!=`.
impl PartialEq for Quat {
    /// Implements the equality operator for a `Quat`. Two `Quat`s
    /// are equal if the Euclidean distance between the two is below a
    /// threshold.
    fn eq(&self, other: &Quat) -> bool {
        (*self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Implements the index operator.
impl Index<usize> for Quat {
    type Output = Float;

    /// Obtains a component from the `Quat` by index.
    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a Float {
        &self.elements[index]
    }
}

/// Implements the mutable index operator.
impl IndexMut<usize> for Quat {
    /// Obtains a mutable reference to a component from the `Quat` by
    /// index.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Float {
        &mut self.elements[index]
    }
}

/// Implements the unary negation operator.
impl Neg for Quat {
    type Output = Quat;

    /// Reverses the direction of the quaternion.
    #[inline]
    fn neg(self) -> Quat {
        Quat{ elements: [ -self[0], -self[1], -self[2], -self[3] ] }
    }
}

/// Implements the addition operator.
impl Add<Quat> for Quat {
    type Output = Quat;

    /// Computes the sum of two `Quat`s.
    #[inline]
    fn add(self, other: Quat) -> Quat {
        self.add(other[0], other[1], other[2], other[3])
    }
}

/// Implements the subtraction operator.
impl Sub<Quat> for Quat {
    type Output = Quat;

    /// Computes the difference between two `Quat`s.
    #[inline]
    fn sub(self, other: Quat) -> Quat {
        self.sub(other[0], other[1], other[2], other[3])
    }
}

/// Implements the multiplication operator between a `Quat` and a scalar.
impl Mul<Float> for Quat {
    type Output = Quat;

    /// Computes the result of multiplying a `Quat` by a scalar.
    fn mul(self, s: Float) -> Quat {
        Quat::new(self[0]*s, self[1]*s, self[2]*s, self[3]*s)
    }
}

/// Implements the multiplication operator between two `Quat`s.
impl Mul<Quat> for Quat {
    type Output = Quat;

    /// Multiplies two quaternions and returns the result.
    #[inline]
    fn mul(self, other: Quat) -> Quat {
        self.mult(other[0], other[1], other[2], other[3])
    }
}

/// Implements the division operator between a `Quat` and a scalar.
impl Div<Float> for Quat {
    type Output = Quat;

    /// Divides the `Quat` by a scalar.
    #[inline]
    fn div(self, s: Float) -> Quat {
        Quat::new(self[0]/s, self[1]/s, self[2]/s, self[3]/s)
    }
}
