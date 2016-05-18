use maths::Transform;
use shapes::Shape;
use collisions::CollisionData;

pub trait Narrowphase {
    type Data: Sized;

    fn check(&self, &CollisionData<Self::Data>, other: &CollisionData<Self::Data>) -> bool;
    // TODO having all this unnecessary &mut self restriction forces this
    // operation to only be validly executed in a single threaded context. Can
    // this be avoided?
    fn update(&mut self, &mut CollisionData<Self::Data>);
    fn create_data(&mut self, shape: &Shape, transform: &Transform) -> Self::Data;
}
