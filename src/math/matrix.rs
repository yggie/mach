/// A representation of a 3-by-3 matrix
pub struct Matrix {
    elements: [f32, ..9]
}

impl Matrix {

    /// Constructs a new matrix given 9 elements in column major order.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let m = Matrix::new(&elems);
    ///
    /// assert!((m[0], m[3], m[6]) == (1.0, 4.0, 7.0))
    /// assert!((m[1], m[4], m[7]) == (2.0, 5.0, 8.0))
    /// assert!((m[2], m[5], m[8]) == (3.0, 6.0, 9.0))
    /// ```
    pub fn new(elements: &[f32, ..9]) -> Matrix {
        Matrix{ elements: *elements }
    }

    /// Constructs a new matrix given 3 elements in the matrix diagonal.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let m = Matrix::diag(1.0, 2.0, 3.0);
    ///
    /// assert!((m[0], m[3], m[6]) == (1.0, 0.0, 0.0))
    /// assert!((m[1], m[4], m[7]) == (0.0, 2.0, 0.0))
    /// assert!((m[2], m[5], m[8]) == (0.0, 0.0, 3.0))
    /// ```
    pub fn diag(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{ elements: [
              x, 0.0, 0.0,
            0.0,   y, 0.0,
            0.0, 0.0,   z
        ] }
    }

    /// Takes an element from the matrix
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let m = Matrix::diag(4.0, 5.0, 2.0);
    ///
    /// assert!((m.get(0, 0), m.get(0, 1), m.get(0, 2)) == (4.0, 0.0, 0.0))
    /// assert!((m.get(1, 0), m.get(1, 1), m.get(1, 2)) == (0.0, 5.0, 0.0))
    /// assert!((m.get(2, 0), m.get(2, 1), m.get(2, 2)) == (0.0, 0.0, 2.0))
    /// ```
    #[inline(always)]
    pub fn get(&self, row: uint, col: uint) -> f32 {
        self.elements[3*col + row]
    }
}

/// Implement the index operator.
impl Index<uint, f32> for Matrix {
    /// Obtain the elements in the Matrix in column major order.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let m = Matrix::new(&elems);
    ///
    /// assert!((m[0], m[3], m[6]) == (1.0, 4.0, 7.0))
    /// assert!((m[1], m[4], m[7]) == (2.0, 5.0, 8.0))
    /// assert!((m[2], m[5], m[8]) == (3.0, 6.0, 9.0))
    /// ```
    #[inline(always)]
    fn index<'a>(&'a self, index: &uint) -> &'a f32 {
        &self.elements[*index]
    }
}

/// Implement the mutable index operator.
impl IndexMut<uint, f32> for Matrix {
    /// Obtains a mutable reference to the element in the Matrix in column
    /// major order.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let mut m = Matrix::new(&elems);
    ///
    /// m[0] = 11.0;
    /// m[4] = 12.0;
    /// m[8] = 13.0;
    ///
    /// assert!((m[0], m[3], m[6]) == (11.0, 4.0, 7.0))
    /// assert!((m[1], m[4], m[7]) == (2.0, 12.0, 8.0))
    /// assert!((m[2], m[5], m[8]) == (3.0, 6.0, 13.0))
    /// ```
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f32 {
        &mut self.elements[*index]
    }
}

/// Implement the unary negation operator.
impl Neg<Matrix> for Matrix {
    /// Applies the negation operator to each element in the matrix.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let m = -Matrix::new(&elems);
    ///
    /// assert!((m[0], m[3], m[6]) == (-1.0, -4.0, -7.0))
    /// assert!((m[1], m[4], m[7]) == (-2.0, -5.0, -8.0))
    /// assert!((m[2], m[5], m[8]) == (-3.0, -6.0, -9.0))
    /// ```
    fn neg(&self) -> Matrix {
        let elems: [f32, ..9] = [
            -self[0], -self[1], -self[2],
            -self[3], -self[4], -self[5],
            -self[6], -self[7], -self[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the addition operator between Matrices.
impl Add<Matrix, Matrix> for Matrix {
    /// Calculates the sum of two matrices.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let a = Matrix::new(&elems);
    /// let b = Matrix::diag(3.0, 2.0, 1.0);
    ///
    /// let m = a + b;
    ///
    /// assert!((m[0], m[3], m[6]) == (4.0, 4.0, 7.0))
    /// assert!((m[1], m[4], m[7]) == (2.0, 7.0, 8.0))
    /// assert!((m[2], m[5], m[8]) == (3.0, 6.0, 10.0))
    /// ```
    fn add(&self, other: &Matrix) -> Matrix {
        let elems: [f32, ..9] = [
            self[0] + other[0], self[1] + other[1], self[2] + other[2],
            self[3] + other[3], self[4] + other[4], self[5] + other[5],
            self[6] + other[6], self[7] + other[7], self[8] + other[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the subtraction operator between Matrices.
impl Sub<Matrix, Matrix> for Matrix {
    /// Calculates the difference between two vectors.
    ///
    /// ```rust
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let a = Matrix::new(&elems);
    /// let b = Matrix::diag(1.0, 2.0, 3.0);
    ///
    /// let m = a - b;
    ///
    /// assert!((m[0], m[3], m[6]) == (0.0, 4.0, 7.0))
    /// assert!((m[1], m[4], m[7]) == (2.0, 3.0, 8.0))
    /// assert!((m[2], m[5], m[8]) == (3.0, 6.0, 6.0))
    /// ```
    fn sub(&self, other: &Matrix) -> Matrix {
        let elems: [f32, ..9] = [
            self[0] - other[0], self[1] - other[1], self[2] - other[2],
            self[3] - other[3], self[4] - other[4], self[5] - other[5],
            self[6] - other[6], self[7] - other[7], self[8] - other[8],
        ];
        Matrix{ elements: elems }
    }
}
//
// /// Implement the multiplication operator between Matrices.
// impl Mul<Matrix, Matrix> for Matrix {
//     /// Calculates the result of applying matrix multiplication between two
//     /// matrices.
//     ///
//     /// ```rust
//     /// # use mithril::math::Matrix;
//     /// let elems_a: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
//     /// let a = Matrix::new(&elems_a);
//     /// let elems_b: [f32, ..9] = [3.0, 2.0, 1.0, 6.0, 5.0, 4.0, 9.0, 8.0, 7.0];
//     /// let b = Matrix::new(&elems_b);
//     ///
//     /// let m = a * b;
//     ///
//     /// assert!((m[0], m[3], m[6]) == (18.0, 54.0, 90.0))
//     /// assert!((m[1], m[4], m[7]) == (24.0, 69.0, 114.0))
//     /// assert!((m[2], m[5], m[8]) == (30.0, 84.0, 138.0))
//     /// ```
//     fn mul(&self, other: &Matrix) -> Matrix {
//         let elems: [f32, ..9] = [
//             self[0]*other[0] + self[3]*other[1] + self[6]*other[2],
//             self[1]*other[0] + self[4]*other[1] + self[7]*other[2],
//             self[2]*other[0] + self[5]*other[1] + self[8]*other[2],
//             self[0]*other[3] + self[3]*other[4] + self[6]*other[5],
//             self[1]*other[3] + self[4]*other[4] + self[7]*other[5],
//             self[2]*other[3] + self[5]*other[4] + self[8]*other[5],
//             self[0]*other[6] + self[3]*other[7] + self[6]*other[8],
//             self[1]*other[6] + self[4]*other[7] + self[7]*other[8],
//             self[2]*other[6] + self[5]*other[7] + self[8]*other[8],
//         ];
//         Matrix{ elements: elems }
//     }
// }
