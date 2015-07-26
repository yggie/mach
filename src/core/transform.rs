use math::{ Vector, Quaternion };

/// The `Transform` object represents a spatial transformation in 3D space.
#[derive(Clone, Copy)]
pub struct Transform {
    translation: Vector,
    rotation: Quaternion,
}

impl Transform {
    /// Create a new `Transform` instance with the given translation and
    /// rotation.
    pub fn new(translation: Vector, rotation: Quaternion) -> Transform {
        Transform {
            translation: translation,
            rotation: rotation,
        }
    }

    /// The positional translation component of the transform.
    #[inline(always)]
    pub fn translation(&self) -> Vector {
        self.translation
    }

    /// The rotational component of the transform.
    #[inline(always)]
    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    /// Applies the transform to a point.
    pub fn apply_to_point(&self, point: Vector) -> Vector {
        point.rotate_by_quaternion(self.rotation()) + self.translation()
    }

    /// Applies the inverse of the transform to a direction.
    pub fn apply_inverse_to_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rotation().inverse())
    }
}
