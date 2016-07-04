use collisions::{Body, CollisionData};

pub trait Narrowphase: 'static + Clone {
    fn new(data: &CollisionData) -> Self;
    fn test<T>(body_0: &Body<Self, T>, body_1: &Body<Self, T>) -> bool;
    fn update<T>(body: &mut Body<Self, T>);
}
