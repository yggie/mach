use math::Vector;

#[cfg(test)]
#[path="../../tests/math/matrix_test.rs"]
mod tests;

/// A representation of a 3-by-3 matrix
pub struct Matrix {
    elements: [f32, ..9]
}

impl Matrix {

    /// Constructs a new matrix given 9 elements in row major order.
    #[inline(always)]
    pub fn new(elements: &[f32, ..9]) -> Matrix {
        Matrix{ elements: *elements }
    }

    /// Constructs an identity matrix.
    pub fn identity() -> Matrix {
        Matrix::diag(1.0, 1.0, 1.0)
    }

    /// Constructs a new matrix given 3 elements in the matrix diagonal.
    #[inline(always)]
    pub fn diag(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{ elements: [
              x, 0.0, 0.0,
            0.0,   y, 0.0,
            0.0, 0.0,   z
        ] }
    }

    /// Constructs a skew matrix based on the input vector.
    #[inline(always)]
    pub fn skew(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{ elements: [
            0.0,  -z,   y,
              z, 0.0,  -x,
             -y,   x, 0.0
        ] }
    }

    /// Computes the orientation matrix given the axis of rotation and angle
    /// of rotation measured in radians.
    pub fn rotation(radians: f32, axis: &Vector) -> Matrix {
        let c = radians.cos();
        let s = radians.sin();
        let a = axis.normalize();
        let c1 = 1.0 - c;
        let aa = Vector::new(a[0]*c1, a[1]*c1, a[2]*c1);
        Matrix::diag(c, c, c) + a.outer(&aa) + Matrix::skew(a[0]*s, a[1]*s, a[2]*s)
    }

    /// Returns an element from the matrix, given the row and column numbers.
    #[inline(always)]
    pub fn get(&self, row: uint, col: uint) -> f32 {
        self.elements[3*col + row]
    }

    /// Calculates the result of applying matrix multiplication between two
    /// matrices. (TEMPORARY UNTIL ASSOCIATED TYPES ARE SUPPORTED IN RUST)
    #[experimental]
    pub fn mult(a: &Matrix, b: &Matrix) -> Matrix {
        let elems: [f32, ..9] = [
            a[0]*b[0] + a[1]*b[3] + a[2]*b[6],
            a[0]*b[1] + a[1]*b[4] + a[2]*b[7],
            a[0]*b[2] + a[1]*b[5] + a[2]*b[8],
            a[3]*b[0] + a[4]*b[3] + a[5]*b[6],
            a[3]*b[1] + a[4]*b[4] + a[5]*b[7],
            a[3]*b[2] + a[4]*b[5] + a[5]*b[8],
            a[6]*b[0] + a[7]*b[3] + a[8]*b[6],
            a[6]*b[1] + a[7]*b[4] + a[8]*b[7],
            a[6]*b[2] + a[7]*b[5] + a[8]*b[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the index operator.
impl Index<uint, f32> for Matrix {
    /// Obtain the elements in the Matrix in column major order.
    #[inline(always)]
    fn index<'a>(&'a self, index: &uint) -> &'a f32 {
        &self.elements[*index]
    }
}

/// Implement the mutable index operator.
impl IndexMut<uint, f32> for Matrix {
    /// Obtains a mutable reference to the element in the Matrix in column
    /// major order.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f32 {
        &mut self.elements[*index]
    }
}

/// Implement the unary negation operator.
impl Neg<Matrix> for Matrix {
    /// Applies the negation operator to each element in the matrix.
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
    fn sub(&self, other: &Matrix) -> Matrix {
        let elems: [f32, ..9] = [
            self[0] - other[0], self[1] - other[1], self[2] - other[2],
            self[3] - other[3], self[4] - other[4], self[5] - other[5],
            self[6] - other[6], self[7] - other[7], self[8] - other[8],
        ];
        Matrix{ elements: elems }
    }
}

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
//     /// assert_eq!((m[0], m[1], m[2]), ( 42.0,  36.0,  30.0))
//     /// assert_eq!((m[3], m[4], m[5]), ( 96.0,  81.0,  66.0))
//     /// assert_eq!((m[6], m[7], m[8]), (150.0, 126.0, 102.0))
//     /// ```
//     fn mul(&self, other: &Matrix) -> Matrix {
//         let elems: [f32, ..9] = [
//             self[0]*other[0] + self[1]*other[3] + self[2]*other[6],
//             self[0]*other[1] + self[1]*other[4] + self[2]*other[7],
//             self[0]*other[2] + self[1]*other[5] + self[2]*other[8],
//             self[3]*other[0] + self[4]*other[3] + self[5]*other[6],
//             self[3]*other[1] + self[4]*other[4] + self[5]*other[7],
//             self[3]*other[2] + self[4]*other[5] + self[5]*other[8],
//             self[6]*other[0] + self[7]*other[3] + self[8]*other[6],
//             self[6]*other[1] + self[7]*other[4] + self[8]*other[7],
//             self[6]*other[2] + self[7]*other[5] + self[8]*other[8],
//         ];
//         Matrix{ elements: elems }
//     }
// }

impl Mul<Vector, Vector> for Matrix {
    /// Computes the resulting vector from the multiplication between a matrix
    /// and a vector.
    fn mul(&self, vect: &Vector) -> Vector {
        Vector::new(
            self[0]*vect[0] + self[3]*vect[1] + self[6]*vect[2],
            self[1]*vect[0] + self[4]*vect[1] + self[7]*vect[2],
            self[2]*vect[0] + self[5]*vect[1] + self[8]*vect[2],
        )
    }
}
