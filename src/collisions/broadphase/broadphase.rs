use collisions::{CollisionObject, CollisionObjectSpace, NarrowphaseData};

pub trait Broadphase<T>: CollisionObjectSpace<T> where T: NarrowphaseData {
    fn update(&mut self);
    fn possible_collision_pairs_iter(&self) -> Box<Iterator<Item=(CollisionObject<T>, CollisionObject<T>)>>;
}
