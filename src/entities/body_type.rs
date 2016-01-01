use entities::{RigidBody, StaticBody};

pub enum BodyType<'a> {
    Rigid(&'a RigidBody),
    Static(&'a StaticBody),
}
