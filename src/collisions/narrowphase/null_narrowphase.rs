#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/null_narrowphase_test.rs"]
mod tests;

use collisions::{CollisionData, Narrowphase};
use collisions::narrowphase::{NarrowphaseRef, NarrowphaseRefMut};

#[derive(Clone, Copy, Debug)]
pub struct NullNarrowphase { }

impl Narrowphase for NullNarrowphase {
    fn new(_data: &CollisionData) -> NullNarrowphase {
        NullNarrowphase { }
    }

    fn test(_ref_0: NarrowphaseRef<Self>, _ref_1: NarrowphaseRef<Self>) -> bool {
        true
    }

    fn update(_ref: NarrowphaseRefMut<Self>) {
        // do nothing
    }
}
