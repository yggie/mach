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

    pub fn with_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        Motion {
            velocity: Vect::new(x, y, z),
            .. self
        }
    }

    pub fn with_angular_velocity(self, x: Scalar, y: Scalar, z: Scalar) -> Motion {
        Motion {
            angular_velocity: Vect::new(x, y, z),
            .. self
        }
    }
}

#[macro_export]
macro_rules! motion_field_accessors {
    (field_name: $field_name:ident) => {
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
    };
}
