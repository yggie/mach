use ID;
use maths::IntegratableMut;
use entities::{Body, BodyHandle, BodyParams, Ref, RefMut};

pub trait EntityStore {
    fn create_rigid_body(&mut self, &BodyParams) -> ID;
    fn create_static_body(&mut self, &BodyParams) -> ID;
    fn find_body(&self, ID) -> Option<Ref<Box<Body>>>;
    fn find_body_handle(&self, ID) -> Option<&BodyHandle>;
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a>;
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a>;
    fn integratable_iter_mut<'a, 'b>(&'a mut self) -> Box<Iterator<Item=IntegratableMut> + 'a>;
}
