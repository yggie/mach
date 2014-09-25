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
    /// let r = Matrix::rotation(2.5, &Vector::new(1.0, 0.0, 0.0));
    ///
    /// let transform = Transform::new(r, t);
    /// ```
    pub fn new(rotation: Matrix, translation: Vector) -> Transform {
        Transform{ rotation: rotation, translation: translation }
    }

    /// Constructs an identity Transform.
    ///
    /// ```rust
    /// # use mithril::math::Transform;
    /// let t = Transform::identity();
    /// let m = t.rotation();
    /// let v = t.translation();
    ///
    /// assert_eq!((m[0], m[1], m[2], v[0]), (1.0, 0.0, 0.0, 0.0))
    /// assert_eq!((m[3], m[4], m[5], v[1]), (0.0, 1.0, 0.0, 0.0))
    /// assert_eq!((m[6], m[7], m[8], v[2]), (0.0, 0.0, 1.0, 0.0))
    /// ```
    pub fn identity() -> Transform {
        Transform::new(Matrix::identity(), Vector::zero())
    }

    /// Returns the current orientation expressed as a matrix.
    ///
    /// ```rust
    /// # use mithril::math::Vector;
    /// # use mithril::math::Matrix;
    /// # use mithril::math::Transform;
    /// let t = Vector::new(1.0, 2.0, 3.0);
    /// let r = Matrix::rotation(2.5, &Vector::new(1.0, 0.0, 0.0));
    /// let transform = Transform::new(r, t);
    ///
    /// let m = transform.rotation();
    ///
    /// assert_eq!((m[0], m[1], m[2]), (r[0], r[1], r[2]))
    /// assert_eq!((m[3], m[4], m[5]), (r[3], r[4], r[5]))
    /// assert_eq!((m[6], m[7], m[8]), (r[6], r[7], r[8]))
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
    /// let r = Matrix::rotation(1.5, &Vector::new(1.0, 1.0, 0.0));
    /// let transform = Transform::new(r, t);
    ///
    /// let v = transform.translation();
    ///
    /// assert_eq!((v[0], v[1], v[2]), (t[0], t[1], t[2]))
    /// ```
    pub fn translation(&self) -> Vector {
        self.translation
    }
}

