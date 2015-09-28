use core::Float;
use maths::{ Vector, Quaternion };

/// The `Transform` object represents a spatial transformation in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    translation: Vector,
    rotation: Quaternion,
}

impl Transform {
    /// Creates a new `Transform` instance with the given translation and
    /// rotation.
    pub fn new(translation: Vector, rotation: Quaternion) -> Transform {
        Transform {
            translation: translation,
            rotation: rotation,
        }
    }

    /// Creates a new `Transform` instance with the given translation and no
    /// rotation.
    pub fn new_with_translation(x: Float, y: Float, z: Float) -> Transform {
        Transform::new(Vector::new(x, y, z), Quaternion::new_identity())
    }

    /// Creates a new `Transform` instance representing the identity
    /// transformation.
    pub fn new_identity() -> Transform {
        Transform::new(Vector::new_zero(), Quaternion::new_identity())
    }

    /// The positional translation component of the transform.
    #[inline(always)]
    pub fn translation(&self) -> Vector {
        self.translation
    }

    /// Returns a mutable reference to the translation `Vector`.
    #[inline(always)]
    pub fn translation_mut(&mut self) -> &mut Vector {
        &mut self.translation
    }

    /// The rotational component of the transform.
    #[inline(always)]
    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    /// Returns a mutable reference to the rotation `Quaternion`.
    #[inline(always)]
    pub fn rotation_mut(&mut self) -> &mut Quaternion {
        &mut self.rotation
    }

    /// Applies the transform to a point.
    pub fn apply_to_point(&self, point: Vector) -> Vector {
        point.rotate_by_quaternion(self.rotation()) + self.translation()
    }

    /// Applies the `Transform` on the `Vector` treating it as a direction.
    pub fn apply_to_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rotation())
    }

    /// Applies the inverse of the transform to a direction.
    pub fn apply_inverse_to_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rotation().inverse())
    }
}
