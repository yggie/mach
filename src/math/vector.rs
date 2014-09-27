use math::Matrix;

#[cfg(test)]
#[path="../../tests/math/vector_test.rs"]
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
    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// A multiplies a Vector by a scalar.
    ///
    /// > *Will be removed once Traits support type inference.*
    #[inline(always)]
    #[experimental]
    pub fn scale(&self, s: f32) -> Vector {
        Vector::new(self[0]*s, self[1]*s, self[2]*s)
    }

    /// Computes the dot product between two vectors.
    #[inline(always)]
    pub fn dot(&self, other: &Vector) -> f32 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }

    /// Computes the cross product between two vectors.
    #[inline]
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self[1]*other[2] - self[2]*other[1],
            self[2]*other[0] - self[0]*other[2],
            self[0]*other[1] - self[1]*other[0],
        )
    }

    /// Computes the direction vector of a Vector.
    #[inline]
    pub fn normalize(&self) -> Vector {
        let l = self.length();
        Vector::new(self[0]/l, self[1]/l, self[2]/l)
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
    pub fn outer(&self, other: &Vector) -> Matrix {
        let elems: [f32, ..9] = [
            self[0]*other[0], self[0]*other[1], self[0]*other[2],
            self[1]*other[0], self[1]*other[1], self[1]*other[2],
            self[2]*other[0], self[2]*other[1], self[2]*other[2],
        ];
        Matrix::new(&elems)
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
        Vector{ elements: [self[0] + other[0], self[1] + other[1], self[2] + other[2] ] }
    }
}

/// Implement the subtraction operator between Vectors.
impl Sub<Vector, Vector> for Vector {
    /// Calculates the difference between two vectors.
    #[inline]
    fn sub(&self, other: &Vector) -> Vector {
        Vector{ elements: [ self[0] - other[0], self[1] - other[1], self[2] - other[2] ] }
    }
}
