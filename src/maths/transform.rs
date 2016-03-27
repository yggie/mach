#[cfg(test)]
#[path="../../tests/maths/transform_test.rs"]
mod transform_test;

use Scalar;
use maths::{Quat, Vect};

/// The `Transform` object represents a spatial transformation in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vect,
    pub rotation: Quat,
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

    /// Creates a new `Transform` instance representing the identity
    /// transformation.
    pub fn identity() -> Transform {
        Transform::new(Vect::zero(), Quat::identity())
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

    #[inline]
    pub fn with_translation_vect(self, translation: Vect) -> Transform {
        Transform {
            translation: translation,
            .. self
        }
    }

    #[inline]
    pub fn with_translation(self, x: Scalar, y: Scalar, z: Scalar) -> Transform {
        self.with_translation_vect(Vect::new(x, y, z))
    }

    #[inline]
    pub fn with_zero_translation(self) -> Transform {
        self.with_translation_vect(Vect::zero())
    }

    #[inline]
    pub fn with_rotation(self, rotation: Quat) -> Transform {
        Transform {
            rotation: rotation,
            .. self
        }
    }

    #[inline]
    pub fn with_axis_angle(self, axis: Vect, angle: Scalar) -> Transform {
        self.with_rotation(Quat::from_axis_angle(axis, angle))
    }

    #[inline]
    pub fn with_zero_rotation(self) -> Transform {
        self.with_rotation(Quat::identity())
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

#[macro_export]
macro_rules! include_transform_helpers {
    (struct_signature: $S:ty, struct_name: $s:ident, field_name: $field_name:ident,) => {
        #[inline]
        pub fn transform(&self) -> &Transform {
            &self.$field_name
        }

        #[inline]
        pub fn transform_mut(&mut self) -> &mut Transform {
            &mut self.$field_name
        }

        #[inline]
        pub fn translation(&self) -> &Vect {
            &self.$field_name.translation
        }

        #[inline]
        pub fn translation_mut(&mut self) -> &mut Vect {
            &mut self.$field_name.translation
        }

        #[inline]
        pub fn rotation(&self) -> &Quat {
            &self.$field_name.rotation
        }

        #[inline]
        pub fn rotation_mut(&mut self) -> &mut Quat {
            &mut self.$field_name.rotation
        }

        chain_method!($S, $s, $field_name, with_translation(self, x: Scalar, y: Scalar, z: Scalar));
        chain_method!($S, $s, $field_name, with_translation_vect(self, vect: Vect));
        chain_method!($S, $s, $field_name, with_zero_translation(self));
        chain_method!($S, $s, $field_name, with_axis_angle(self, axis: Vect, angle: Scalar));
        chain_method!($S, $s, $field_name, with_rotation(self, rotation: Quat));
        chain_method!($S, $s, $field_name, with_zero_rotation(self));
    };

    (struct_name: $struct_name:ident,) => {
        include_transform_helpers! {
            struct_signature: $struct_name,
            struct_name: $struct_name,
            field_name: transform,
        }
    };
}
