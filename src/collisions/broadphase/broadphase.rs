use utils::Ref;
use collisions::{CloseProximityPair, CollisionObject, CollisionObjectSpace};
use collisions::shapes::Ray;

pub trait Broadphase<B>: CollisionObjectSpace<B> where B: CollisionObject {
    fn update(&mut self);
    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<B>>>;
    fn cast_ray<'a>(&'a self, ray: &Ray) -> Box<Iterator<Item=Ref<B>> + 'a>;
}
