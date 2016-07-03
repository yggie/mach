use utils::Ref;
use dynamics::{Body, FixedBody, FixedBodyData, FixedBodyHandle, RigidBody, RigidBodyData, RigidBodyHandle};
use collisions::NarrowphaseData;

pub trait PhysicsObjectSpace<T> where T: NarrowphaseData {
    // fn find(&self, id: ID) -> Option<Body<T>>;
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<T>>> + 'a>;
    fn rigid_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody<T>>> + 'a>;
    fn rigid_body_handles_iter<'a>(&'a self) -> Box<Iterator<Item=RigidBodyHandle<T>> + 'a>;
    fn fixed_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<FixedBody<T>>> + 'a>;
    fn fixed_body_handles_iter<'a>(&'a self) -> Box<Iterator<Item=FixedBodyHandle<T>> + 'a>;
    fn create_rigid_body(&mut self, data: RigidBodyData<T>) -> RigidBodyHandle<T>;
    fn create_fixed_body(&mut self, data: FixedBodyData<T>) -> FixedBodyHandle<T>;
}
