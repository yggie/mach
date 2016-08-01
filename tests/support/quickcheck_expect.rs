macro_rules! quickcheck_expect {
    ( $expression:expr ) => {
        {
            let result = $expression;

            if result.is_failure() || result.is_error() {
                return result;
            } else {
                result
            }
        }
    };
}
