use entities::{RigidBody, StaticBody};

pub enum BodyType<'a> {
    Rigid(&'a RigidBody),
    Static(&'a StaticBody),
}

pub enum BodyTypeMut<'a> {
    Rigid(&'a mut RigidBody),
    Static(&'a mut StaticBody),
}
