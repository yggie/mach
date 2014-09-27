use math::Vector;
use math::Matrix;

#[cfg(test)]
#[path="../../tests/math/transform_test.rs"]
mod tests;

/// Represents a rigid-body transformation in space.
pub struct Transform {
    rotation: Matrix,
    translation: Vector,
}

impl Transform {

    /// Creates a new Transform given the rotation matrix and translation vector.
    pub fn new(rotation: Matrix, translation: Vector) -> Transform {
        Transform{ rotation: rotation, translation: translation }
    }

    /// Constructs an identity Transform.
    pub fn identity() -> Transform {
        Transform::new(Matrix::identity(), Vector::zero())
    }

    /// Returns the current orientation expressed as a matrix.
    pub fn rotation(&self) -> Matrix {
        self.rotation
    }

    /// Returns the current orientation expressed as a matrix.
    pub fn translation(&self) -> Vector {
        self.translation
    }
}

