#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/null_narrowphase_test.rs"]
mod tests;

use collisions::{Body, CollisionData, Narrowphase};

#[derive(Clone, Copy, Debug)]
pub struct NullNarrowphase { }

impl Narrowphase for NullNarrowphase {
    fn new(_data: &CollisionData) -> NullNarrowphase {
        NullNarrowphase { }
    }

    fn test<T>(_body_0: &Body<Self, T>, _body_1: &Body<Self, T>) -> bool {
        true
    }

    fn update<T>(_body: &mut Body<Self, T>) {
        // do nothing
    }
}
