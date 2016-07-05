#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/null_narrowphase_test.rs"]
mod tests;

use collisions::{BodyData, CollisionData, Narrowphase};

#[derive(Clone, Copy, Debug)]
pub struct NullNarrowphase { }

impl Narrowphase for NullNarrowphase {
    fn new(_data: &CollisionData) -> NullNarrowphase {
        NullNarrowphase { }
    }

    fn test(_body_0: &BodyData<Self>, _body_1: &BodyData<Self>) -> bool {
        true
    }

    fn update(_body: &mut BodyData<Self>) {
        // do nothing
    }
}
