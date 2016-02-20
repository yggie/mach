use std::cell::{Ref, RefMut};

use maths::{Motion, Quat, Vect};
use entities::{Body, BodyType, BodyTypeMut};

pub struct Integratable<'a> {
    // TODO remove dependency on RefMut once #cell_extras has stabilized:
    // https://github.com/rust-lang/rust/issues/27746
    _body: Ref<'a, Box<Body>>,
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

    pub fn translation(&self) -> &Vect {
        self.body.translation()
    }

    pub fn rotation(&self) -> &Quat {
        self.body.rotation()
    }

    pub fn rotation_mut(&mut self) -> &mut Quat {
        self.body.rotation_mut()
    }

    pub fn translation_mut(&mut self) -> &mut Vect {
        self.body.translation_mut()
    }

    pub fn motion(&self) -> &Motion {
        match self.body.downcast() {
            BodyType::Rigid(rigid_body) => rigid_body.motion(),

            _otherwise => panic!("Unexpected body type as integratable!"),
        }
    }

    pub fn motion_mut(&mut self) -> &mut Motion {
        match self.body.downcast_mut() {
            BodyTypeMut::Rigid(rigid_body) => rigid_body.motion_mut(),

            _otherwise => panic!("Unexpected body type as integratable!"),
        }
    }

    pub fn velocity(&self) -> &Vect {
        &self.motion().velocity
    }

    pub fn velocity_mut(&mut self) -> &mut Vect {
        &mut self.motion_mut().velocity
    }

    pub fn angular_velocity(&self) -> &Vect {
        &self.motion().angular_velocity
    }
}
