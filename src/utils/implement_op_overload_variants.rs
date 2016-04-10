#[macro_export]
macro_rules! implement_op_overload_variants {
    ($trait_name:ident, $method:ident, $struct_name:ty, $target:ty, $output:ty) => {
        impl<'a> $trait_name<$target> for &'a $struct_name {
            type Output = $output;

            #[inline(always)]
            fn $method(self, other: $target) -> Self::Output {
                self.$method(&other)
            }
        }

        impl<'a> $trait_name<&'a $target> for $struct_name {
            type Output = $output;

            #[inline(always)]
            fn $method(self, other: &'a $target) -> Self::Output {
                (&self).$method(other)
            }
        }

        impl $trait_name<$target> for $struct_name {
            type Output = $output;

            #[inline(always)]
            fn $method(self, other: $target) -> Self::Output {
                (&self).$method(&other)
            }
        }
    };
}
