use ID;
use collisions::{CollisionData, CollisionObject};

pub trait CollisionObjectSpace<T> {
    fn find(&self, id: ID) -> Option<CollisionObject<T>>;
    fn objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a>;
    fn foreground_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a>;
    fn background_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a>;
    fn create_foreground_object(&mut self, data: CollisionData<T>) -> CollisionObject<T>;
    fn create_background_object(&mut self, data: CollisionData<T>) -> CollisionObject<T>;
}
