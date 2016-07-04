use collisions::{CloseProximityPair, CollisionObjectSpace, Narrowphase};

pub trait Broadphase<N, T>: CollisionObjectSpace<N, T> where N: Narrowphase {
    fn update(&mut self);
    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<N, T>>>;
}
