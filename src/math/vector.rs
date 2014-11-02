use math::{ TOLERANCE, Matrix };

use std::fmt;

#[cfg(test)]
#[path="../../tests/unit/math/vector_test.rs"]
mod tests;

/// A representation of a 3-dimensional column vector.
pub struct Vector {
    elements: [f32, ..3]
}

/// Static methods for the Vector struct.
impl Vector {

    /// A simple constructor which builds a column vector given three elements.
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{ elements: [x, y, z] }
    }

    /// Constructs a zero vector.
    pub fn new_zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// Set the components of the `Vector` to the specified values.
    #[inline]
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self[0] = x;
        self[1] = y;
        self[2] = z;
    }

    /// Computes the sum of the `Vector` and three scalars treated as components
    /// of a `Vector`.
    #[inline]
    pub fn add(&self, x: f32, y: f32, z: f32) -> Vector {
        Vector{ elements: [
            self[0] + x,
            self[1] + y,
            self[2] + z,
        ] }
    }

    /// Computes the difference between a `Vector` and three scalars treated as
    /// components of a `Vector`.
    #[inline]
    pub fn sub(&self, x: f32, y: f32, z: f32) -> Vector {
        Vector{ elements: [
            self[0] - x,
            self[1] - y,
            self[2] - z,
        ] }
    }

    /// Computes the dot product between two vectors.
    #[inline(always)]
    pub fn dot(&self, other: Vector) -> f32 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }

    /// Computes the cross product between two vectors.
    #[inline]
    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self[1]*other[2] - self[2]*other[1],
            self[2]*other[0] - self[0]*other[2],
            self[0]*other[1] - self[1]*other[0],
        )
    }

    /// Computes the direction vector of a Vector.
    #[inline]
    pub fn normalize(&self) -> Vector {
        self / self.length()
    }

    /// Computes the squared length of a Vector.
    #[inline(always)]
    pub fn length_sq(&self) -> f32 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    /// Computes the length of a Vector.
    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Computes the outer product between two Vectors.
    pub fn outer(&self, other: Vector) -> Matrix {
        let elems: [f32, ..9] = [
            self[0]*other[0], self[0]*other[1], self[0]*other[2],
            self[1]*other[0], self[1]*other[1], self[1]*other[2],
            self[2]*other[0], self[2]*other[1], self[2]*other[2],
        ];
        Matrix::new(&elems)
    }
}

/// Implements the clone operation.
impl Clone for Vector {
    /// Returns a copy of the `Vector`.
    fn clone(&self) -> Vector {
        Vector::new(self[0], self[1], self[2])
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on Vectors.
impl fmt::Show for Vector {
    /// Implements the fmt operation for `Vector`s. The resulting format is
    /// equivalent to:
    ///
    /// ```rust,ignore
    /// println!("[{}, {}, {}]", vec[0], vec[1], vec[2]);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

/// Guarantees that equality satisfies the equivalence relations.
impl Eq for Vector { }

/// Implementation for the equality operations, allows the use of `==` and `!=`
/// operators on `Vector`s.
impl PartialEq for Vector {
    /// Implements the equality operator for Vectors. Returns true if the
    /// Euclidean distance between the two vectors is below an allowed
    /// tolerance.
    fn eq(&self, other: &Vector) -> bool {
        (self - *other).length_sq() < TOLERANCE*TOLERANCE
    }
}

/// Implement the index operator.
impl Index<uint, f32> for Vector {
    /// Obtain the vector's elements by index. Uses zero-based indexing.
    #[inline(always)]
    fn index<'a>(&'a self, index: &uint) -> &'a f32 {
        &self.elements[*index]
    }
}

/// Implement the mutable index operator.
impl IndexMut<uint, f32> for Vector {
    /// Allows setting a vector's element using index notation.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f32 {
        &mut self.elements[*index]
    }
}

/// Implement the unary negation operator.
impl Neg<Vector> for Vector {
    /// Reverses the direction of the vector.
    #[inline]
    fn neg(&self) -> Vector {
        Vector{ elements: [ -self[0], -self[1], -self[2] ] }
    }
}

/// Implement the addition operator between Vectors.
impl Add<Vector, Vector> for Vector {
    /// Calculates the sum of two vectors.
    #[inline]
    fn add(&self, other: &Vector) -> Vector {
        self.add(other[0], other[1], other[2])
    }
}

/// Implement the subtraction operator between Vectors.
impl Sub<Vector, Vector> for Vector {
    /// Calculates the difference between two vectors.
    #[inline]
    fn sub(&self, other: &Vector) -> Vector {
        self.sub(other[0], other[1], other[2])
    }
}

/// Implements the multiplication operator between a `Vector` and a scalar.
impl Mul<f32, Vector> for Vector {
    /// Multiplies a `Vector` by a scalar.
    #[inline]
    fn mul(&self, scale: &f32) -> Vector {
        let s = *scale;
        Vector::new(self[0]*s, self[1]*s, self[2]*s)
    }
}

/// Implements the division operator between a `Vector` and a scalar.
impl Div<f32, Vector> for Vector {
    /// Divides the `Vector` by a scalar.
    #[inline]
    fn div(&self, scale: &f32) -> Vector {
        let s = *scale;
        Vector::new(self[0]/s, self[1]/s, self[2]/s)
    }
}
