use std::cell::{Ref, RefMut};

use ID;
use maths::IntegratableMut;
use entities::{Body, BodyParams};

pub trait EntityStore {
    fn create_rigid_body(&mut self, &BodyParams) -> ID;
    fn create_static_body(&mut self, &BodyParams) -> ID;
    fn find_body(&self, ID) -> Option<Ref<Box<Body>>>;
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a>;
    // fn create_static_body(&mut self, &BodyParams) -> ID;
    fn integratable_iter_mut<'a, 'b>(&'a mut self) -> Box<Iterator<Item=IntegratableMut> + 'a>;
}
