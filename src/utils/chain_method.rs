#[macro_export]
macro_rules! chain_method {
    ($struct_signature:ty, $struct_name:ident, $field_name:ident, $method_name:ident(self, $( $variable_name:ident : $type_name:ident ),*)) => {
        #[inline]
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
