use physics::{FixedBody, RigidBody};
use collisions::NarrowphaseData;

pub enum BodyRefMut<'a, T> where T: NarrowphaseData {
    Rigid(&'a mut RigidBody<T>),
    Fixed(&'a mut FixedBody<T>),
}
