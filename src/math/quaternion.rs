use math::{ TOLERANCE, Vector };

use std::fmt;
use std::ops::{ Add, Div, Index, IndexMut, Mul, Neg, Sub };

/// A representation of a quaternion.
#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    elements: [f32; 4]
}

impl Quaternion {
    /// Creates a new `Quaternion` with the coordinates provided.
    #[inline(always)]
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion { elements: [r, i, j, k] }
    }

    /// Creates a new `Quaternion` representing an identity transformation.
    #[inline(always)]
    pub fn new_identity() -> Quaternion {
        Quaternion { elements: [1.0, 0.0, 0.0, 0.0] }
    }

    /// Creates a new `Quaternion` taking the input `Vector` as the components
    /// of the complex part of the `Quaternion`.
    #[inline]
    pub fn new_from_vector(vector: Vector) -> Quaternion {
        Quaternion::new(0.0, vector[0], vector[1], vector[2])
    }

    /// Creates a new `Quaternion` representing a rotation about an axis.
    pub fn new_from_axis_angle(axis: Vector, angle_in_radians: f32) -> Quaternion {
        let length = axis.length();
        let half_radians = angle_in_radians / 2.0;
        let sl = half_radians.sin() / length;
        let c = half_radians.cos();

        return Quaternion::new(c, sl*axis[0], sl*axis[1], sl*axis[2]);
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
        *self / self.length()
    }

    /// Computes the inverse of the `Quaternion`.
    #[inline]
    pub fn inverse(&self) -> Quaternion {
        let denom = self.length_sq();
        Quaternion::new(self[0]/denom, -self[1]/denom, -self[2]/denom, -self[3]/denom)
    }

    /// Sets the components of the `Quaternion` to the specified values.
    #[inline]
    pub fn set(&mut self, r: f32, i: f32, j: f32, k: f32) {
        self[0] = r;
        self[1] = i;
        self[2] = j;
        self[3] = k;
    }

    /// Computes the sum between the `Quaternion` and the input scalars treated
    /// as components of a `Quaternion`.
    #[inline]
    pub fn add(self, r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion{ elements: [
            self[0] + r,
            self[1] + i,
            self[2] + j,
            self[3] + k,
        ] }

    }

    /// Computes the difference between the `Quaternion` and the input scalars
    /// treated as components of a `Quaternion`.
    #[inline]
    pub fn sub(self, r: f32, i: f32, j: f32, k: f32) -> Quaternion {
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
impl fmt::Display for Quaternion {
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
        (*self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Implements the index operator.
impl Index<usize> for Quaternion {
    type Output = f32;

    /// Obtains a component from the `Quaternion` by index.
    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a f32 {
        &self.elements[index]
    }
}

/// Implements the mutable index operator.
impl IndexMut<usize> for Quaternion {
    /// Obtains a mutable reference to a component from the `Quaternion` by
    /// index.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        &mut self.elements[index]
    }
}

/// Implements the unary negation operator.
impl Neg for Quaternion {
    type Output = Quaternion;

    /// Reverses the direction of the quaternion.
    #[inline]
    fn neg(self) -> Quaternion {
        Quaternion{ elements: [ -self[0], -self[1], -self[2], -self[3] ] }
    }
}

/// Implements the addition operator.
impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Computes the sum of two `Quaternion`s.
    #[inline]
    fn add(self, other: Quaternion) -> Quaternion {
        self.add(other[0], other[1], other[2], other[3])
    }
}

/// Implements the subtraction operator.
impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Computes the difference between two `Quaternion`s.
    #[inline]
    fn sub(self, other: Quaternion) -> Quaternion {
        self.sub(other[0], other[1], other[2], other[3])
    }
}

/// Implements the multiplication operator between a `Quaternion` and a scalar.
impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    /// Computes the result of multiplying a `Quaternion` by a scalar.
    fn mul(self, s: f32) -> Quaternion {
        Quaternion::new(self[0]*s, self[1]*s, self[2]*s, self[3]*s)
    }
}

/// Implements the multiplication operator between two `Quaternion`s.
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Multiplies two quaternions and returns the result.
    #[inline]
    fn mul(self, other: Quaternion) -> Quaternion {
        self.mult(other[0], other[1], other[2], other[3])
    }
}

/// Implements the division operator between a `Quaternion` and a scalar.
impl Div<f32> for Quaternion {
    type Output = Quaternion;

    /// Divides the `Quaternion` by a scalar.
    #[inline]
    fn div(self, s: f32) -> Quaternion {
        Quaternion::new(self[0]/s, self[1]/s, self[2]/s, self[3]/s)
    }
}
