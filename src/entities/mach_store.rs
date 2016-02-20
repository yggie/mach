#[cfg(test)]
#[path="../../tests/entities/mach_store_test.rs"]
mod mach_store_test;

use std::cell::{Ref, RefCell, RefMut};

use ID;
use maths::IntegratableMut;
use entities::{Body, BodyParams, BodyType, EntityStore, RigidBody, StaticBody};

pub struct MachStore {
    bodies: Vec<RefCell<Box<Body>>>,
}

impl MachStore {
    pub fn new() -> MachStore {
        MachStore { bodies: Vec::new() }
    }

    fn gen_id(&self) -> ID {
        ID(self.bodies.len() as u32)
    }
}

impl EntityStore for MachStore {
    fn create_rigid_body(&mut self, params: &BodyParams) -> ID {
        let id = self.gen_id();
        let rigid_body = RigidBody::with_id(id, params);
        let cell = RefCell::new(Box::new(rigid_body) as Box<Body>);

        self.bodies.push(cell);

        return id;
    }

    fn create_static_body(&mut self, params: &BodyParams) -> ID {
        let id = self.gen_id();
        let static_body = StaticBody::with_id(id, params);
        let cell = RefCell::new(Box::new(static_body) as Box<Body>);

        self.bodies.push(cell);

        return id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<Box<Body>>> {
        self.bodies.get(id.0 as usize).map(|cell| cell.borrow())
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a> {
        let iterator = self.bodies.iter()
            .map(|cell| cell.borrow_mut());

        return Box::new(iterator);
    }

    fn integratable_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=IntegratableMut> + 'a> {
        let iterator = self.bodies.iter()
            .filter_map(|cell| {
                let body_ref = cell.borrow_mut();
                match body_ref.downcast() {
                    BodyType::Rigid(_rigid_body) => (),

                    _otherwise => return None,
                }

                return Some(IntegratableMut::new(body_ref));
            });

        return Box::new(iterator);
    }
}
