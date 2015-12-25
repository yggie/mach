use Scalar;
use maths::{Vect, Quat};

/// The `Transform` object represents a spatial transformation in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    translation: Vect,
    rotation: Quat,
}

impl Transform {
    /// Creates a new `Transform` instance with the given translation and
    /// rotation.
    pub fn new(translation: Vect, rotation: Quat) -> Transform {
        Transform {
            translation: translation,
            rotation: rotation,
        }
    }

    /// Creates a new `Transform` instance with the given translation and no
    /// rotation.
    pub fn new_with_translation(x: Scalar, y: Scalar, z: Scalar) -> Transform {
        Transform::new(Vect::new(x, y, z), Quat::new_identity())
    }

    /// Creates a new `Transform` instance representing the identity
    /// transformation.
    pub fn new_identity() -> Transform {
        Transform::new(Vect::new_zero(), Quat::new_identity())
    }

    /// The positional translation component of the transform.
    #[inline(always)]
    pub fn translation(&self) -> Vect {
        self.translation
    }

    /// Returns a mutable reference to the translation `Vect`.
    #[inline(always)]
    pub fn translation_mut(&mut self) -> &mut Vect {
        &mut self.translation
    }

    /// The rotational component of the transform.
    #[inline(always)]
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Returns a mutable reference to the rotation `Quat`.
    #[inline(always)]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.rotation
    }

    /// Applies the transform to a point.
    pub fn apply_to_point(&self, point: Vect) -> Vect {
        point.rotate_by_quaternion(self.rotation()) + self.translation()
    }

    /// Applies the `Transform` on the `Vect` treating it as a direction.
    pub fn apply_to_direction(&self, direction: Vect) -> Vect {
        direction.rotate_by_quaternion(self.rotation())
    }

    /// Applies the inverse of the transform to a direction.
    pub fn apply_inverse_to_direction(&self, direction: Vect) -> Vect {
        direction.rotate_by_quaternion(self.rotation().inverse())
    }
}
