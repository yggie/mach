use math::TOLERANCE;

use std::fmt;

#[cfg(test)]
#[path="../../tests/unit/math/quaternion_test.rs"]
mod tests;

/// A representation of a quaternion.
pub struct Quaternion {
    elements: [f32, ..4]
}

impl Quaternion {
    /// Creates a new `Quaternion` with the coordinates provided.
    #[inline(always)]
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion{ elements: [r, i, j, k] }
    }

    /// Creates a new `Quaternion` representing an identity transformation.
    #[inline(always)]
    pub fn new_identity() -> Quaternion {
        Quaternion{ elements: [1.0, 0.0, 0.0, 0.0] }
    }

    /// Computes the squared length of the `Quaternion`.
    #[inline(always)]
    pub fn length_sq(&self) -> f32 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3]
    }

    /// Computes the length of the `Quaternion`.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Computes a unit `Quaternion` with the same direction as the current
    /// `Quaternion`.
    #[inline]
    pub fn normalize(&self) -> Quaternion {
        self / self.length()
    }

    /// Computes the difference between the `Quaternion` and the input scalars
    /// treated as components of a `Quaternion`.
    #[inline]
    pub fn sub(&self, r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion{ elements: [
            self[0] - r,
            self[1] - i,
            self[2] - j,
            self[3] - k,
        ] }
    }

    /// Computes the `Quaternion` multiplication with the input scalars as
    /// components of a `Quaternion`.
    #[inline]
    pub fn mult(&self, r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion{ elements: [
            self[0]*r - self[1]*i - self[2]*j - self[3]*k,
            self[0]*i + self[1]*r + self[2]*k - self[3]*j,
            self[0]*j - self[1]*k + self[2]*r + self[3]*i,
            self[0]*k + self[1]*j - self[2]*i + self[3]*r,
        ] }
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on
/// `Quaternion`s
impl fmt::Show for Quaternion {
    /// Implements the fmt operation for `Quaternion`s. The resulting format is
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
impl Eq for Quaternion { }

/// Implements the equality operators: `==` and `!=`.
impl PartialEq for Quaternion {
    /// Implements the equality operator for a `Quaternion`. Two `Quaternion`s
    /// are equal if the Euclidean distance between the two is below a
    /// threshold.
    fn eq(&self, other: &Quaternion) -> bool {
        (self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Implements the index operator.
impl Index<uint, f32> for Quaternion {
    /// Obtains a component from the `Quaternion` by index.
    #[inline(always)]
    fn index<'a>(&'a self, index: &uint) -> &'a f32 {
        &self.elements[*index]
    }
}

/// Implements the mutable index operator.
impl IndexMut<uint, f32> for Quaternion {
    /// Obtains a mutable reference to a component from the `Quaternion` by
    /// index.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f32 {
        &mut self.elements[*index]
    }
}

/// Implements the unary negation operator.
impl Neg<Quaternion> for Quaternion {
    /// Reverses the direction of the quaternion.
    #[inline]
    fn neg(&self) -> Quaternion {
        Quaternion{ elements: [ -self[0], -self[1], -self[2], -self[3] ] }
    }
}

/// Implements the subtraction operator.
impl Sub<Quaternion, Quaternion> for Quaternion {
    /// Computes the difference between two `Quaternion`s.
    #[inline]
    fn sub(&self, other: &Quaternion) -> Quaternion {
        self.sub(other[0], other[1], other[2], other[3])
    }
}

/// Implements the multiplication operator between a `Quaternion` and a scalar.
impl Mul<f32, Quaternion> for Quaternion {
    /// Computes the result of multiplying a `Quaternion` by a scalar.
    fn mul(&self, scalar: &f32) -> Quaternion {
        let s = *scalar;
        Quaternion::new(self[0]*s, self[1]*s, self[2]*s, self[3]*s)
    }
}

/// Implements the multiplication operator between two `Quaternion`s.
impl Mul<Quaternion, Quaternion> for Quaternion {
    /// Multiplies two quaternions and returns the result.
    #[inline]
    fn mul(&self, other: &Quaternion) -> Quaternion {
        self.mult(other[0], other[1], other[2], other[3])
    }
}

/// Implements the division operator between a `Quaternion` and a scalar.
impl Div<f32, Quaternion> for Quaternion {
    /// Divides the `Quaternion` by a scalar.
    #[inline]
    fn div(&self, scalar: &f32) -> Quaternion {
        let s = *scalar;
        Quaternion::new(self[0]/s, self[1]/s, self[2]/s, self[3]/s)
    }
}
