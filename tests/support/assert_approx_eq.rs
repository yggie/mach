#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        {
            use mach::maths::ApproxEq;

            let left = $left;
            let right = $right;

            if !left.approx_eq(right) {
                panic!("assertion failed: `(left.approx_eq(right))` (left: `{:?}`, right: `{:?}`)", left, right);
            }
        }
    };
}
