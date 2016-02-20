macro_rules! chain_method {
    ($struct_signature:ty, $struct_name:ident, $field_name:ident, $method_name:ident(self, $( $variable_name:ident : $type_name:ident ),*)) => {
        pub fn $method_name(self, $( $variable_name : $type_name ),*) -> $struct_signature {
            $struct_name {
                $field_name: self.$field_name.$method_name($( $variable_name ),*),
                .. self
            }
        }
    };

    ($struct_signature:ty, $struct_name:ident, $field_name:ident, $method_name:ident(self)) => {
        chain_method!($struct_signature, $struct_name, $field_name, $method_name(self,));
    };
}

#[macro_export]
macro_rules! inline_chainable_params_methods {
    (struct_signature: $S:ty, struct_name: $s:ident, field_name: $f:ident,) => {
        chain_method!($S, $s, $f, as_shape(self, shape_desc: ShapeDesc));
        chain_method!($S, $s, $f, as_sphere(self, radius: Scalar));
        chain_method!($S, $s, $f, as_cube(self, size: Scalar));
        chain_method!($S, $s, $f, with_translation(self, x: Scalar, y: Scalar, z: Scalar));
        chain_method!($S, $s, $f, with_translation_vect(self, vect: Vect));
        chain_method!($S, $s, $f, with_rotation(self, rotation: Quat));
        chain_method!($S, $s, $f, with_no_rotation(self));
        chain_method!($S, $s, $f, with_velocity(self, vx: Scalar, vy: Scalar, vz: Scalar));
        chain_method!($S, $s, $f, with_angular_velocity(self, wx: Scalar, wy: Scalar, wz: Scalar));
    };

    (struct_name: $struct_name:ident, field_name: $field_name:ident) => {
        inline_chainable_params_methods! {
            struct_signature: $struct_name,
            struct_name: $struct_name,
            field_name: $field_name,
        }
    };
}
