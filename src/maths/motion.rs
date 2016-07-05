use Scalar;
use maths::Vec3D;

// TODO move this into physics module
#[derive(Clone, Debug)]
pub struct Motion {
    pub velocity: Vec3D,
    pub angular_velocity: Vec3D,
}

impl Motion {
    pub fn new(velocity: Vec3D, angular_velocity: Vec3D) -> Motion {
        Motion {
            velocity: velocity,
            angular_velocity: angular_velocity,
        }
    }

    pub fn stationary() -> Motion {
        Motion {
            velocity: Vec3D::zero(),
            angular_velocity: Vec3D::zero(),
        }
    }

    pub fn zero() -> Motion {
        Motion::stationary()
    }

    #[inline]
    pub fn with_velocity_vect(self, velocity: Vec3D) -> Motion {
        Motion {
            velocity: velocity,
            .. self
        }
    }

    #[inline]
    pub fn with_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        self.with_velocity_vect(Vec3D::new(x, y, z))
    }

    #[inline]
    pub fn with_angular_velocity_vect(self, angular_velocity: Vec3D) -> Motion {
        Motion {
            angular_velocity: angular_velocity,
            .. self
        }
    }

    #[inline]
    pub fn with_angular_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        self.with_angular_velocity_vect(Vec3D::new(x, y, z))
    }
}

#[macro_export]
macro_rules! include_motion_helpers {
    (struct_signature: $S:ty, struct_name: $s:ident, field_name: $field_name:ident,) => {
        #[inline]
        pub fn motion(&self) -> &Motion {
            &self.$field_name
        }

        #[inline]
        pub fn motion_mut(&mut self) -> &mut Motion {
            &mut self.$field_name
        }

        #[inline]
        pub fn velocity(&self) -> &Vec3D {
            &self.$field_name.velocity
        }

        #[inline]
        pub fn velocity_mut(&mut self) -> &mut Vec3D {
            &mut self.$field_name.velocity
        }

        #[inline]
        pub fn angular_velocity(&self) -> &Vec3D {
            &self.$field_name.angular_velocity
        }

        #[inline]
        pub fn angular_velocity_mut(&mut self) -> &mut Vec3D {
            &mut self.$field_name.angular_velocity
        }

        chain_method!($S, $s, $field_name, with_velocity(self, vx: Scalar, vy: Scalar, vz: Scalar));
        chain_method!($S, $s, $field_name, with_velocity_vect(self, velocity: Vec3D));
        chain_method!($S, $s, $field_name, with_angular_velocity(self, wx: Scalar, wy: Scalar, wz: Scalar));
        chain_method!($S, $s, $field_name, with_angular_velocity_vect(self, angular_velocity: Vec3D));
    };

    (struct_signature: $struct_signature:ty, struct_name: $struct_name:ident,) => {
        include_motion_helpers! {
            struct_signature: $struct_signature,
            struct_name: $struct_name,
            field_name: motion,
        }
    };

    (struct_name: $struct_name:ident,) => {
        include_motion_helpers! {
            struct_signature: $struct_name,
            struct_name: $struct_name,
            field_name: motion,
        }
    };
}
