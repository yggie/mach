use collisions::{CollisionObject, CollisionObjectSpace};

pub trait Broadphase<T>: CollisionObjectSpace<T> {
    fn update(&mut self);
    fn possible_collision_pairs_iter<'a>(&'a self) -> Box<Iterator<Item=(CollisionObject<T>, CollisionObject<T>)> + 'a>;
}
