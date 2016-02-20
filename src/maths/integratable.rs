use std::cell::{Ref, RefMut};

use maths::{Motion, Transform};
use entities::{Body, BodyType};

pub struct Integratable<'a> {
    // TODO remove dependency on RefMut once #cell_extras has stabilized:
    // https://github.com/rust-lang/rust/issues/27746
    body: Ref<'a, Box<Body>>,
}

pub struct IntegratableMut<'a> {
    // TODO remove dependency on RefMut once #cell_extras has stabilized:
    // https://github.com/rust-lang/rust/issues/27746
    body: RefMut<'a, Box<Body>>,
}

// TODO how it should have looked:
// pub struct IntegratableMut<'a> {
//     transform: &'a mut Transform,
//     motion: &'a mut Motion,
// }

impl<'a> IntegratableMut<'a> {
    pub fn new(body_ref: RefMut<'a, Box<Body>>) -> IntegratableMut<'a> {
        IntegratableMut {
            body: body_ref,
        }
    }

    pub fn motion(&self) -> &Motion {
        match self.body.downcast() {
            BodyType::Rigid(rigid_body) => rigid_body.motion(),

            _otherwise => panic!("Unexpected body type as integratable!"),
        }
    }
}
