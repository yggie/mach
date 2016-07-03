use dynamics::{FixedBody, RigidBody};
use collisions::NarrowphaseData;

pub enum BodyRef<'a, T> where T: NarrowphaseData {
    Rigid(&'a RigidBody<T>),
    Fixed(&'a FixedBody<T>),
}
