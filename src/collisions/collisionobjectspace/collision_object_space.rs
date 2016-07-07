use ID;
use utils::{Handle, Ref, RefMut};
use collisions::{BodyDef, CollisionBody};

pub trait CollisionObjectSpace<B> where B: CollisionBody {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, B>>;
    fn find_handle(&self, id: ID) -> Option<&Handle<B>>;
    fn create_body(&mut self, body_def: BodyDef, extension: B::Extension) -> Handle<B>;
    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a>;
    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<B>> + 'a>;
    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<B>> + 'a>;
    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a>;
    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<B>> + 'a>;
    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<B>> + 'a>;

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a> {
        let iterator = self.foreground_bodies_iter()
            .chain(self.environment_bodies_iter());

        return Box::new(iterator);
    }

    fn bodies_iter_mut<'a>(&'a self) -> Box<Iterator<Item=RefMut<B>> + 'a> {
        let iterator = self.foreground_bodies_mut_iter()
            .chain(self.environment_bodies_mut_iter());

        return Box::new(iterator);
    }
}
