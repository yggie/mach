use Scalar;
use maths::Vect;

#[derive(Clone, Debug)]
pub struct Motion {
    pub velocity: Vect,
    pub angular_velocity: Vect,
}

impl Motion {
    pub fn new(velocity: Vect, angular_velocity: Vect) -> Motion {
        Motion {
            velocity: velocity,
            angular_velocity: angular_velocity,
        }
    }

    pub fn stationary() -> Motion {
        Motion {
            velocity: Vect::zero(),
            angular_velocity: Vect::zero(),
        }
    }

    #[inline]
    pub fn with_velocity_vect(self, velocity: Vect) -> Motion {
        Motion {
            velocity: velocity,
            .. self
        }
    }

    #[inline]
    pub fn with_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        self.with_velocity_vect(Vect::new(x, y, z))
    }

    #[inline]
    pub fn with_angular_velocity_vect(self, angular_velocity: Vect) -> Motion {
        Motion {
            angular_velocity: angular_velocity,
            .. self
        }
    }

    #[inline]
    pub fn with_angular_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        self.with_angular_velocity_vect(Vect::new(x, y, z))
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
        pub fn velocity(&self) -> &Vect {
            &self.$field_name.velocity
        }

        #[inline]
        pub fn velocity_mut(&mut self) -> &mut Vect {
            &mut self.$field_name.velocity
        }

        #[inline]
        pub fn angular_velocity(&self) -> &Vect {
            &self.$field_name.angular_velocity
        }

        #[inline]
        pub fn angular_velocity_mut(&mut self) -> &mut Vect {
            &mut self.$field_name.angular_velocity
        }

        chain_method!($S, $s, $field_name, with_velocity(self, vx: Scalar, vy: Scalar, vz: Scalar));
        chain_method!($S, $s, $field_name, with_velocity_vect(self, velocity: Vect));
        chain_method!($S, $s, $field_name, with_angular_velocity(self, wx: Scalar, wy: Scalar, wz: Scalar));
        chain_method!($S, $s, $field_name, with_angular_velocity_vect(self, angular_velocity: Vect));
    };

    (struct_name: $struct_name:ident,) => {
        include_motion_helpers! {
            struct_signature: $struct_name,
            struct_name: $struct_name,
            field_name: motion,
        }
    };
}
