use collisions::{CloseProximityPair, CollisionObjectSpace, Narrowphase};

pub trait Broadphase<D, N>: CollisionObjectSpace<D, N> where N: Narrowphase {
    fn update(&mut self);
    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<D, N>>>;
}
