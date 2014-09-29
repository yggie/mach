use math::{ Vector, Matrix };

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

    /// Creates a pure rotational transformation matrix.
    pub fn new_rotation(rotation: Matrix) -> Transform {
        Transform{ rotation: rotation, translation: Vector::new_zero() }
    }

    /// Creates a pure translational transformation matrix.
    pub fn new_translation(translation: Vector) -> Transform {
        Transform{ rotation: Matrix::new_identity(), translation: translation }
    }

    /// Constructs an identity Transform.
    pub fn new_identity() -> Transform {
        Transform::new(Matrix::new_identity(), Vector::new_zero())
    }

    /// Returns the rotational component of the transformation matrix as a Matrix.
    pub fn rotation_matrix(&self) -> Matrix {
        self.rotation
    }

    /// Returns the translational component of the transformation matrix as a Vector.
    pub fn translation_vector(&self) -> Vector {
        self.translation
    }
}

