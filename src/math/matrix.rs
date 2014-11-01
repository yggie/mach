use math::Vector;

#[cfg(test)]
#[path="../../tests/unit/math/matrix_test.rs"]
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
    pub fn new_identity() -> Matrix {
        Matrix::new_diag(1.0, 1.0, 1.0)
    }

    /// Constructs a new matrix given 3 elements in the matrix diagonal.
    #[inline(always)]
    pub fn new_diag(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{ elements: [
              x, 0.0, 0.0,
            0.0,   y, 0.0,
            0.0, 0.0,   z
        ] }
    }

    /// Constructs a skew matrix based on the input vector.
    #[inline(always)]
    pub fn new_skew(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{ elements: [
            0.0,  -z,   y,
              z, 0.0,  -x,
             -y,   x, 0.0
        ] }
    }

    /// Computes the orientation matrix given the axis of rotation and angle
    /// of rotation measured in radians.
    pub fn new_rotation(radians: f32, axis: Vector) -> Matrix {
        let c = radians.cos();
        let s = radians.sin();
        let a = axis.normalize();
        let c1 = 1.0 - c;
        let aa = Vector::new(a[0]*c1, a[1]*c1, a[2]*c1);
        Matrix::new_diag(c, c, c) + a.outer(aa) + Matrix::new_skew(a[0]*s, a[1]*s, a[2]*s)
    }

    /// Returns an element from the matrix, given the row and column numbers.
    #[inline(always)]
    pub fn get(&self, row: uint, col: uint) -> f32 {
        self.elements[3*col + row]
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

/// Implement the multiplication operator between Matrices.
impl Mul<Matrix, Matrix> for Matrix {
    /// Calculates the result of applying matrix multiplication between two
    /// matrices.
    fn mul(&self, other: &Matrix) -> Matrix {
        let elems: [f32, ..9] = [
            self[0]*other[0] + self[1]*other[3] + self[2]*other[6],
            self[0]*other[1] + self[1]*other[4] + self[2]*other[7],
            self[0]*other[2] + self[1]*other[5] + self[2]*other[8],
            self[3]*other[0] + self[4]*other[3] + self[5]*other[6],
            self[3]*other[1] + self[4]*other[4] + self[5]*other[7],
            self[3]*other[2] + self[4]*other[5] + self[5]*other[8],
            self[6]*other[0] + self[7]*other[3] + self[8]*other[6],
            self[6]*other[1] + self[7]*other[4] + self[8]*other[7],
            self[6]*other[2] + self[7]*other[5] + self[8]*other[8],
        ];
        Matrix{ elements: elems }
    }
}

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
