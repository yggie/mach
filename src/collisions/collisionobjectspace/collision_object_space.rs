use ID;
use utils::{Handle, Ref, RefMut};
use collisions::{BodyDef, CollisionObject};

pub trait CollisionObjectSpace<O> where O: CollisionObject {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, O>>;
    fn find_handle(&self, id: ID) -> Option<&Handle<O>>;
    fn create_body(&mut self, body_def: BodyDef, extension: O::Extension) -> Handle<O>;
    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a>;
    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a>;
    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a>;
    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a>;
    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a>;
    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a>;

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        let iterator = self.foreground_bodies_iter()
            .chain(self.environment_bodies_iter());

        return Box::new(iterator);
    }

    fn bodies_iter_mut<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        let iterator = self.foreground_bodies_mut_iter()
            .chain(self.environment_bodies_mut_iter());

        return Box::new(iterator);
    }
}
