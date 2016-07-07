use collisions::{CloseProximityPair, CollisionBody, CollisionObjectSpace};

pub trait Broadphase<B>: CollisionObjectSpace<B> where B: CollisionBody {
    fn update(&mut self);
    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<B>>>;
}
