use ID;
use entities::{Body, BodyHandle, Ref, RefMut, RigidBody, StaticBody};

pub trait EntityStore {
    fn add_rigid_body(&mut self, RigidBody) -> ID;
    fn add_static_body(&mut self, StaticBody) -> ID;
    fn find_body(&self, ID) -> Option<Ref<Box<Body>>>;
    fn find_rigid_body(&self, ID) -> Option<Ref<Box<RigidBody>>>;
    fn find_body_handle(&self, ID) -> Option<&BodyHandle>;
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a>;
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a>;
    fn rigid_body_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<RigidBody>>> + 'a>;
}
