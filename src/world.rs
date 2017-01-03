use Scalar;
use maths::Vec3D;
use utils::{Ref, Handle};
use dynamics::{DynamicBody, FixedBodyDef, RigidBodyDef};
use collisions::Contact;
use collisions::shapes::Ray;

pub trait World<T> where T: DynamicBody {
    fn update(&mut self, time_step: Scalar) -> Vec<Contact<T>>;
    fn set_gravity(&mut self, gravity: Vec3D);
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<T>> + 'a>;
    fn create_rigid_body(&mut self, def: RigidBodyDef, extension: <T as DynamicBody>::Extension) -> Handle<T>;
    fn create_fixed_body(&mut self, def: FixedBodyDef, extension: <T as DynamicBody>::Extension) -> Handle<T>;
    fn cast_ray<'a>(&'a self, ray: &Ray) -> Option<Ref<'a, T>>;
}
