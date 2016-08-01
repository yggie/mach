macro_rules! quickcheck_assert {
    ( $expression:expr, $message:expr ) => {
        if !$expression {
            return quickcheck::TestResult::error($message);
        }
    };

    ( $expression:expr, $message:expr, ) => {
        quickcheck_assert!($expression, $message);
    };

    ( $expression:expr ) => {
        quickcheck_assert!($expression, format!("expected {} to return true but got false", stringify!($expression)));
    };
}
