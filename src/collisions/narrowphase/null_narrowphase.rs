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

    fn test<D>(_body_0: &Body<D, Self>, _body_1: &Body<D, Self>) -> bool {
        true
    }

    fn update<D>(_body: &mut Body<D, Self>) {
        // do nothing
    }
}
