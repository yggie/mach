use math::Vector;
use math::Matrix;

/// Represents a rigid-body transformation in space.
pub struct Transform {
    rotation: Matrix,
    translation: Vector,
}

impl Transform {

    /// Creates a new Transform given the rotation matrix and translation vector.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// # use mithril::math::Matrix;
    /// # use mithril::math::Transform;
    /// let t = Vector::new(1.0, 2.0, 3.0);
    /// let r = Matrix::diag(1.0, 2.0, 3.0);
    ///
    /// let transform = Transform::new(r, t);
    /// ```
    pub fn new(rotation: Matrix, translation: Vector) -> Transform {
        Transform{ rotation: rotation, translation: translation }
    }

    /// Returns the current orientation expressed as a matrix.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// # use mithril::math::Matrix;
    /// # use mithril::math::Transform;
    /// let t = Vector::new(1.0, 2.0, 3.0);
    /// let r = Matrix::diag(1.0, 2.0, 3.0);
    /// let transform = Transform::new(r, t);
    ///
    /// let r2 = transform.rotation();
    /// ```
    pub fn rotation(&self) -> Matrix {
        self.rotation
    }

    /// Returns the current orientation expressed as a matrix.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// # use mithril::math::Matrix;
    /// # use mithril::math::Transform;
    /// let t = Vector::new(1.0, 2.0, 3.0);
    /// let r = Matrix::diag(1.0, 2.0, 3.0);
    /// let transform = Transform::new(r, t);
    ///
    /// let v = transform.translation();
    ///
    /// assert!((v[0], v[1], v[2]) == (1.0, 2.0, 3.0))
    /// ```
    pub fn translation(&self) -> Vector {
        self.translation
    }
}

