#[cfg(test)]
#[path="../../tests/maths/transform_test.rs"]
mod transform_test;

use Scalar;
use maths::{UnitQuat, UnitVec3D, Vec3D};

// TODO move this into collisions module
/// The `Transform` object represents a spatial transformation in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec3D,
    pub rotation: UnitQuat,
}

impl Transform {
    /// Creates a new `Transform` instance with the given translation and
    /// rotation.
    pub fn new(translation: Vec3D, rotation: UnitQuat) -> Transform {
        Transform {
            translation: translation,
            rotation: rotation,
        }
    }

    /// Creates a new `Transform` instance representing the identity
    /// transformation.
    pub fn identity() -> Transform {
        Transform::new(Vec3D::zero(), UnitQuat::identity())
    }

    pub fn translate(mut self, x: Scalar, y: Scalar, z: Scalar) -> Transform {
        self.translation.x += x;
        self.translation.y += y;
        self.translation.z += z;

        self
    }

    pub fn rotate(self, axis: UnitVec3D, angle: Scalar) -> Transform {
        let rotation = UnitQuat::from_axis_angle(axis, angle);

        Transform {
            translation: rotation.rotate(self.translation),
            rotation: self.rotation * rotation,
        }
    }

    /// The positional translation component of the transform.
    #[inline(always)]
    pub fn translation(&self) -> Vec3D {
        self.translation
    }

    /// Returns a mutable reference to the translation `Vec3D`.
    #[inline(always)]
    pub fn translation_mut(&mut self) -> &mut Vec3D {
        &mut self.translation
    }

    /// The rotational component of the transform.
    #[inline(always)]
    pub fn rotation(&self) -> UnitQuat {
        self.rotation
    }

    /// Returns a mutable reference to the rotation `UnitQuat`.
    #[inline(always)]
    pub fn rotation_mut(&mut self) -> &mut UnitQuat {
        &mut self.rotation
    }

    #[inline]
    pub fn with_translation_vect(self, translation: Vec3D) -> Transform {
        Transform {
            translation: translation,
            .. self
        }
    }

    #[inline]
    pub fn with_translation(self, x: Scalar, y: Scalar, z: Scalar) -> Transform {
        self.with_translation_vect(Vec3D::new(x, y, z))
    }

    #[inline]
    pub fn with_zero_translation(self) -> Transform {
        self.with_translation_vect(Vec3D::zero())
    }

    #[inline]
    pub fn with_rotation(self, rotation: UnitQuat) -> Transform {
        Transform {
            rotation: rotation,
            .. self
        }
    }

    #[inline]
    pub fn with_axis_angle(self, axis: UnitVec3D, angle: Scalar) -> Transform {
        self.with_rotation(UnitQuat::from_axis_angle(axis, angle))
    }

    #[inline]
    pub fn with_zero_rotation(self) -> Transform {
        self.with_rotation(UnitQuat::identity())
    }

    /// Applies the transform to a point.
    pub fn apply_to_point(&self, point: Vec3D) -> Vec3D {
        self.rotation().rotate(point) + self.translation()
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
        pub fn translation(&self) -> &Vec3D {
            &self.$field_name.translation
        }

        #[inline]
        pub fn translation_mut(&mut self) -> &mut Vec3D {
            &mut self.$field_name.translation
        }

        #[inline(always)]
        pub fn rotation(&self) -> UnitQuat {
            self.$field_name.rotation
        }

        #[inline]
        pub fn rotation_mut(&mut self) -> &mut UnitQuat {
            &mut self.$field_name.rotation
        }

        chain_method!($S, $s, $field_name, with_translation(self, x: Scalar, y: Scalar, z: Scalar));
        chain_method!($S, $s, $field_name, with_translation_vect(self, vect: Vec3D));
        chain_method!($S, $s, $field_name, with_zero_translation(self));
        chain_method!($S, $s, $field_name, with_axis_angle(self, axis: UnitVec3D, angle: Scalar));
        chain_method!($S, $s, $field_name, with_rotation(self, rotation: UnitQuat));
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
