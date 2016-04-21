use Scalar;
use maths::{Transform, UnitQuat, Vect};
use shapes::{Cuboid, Shape};

#[derive(Clone, Debug)]
pub struct Form {
    shape: Box<Shape>,
    transform: Transform,
}

impl Form {
    include_transform_helpers! {
        struct_name: Form,
    }

    pub fn new(shape: Box<Shape>, transform: Transform) -> Form {
        Form {
            shape: shape,
            transform: transform,
        }
    }

    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    #[inline]
    pub fn with_boxed_shape(self, shape: Box<Shape>) -> Form {
        Form {
            shape: shape,
            .. self
        }
    }

    #[inline]
    pub fn with_shape<S: Shape + 'static>(self, shape: S) -> Form {
        self.with_boxed_shape(Box::new(shape))
    }

    #[inline]
    pub fn vertex(&self, index: usize) -> Vect {
        self.transform().apply_to_point(self.shape().vertex(index))
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
        let transform = self.transform.clone();
        let iterator = self.shape.vertices_iter()
            .map(move |v| transform.apply_to_point(v));

        return Box::new(iterator);
    }
}

impl Default for Form {
    fn default() -> Form {
        Form {
            shape: Box::new(Cuboid::cube(1.0)),
            transform: Transform::identity(),
        }
    }
}

#[macro_export]
macro_rules! include_form_helpers {
    (struct_signature: $S:ty, struct_name: $s:ident, field_name: $field_name:ident,) => {
        #[inline]
        pub fn form(&self) -> &Form {
            &self.$field_name
        }

        #[inline]
        pub fn form_mut(&mut self) -> &mut Form {
            &mut self.$field_name
        }

        #[inline]
        pub fn transform(&self) -> &Transform {
            self.$field_name.transform()
        }

        #[inline]
        pub fn transform_mut(&mut self) -> &mut Transform {
            self.$field_name.transform_mut()
        }

        #[inline]
        pub fn shape(&self) -> &Shape {
            self.$field_name.shape()
        }

        #[inline]
        pub fn translation(&self) -> &Vect {
            self.$field_name.translation()
        }

        #[inline]
        pub fn translation_mut(&mut self) -> &mut Vect {
            self.$field_name.translation_mut()
        }

        #[inline]
        pub fn rotation(&self) -> UnitQuat {
            self.$field_name.rotation()
        }

        #[inline]
        pub fn rotation_mut(&mut self) -> &mut UnitQuat {
            self.$field_name.rotation_mut()
        }

        #[inline]
        pub fn vertex(&self, index: usize) -> Vect {
            self.$field_name.vertex(index)
        }

        #[inline]
        pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
            self.$field_name.vertices_iter()
        }

        #[inline]
        pub fn with_shape<S: Shape + 'static>(self, shape: S) -> $S {
            $s {
                $field_name: self.$field_name.with_shape(shape),
                .. self
            }
        }

        #[inline]
        pub fn with_boxed_shape(self, shape: Box<Shape>) -> $S {
            $s {
                $field_name: self.$field_name.with_boxed_shape(shape),
                .. self
            }
        }

        chain_method!($S, $s, $field_name, with_translation(self, x: Scalar, y: Scalar, z: Scalar));
        chain_method!($S, $s, $field_name, with_translation_vect(self, vect: Vect));
        chain_method!($S, $s, $field_name, with_zero_translation(self));
        chain_method!($S, $s, $field_name, with_axis_angle(self, axis: Vect, angle: Scalar));
        chain_method!($S, $s, $field_name, with_rotation(self, rotation: UnitQuat));
        chain_method!($S, $s, $field_name, with_zero_rotation(self));
    };

    (struct_name: $struct_name:ident,) => {
        include_form_helpers! {
            struct_signature: $struct_name,
            struct_name: $struct_name,
            field_name: form,
        }
    };
}
