#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/null_narrowphase_test.rs"]
mod tests;

use maths::Transform;
use shapes::Shape;
use collisions::{CollisionData, Narrowphase};

pub struct NullNarrowphase { }

impl NullNarrowphase {
    pub fn new() -> NullNarrowphase {
        NullNarrowphase { }
    }
}

impl Narrowphase for NullNarrowphase {
    type Data = ();

    fn check(&self, _data_0: &CollisionData<Self::Data>, _data_1: &CollisionData<Self::Data>) -> bool {
        true
    }

    fn update(&mut self, _data: &mut CollisionData<Self::Data>) { }

    fn create_data(&mut self, _shape: &Shape, _transform: &Transform) -> Self::Data {
        ()
    }
}
