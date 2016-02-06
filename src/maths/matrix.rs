#[cfg(test)]
#[path="../../tests/maths/matrix_test.rs"]
mod matrix_test;

use std::fmt;
use std::ops::{ Add, Div, Index, IndexMut, Mul, Neg, Sub };

use { Scalar, TOLERANCE };
use maths::Vect;

/// A representation of a 3-by-3 matrix
#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    elements: [Scalar; 9]
}

impl Matrix {
    /// Constructs a new matrix given 9 elements in row major order.
    #[inline(always)]
    pub fn new(m11: Scalar, m12: Scalar, m13: Scalar, m21: Scalar, m22: Scalar, m23: Scalar, m31: Scalar, m32: Scalar, m33: Scalar) -> Matrix {
        Matrix {
            elements: [
                m11,
                m12,
                m13,
                m21,
                m22,
                m23,
                m31,
                m32,
                m33,
            ]
        }
    }

    /// Constructs a new matrix from a slice given 9 elements in row major
    /// order.
    pub fn from_slice(elements: &[Scalar]) -> Matrix {
        Matrix {
            elements: [
                elements[0],
                elements[1],
                elements[2],
                elements[3],
                elements[4],
                elements[5],
                elements[6],
                elements[7],
                elements[8],
            ]
        }
    }

    /// Constructs an identity matrix.
    pub fn identity() -> Matrix {
        Matrix::diag(1.0, 1.0, 1.0)
    }

    /// Constructs a new matrix given 3 elements in the matrix diagonal.
    #[inline(always)]
    pub fn diag(x: Scalar, y: Scalar, z: Scalar) -> Matrix {
        Matrix{ elements: [
              x, 0.0, 0.0,
            0.0,   y, 0.0,
            0.0, 0.0,   z
        ] }
    }

    /// Constructs a skew matrix based on the input vector.
    #[inline(always)]
    pub fn skew(x: Scalar, y: Scalar, z: Scalar) -> Matrix {
        Matrix{ elements: [
            0.0,  -z,   y,
              z, 0.0,  -x,
             -y,   x, 0.0
        ] }
    }

    /// Computes the orientation matrix given the axis of rotation and angle
    /// of rotation measured in radians.
    pub fn rotation(radians: Scalar, axis: Vect) -> Matrix {
        let c = radians.cos();
        let s = radians.sin();
        let a = axis.normalize();
        let c1 = 1.0 - c;
        let aa = Vect::new(a.x*c1, a.y*c1, a.z*c1);
        Matrix::diag(c, c, c) + a.outer(aa) + Matrix::skew(a.x*s, a.y*s, a.z*s)
    }

    /// Returns an element from the matrix, given the row and column numbers.
    #[inline(always)]
    pub fn get(&self, row: usize, col: usize) -> Scalar {
        self.elements[3*col + row]
    }

    /// Computes the determinant of the `Matrix`.
    pub fn determinant(&self) -> Scalar {
        return self[0]*(self[4]*self[8] - self[5]*self[7]) -
               self[1]*(self[3]*self[8] - self[5]*self[6]) +
               self[2]*(self[3]*self[7] - self[4]*self[6]);
    }

    /// Computes the inverse of the `Matrix`.
    pub fn inverse(&self) -> Matrix {
        return Matrix::new(
            self[4]*self[8] - self[5]*self[7],
            self[2]*self[7] - self[1]*self[8],
            self[1]*self[5] - self[2]*self[4],
            self[5]*self[6] - self[3]*self[8],
            self[0]*self[8] - self[2]*self[6],
            self[2]*self[3] - self[0]*self[5],
            self[3]*self[7] - self[4]*self[6],
            self[1]*self[6] - self[0]*self[7],
            self[0]*self[4] - self[1]*self[3],
        ) / self.determinant();
    }
}

/// Implements the `std::fmt` operations to allow using `println!` on `Matrix`
/// instances.
impl fmt::Display for Matrix {
    /// Implements the fmt operation for `Matrix` instances. The resulting
    /// format is equivalent to:
    ///
    /// ```rust,ignore
    /// println!("[{}, {}, {}, {}, {}, {}, {}, {}, {}]", m[0], m[1], m[2], m[3],
    ///          m[4], m[5], m[6], m[7], m[8]);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}, {}, {}, {}, {}, {}]",
               self[0], self[1], self[2], self[3], self[4],
               self[5], self[6], self[7], self[8])
    }
}

/// Guarantees that equality satisfies the equivalence relations.
impl Eq for Matrix { }

/// Implementation for the equality operations, allows the use of `==` and `!=`
/// operators on `Matrix` instances.
impl PartialEq for Matrix {
    /// Implements the equality operator for `Matrix` instances. Returns true if
    /// the maximum difference in the `Matrix` components is less than the
    /// allowed tolerance.
    fn eq(&self, other: &Matrix) -> bool {
        for i in 0..9 {
            if (self[i] - other[i]).abs() > TOLERANCE {
                return false;
            }
        }

        return true;
    }
}

/// Implement the index operator.
impl Index<usize> for Matrix {
    type Output = Scalar;

    /// Obtain the elements in the Matrix in column major order.
    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a Scalar {
        &self.elements[index]
    }
}

/// Implement the mutable index operator.
impl IndexMut<usize> for Matrix {
    /// Obtains a mutable reference to the element in the Matrix in column
    /// major order.
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Scalar {
        &mut self.elements[index]
    }
}

/// Implement the unary negation operator.
impl Neg for Matrix {
    type Output = Matrix;

    /// Applies the negation operator to each element in the matrix.
    fn neg(self) -> Matrix {
        let elems: [Scalar; 9] = [
            -self[0], -self[1], -self[2],
            -self[3], -self[4], -self[5],
            -self[6], -self[7], -self[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the addition operator between Matrices.
impl Add<Matrix> for Matrix {
    type Output = Matrix;

    /// Calculates the sum of two matrices.
    fn add(self, other: Matrix) -> Matrix {
        let elems: [Scalar; 9] = [
            self[0] + other[0], self[1] + other[1], self[2] + other[2],
            self[3] + other[3], self[4] + other[4], self[5] + other[5],
            self[6] + other[6], self[7] + other[7], self[8] + other[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the subtraction operator between Matrices.
impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    /// Calculates the difference between two vectors.
    fn sub(self, other: Matrix) -> Matrix {
        let elems: [Scalar; 9] = [
            self[0] - other[0], self[1] - other[1], self[2] - other[2],
            self[3] - other[3], self[4] - other[4], self[5] - other[5],
            self[6] - other[6], self[7] - other[7], self[8] - other[8],
        ];
        Matrix{ elements: elems }
    }
}

/// Implement the multiplication operator between a `Matrix` and a `Vect`.
impl Div<Scalar> for Matrix {
    type Output = Matrix;

    /// Divides all elements of the `Matrix` by the input and returns the
    /// resulting `Matrix`.
    fn div(self, scalar: Scalar) -> Matrix {
        return Matrix::new(
            self[0]/scalar,
            self[1]/scalar,
            self[2]/scalar,
            self[3]/scalar,
            self[4]/scalar,
            self[5]/scalar,
            self[6]/scalar,
            self[7]/scalar,
            self[8]/scalar,
        );
    }
}

/// Implement the multiplication operator between a `Matrix` and a `Vect`.
impl Mul<Scalar> for Matrix {
    type Output = Matrix;

    /// Multiplies all elements of the `Matrix` by the input and returns the
    /// resulting `Matrix`.
    fn mul(self, scalar: Scalar) -> Matrix {
        return Matrix::new(
            self[0]*scalar,
            self[1]*scalar,
            self[2]*scalar,
            self[3]*scalar,
            self[4]*scalar,
            self[5]*scalar,
            self[6]*scalar,
            self[7]*scalar,
            self[8]*scalar,
        );
    }
}

/// Implement the multiplication operator between a `Matrix` and a `Vect`.
impl<'a> Mul<&'a Vect> for Matrix {
    type Output = Vect;

    /// Computes the resulting vector from the multiplication between a matrix
    /// and a vector.
    fn mul(self, vect: &'a Vect) -> Vect {
        Vect::new(
            self[0]*vect.x + self[3]*vect.y + self[6]*vect.z,
            self[1]*vect.x + self[4]*vect.y + self[7]*vect.z,
            self[2]*vect.x + self[5]*vect.y + self[8]*vect.z,
        )
    }
}

/// Implement the multiplication operator between a `Matrix` and a `Vect`.
impl Mul<Vect> for Matrix {
    type Output = Vect;

    /// Computes the resulting vector from the multiplication between a matrix
    /// and a vector.
    fn mul(self, vect: Vect) -> Vect {
        self * &vect
    }
}

/// Implement the multiplication operator between Matrices.
impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    /// Calculates the result of applying matrix multiplication between two
    /// matrices.
    fn mul(self, other: Matrix) -> Matrix {
        let elems: [Scalar; 9] = [
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

        return Matrix { elements: elems };
    }
}
