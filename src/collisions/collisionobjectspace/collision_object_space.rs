use ID;
use utils::{Ref, RefMut};
use collisions::{Body, BodyDef, BodyHandle, Narrowphase};

pub trait CollisionObjectSpace<N, T> where N: Narrowphase {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, Body<N, T>>>;
    fn find_handle(&self, id: ID) -> Option<&BodyHandle<N, T>>;
    fn create_body(&mut self, data: BodyDef<T>) -> BodyHandle<N, T>;
    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a>;
    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a>;
    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a>;
    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a>;
    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a>;
    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a>;

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        let iterator = self.foreground_bodies_iter()
            .chain(self.environment_bodies_iter());

        return Box::new(iterator);
    }

    fn bodies_iter_mut<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a> {
        let iterator = self.foreground_bodies_mut_iter()
            .chain(self.environment_bodies_mut_iter());

        return Box::new(iterator);
    }
}
