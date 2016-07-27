macro_rules! assert_in_quickcheck {
    ( $expression:expr, $message:expr ) => {
        if !$expression {
            return quickcheck::TestResult::error($message);
        }
    };

    ( $expression:expr, $message:expr, ) => {
        assert_in_quickcheck!($expression, $message);
    };

    ( $expression:expr ) => {
        assert_in_quickcheck!($expression, format!("expected {} to return true but got false", stringify!($expression)));
    };
}
