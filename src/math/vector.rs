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
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert!((v[0], v[1], v[2]) == (1.0, 2.0, 3.0))
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{ elements: [x, y, z] }
    }

    /// Computes the dot product between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let b = Vector::new(2.0, -2.0, 2.0);
    ///
    /// assert!(Vector::dot(&a, &b) == 4.0)
    /// ```
    #[inline(always)]
    pub fn dot(a: &Vector, b: &Vector) -> f32 {
        a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
    }

    /// Computes the cross product between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 1.0);
    /// let b = Vector::new(2.0, 1.0, 2.0);
    ///
    /// let c = Vector::cross(&a, &b);
    ///
    /// assert!((c[0], c[1], c[2]) == (3.0, 0.0, -3.0))
    /// ```
    #[inline]
    pub fn cross(a: &Vector, b: &Vector) -> Vector {
        Vector{ elements: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
    }

    /// Computes the direction vector of a Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(12.0, 20.0, 9.0);
    /// let n = Vector::normalize(&v);
    ///
    /// assert!((n[0], n[1], n[2]) == (0.48, 0.80, 0.36))
    /// ```
    #[inline]
    pub fn normalize(v: &Vector) -> Vector {
        let l = v.length();
        Vector{ elements: [ v[0]/l, v[1]/l, v[2]/l ] }
    }

    /// Computes the squared length of the Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(1.0, -2.0, 3.0);
    ///
    /// assert!(v.length_sq() == 14.0)
    /// ```
    #[inline(always)]
    pub fn length_sq(&self) -> f32 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    /// Computes the length of the Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(2.0, 3.0, 6.0);
    ///
    /// assert!(v.length() == 7.0)
    /// ```
    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
}

/// Implement the index operator.
impl Index<uint, f32> for Vector {
    /// Obtain the vector's elements by index. Uses zero-based indexing.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert!((v[0], v[1], v[2]) == (1.0, 2.0, 3.0))
    /// ```
    #[inline(always)]
    fn index<'a>(&'a self, index: &uint) -> &'a f32 {
        &self.elements[*index]
    }
}

/// Implement the mutable index operator.
impl IndexMut<uint, f32> for Vector {
    /// Allows setting a vector's element using index notation.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let mut v = Vector::new(1.0, 2.0, 3.0);
    /// v[0] = 3.0;
    ///
    /// assert!((v[0], v[1], v[2]) == (3.0, 2.0, 3.0))
    /// ```
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f32 {
        &mut self.elements[*index]
    }
}

/// Implement the unary negation operator.
impl Neg<Vector> for Vector {
    /// Reverses the direction of the vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 3.0, 9.0);
    /// let b = -a;
    ///
    /// assert!((b[0], b[1], b[2]) == (-1.0, -3.0, -9.0))
    /// ```
    #[inline]
    fn neg(&self) -> Vector {
        Vector{ elements: [ -self[0], -self[1], -self[2] ] }
    }
}

/// Implement the addition operator between Vectors.
impl Add<Vector, Vector> for Vector {
    /// Calculates the sum of two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 3.0, -1.0);
    /// let b = Vector::new(2.0, 1.0, 1.0);
    ///
    /// let c = a + b;
    ///
    /// assert!((c[0], c[1], c[2]) == (3.0, 4.0, 0.0))
    /// ```
    #[inline]
    fn add(&self, other: &Vector) -> Vector {
        Vector{ elements: [self[0] + other[0], self[1] + other[1], self[2] + other[2] ] }
    }
}

/// Implement the subtraction operator between Vectors.
impl Sub<Vector, Vector> for Vector {
    /// Calculates the difference between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, -1.0, 3.5);
    /// let b = Vector::new(1.0, 1.0, -3.5);
    ///
    /// let c = a - b;
    ///
    /// assert!((c[0], c[1], c[2]) == (0.0, -2.0, 7.0))
    /// ```
    #[inline]
    fn sub(&self, other: &Vector) -> Vector {
        Vector{ elements: [ self[0] - other[0], self[1] - other[1], self[2] - other[2] ] }
    }
}
