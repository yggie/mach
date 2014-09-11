use math::Vector;
use math::Matrix;

impl Mul<Vector, Vector> for Matrix {
    /// Computes the resulting vector from the multiplication between a matrix
    /// and a vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// # use mithril::math::Matrix;
    /// let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let m = Matrix::new(&elems);
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// let a = m * v;
    ///
    /// assert!((a[0], a[1], a[2]) == (30.0, 36.0, 42.0))
    /// ```
    fn mul(&self, vect: &Vector) -> Vector {
        Vector::new(
            self[0]*vect[0] + self[3]*vect[1] + self[6]*vect[2],
            self[1]*vect[0] + self[4]*vect[1] + self[7]*vect[2],
            self[2]*vect[0] + self[5]*vect[1] + self[8]*vect[2],
        )
    }
}
