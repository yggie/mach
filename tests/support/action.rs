extern crate quickcheck;

use std::fmt;

pub trait Action<T>: fmt::Debug + quickcheck::Arbitrary {
    fn perform(&mut self, target: &mut T);
}
