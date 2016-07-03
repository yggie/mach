use ID;
use utils::Ref;
use collisions::{Body, BodyDef, BodyHandle, Narrowphase};

pub trait CollisionObjectSpace<D, N> where N: Narrowphase {
    fn find(&self, id: ID) -> Option<BodyHandle<D, N>>;
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a>;
    fn create_body(&mut self, data: BodyDef<D>) -> BodyHandle<D, N>;
    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a>;
    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a>;
    fn background_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a>;
    fn background_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a>;
}
