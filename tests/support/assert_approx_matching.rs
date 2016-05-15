#[macro_export]
macro_rules! assert_approx_matching {
    ($left:expr, $right:expr) => {
        {
            use maths::ApproxEq;

            let left = $left;
            let right = $right;

            assert!(left.len() == right.len(), format!("LHS ({:?}) does not have the same number of elements as RHS ({:?}): {} vs {}", left, right, left.len(), right.len()));

            let mut left_clone = left.clone();
            let mut missing_values = Vec::new();

            for right_item in right.iter() {
                let result = left_clone.iter().position(|left_item| {
                    left_item.approx_eq(right_item)
                });

                if let Some(index) = result {
                    left_clone.remove(index);
                } else {
                    missing_values.push(right_item.clone());
                }
            }

            if missing_values.len() != 0 {
                panic!(format!("the LHS ({:?}) did not match the RHS ({:?}) vec, missing: {:?}", left, right, missing_values));
            }
        }
    };
}
