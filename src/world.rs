use Scalar;
use maths::Vec3D;
use utils::Ref;
use dynamics::{DynamicBody, DynamicBodyHandle, DynamicBodyType, FixedBodyDef, RigidBodyDef};
use collisions::{Contact, Narrowphase};

pub trait World<N, T> where N: Narrowphase {
    fn update(&mut self, time_step: Scalar) -> Vec<Contact<N, DynamicBodyType<T>>>;
    fn set_gravity(&mut self, gravity: Vec3D);
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<DynamicBody<N, T>>> + 'a>;
    fn create_rigid_body(&mut self, def: RigidBodyDef, extra: T) -> DynamicBodyHandle<N, T>;
    fn create_fixed_body(&mut self, def: FixedBodyDef, extra: T) -> DynamicBodyHandle<N, T>;
}
