use math::Matrix;

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
    /// assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0))
    /// ```
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{ elements: [x, y, z] }
    }

    /// Constructs a zero vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::zero();
    ///
    /// assert_eq!((v[0], v[1], v[2]), (0.0, 0.0, 0.0))
    /// ```
    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// A multiplies a Vector by a scalar.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let v = a.scale(2.5);
    ///
    /// assert_eq!((v[0], v[1], v[2]), (2.5, 5.0, 7.5))
    /// ```
    #[inline(always)]
    pub fn scale(&self, s: f32) -> Vector {
        Vector::new(self[0]*s, self[1]*s, self[2]*s)
    }

    /// Computes the dot product between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let b = Vector::new(2.0, -2.0, 2.0);
    ///
    /// assert_eq!(a.dot(&b), 4.0)
    /// ```
    #[inline(always)]
    pub fn dot(&self, other: &Vector) -> f32 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }

    /// Computes the cross product between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 1.0);
    /// let b = Vector::new(2.0, 1.0, 2.0);
    ///
    /// let c = a.cross(&b);
    ///
    /// assert_eq!((c[0], c[1], c[2]), (3.0, 0.0, -3.0))
    /// ```
    #[inline]
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self[1]*other[2] - self[2]*other[1],
            self[2]*other[0] - self[0]*other[2],
            self[0]*other[1] - self[1]*other[0],
        )
    }

    /// Computes the direction vector of a Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(12.0, 20.0, 9.0);
    /// let n = v.normalize();
    ///
    /// assert_eq!((n[0], n[1], n[2]), (0.48, 0.80, 0.36))
    /// ```
    #[inline]
    pub fn normalize(&self) -> Vector {
        let l = self.length();
        Vector::new(self[0]/l, self[1]/l, self[2]/l)
    }

    /// Computes the squared length of a Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(1.0, -2.0, 3.0);
    ///
    /// assert_eq!(v.length_sq(), 14.0)
    /// ```
    #[inline(always)]
    pub fn length_sq(&self) -> f32 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    /// Computes the length of a Vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(2.0, 3.0, 6.0);
    ///
    /// assert_eq!(v.length(), 7.0)
    /// ```
    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Computes the outer product between two Vectors.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let b = Vector::new(4.0, 5.0, 6.0);
    ///
    /// let m = a.outer(&b);
    ///
    /// assert_eq!((m[0], m[1], m[2]), ( 4.0,  5.0,  6.0))
    /// assert_eq!((m[3], m[4], m[5]), ( 8.0, 10.0, 12.0))
    /// assert_eq!((m[6], m[7], m[8]), (12.0, 15.0, 18.0))
    /// ```
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
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0))
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
    /// assert_eq!((v[0], v[1], v[2]), (3.0, 2.0, 3.0))
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
    /// assert_eq!((b[0], b[1], b[2]), (-1.0, -3.0, -9.0))
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
    /// assert_eq!((c[0], c[1], c[2]), (3.0, 4.0, 0.0))
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
    /// assert_eq!((c[0], c[1], c[2]), (0.0, -2.0, 7.0))
    /// ```
    #[inline]
    fn sub(&self, other: &Vector) -> Vector {
        Vector{ elements: [ self[0] - other[0], self[1] - other[1], self[2] - other[2] ] }
    }
}
