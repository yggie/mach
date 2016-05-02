use entities::{RigidBody, StaticBody};

pub enum BodyRef<'a> {
    Rigid(&'a RigidBody),
    Static(&'a StaticBody),
}

pub enum BodyRefMut<'a> {
    Rigid(&'a mut RigidBody),
    Static(&'a mut StaticBody),
}
