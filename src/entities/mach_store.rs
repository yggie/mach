#[cfg(test)]
#[path="../../tests/entities/mach_store_test.rs"]
mod mach_store_test;

use std::mem;

use ID;
use entities::{Body, BodyHandle, BodyType, EntityStore, Ref, RefMut, RigidBody, StaticBody};

pub struct MachStore {
    bodies: Vec<BodyHandle>,
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
    fn add_rigid_body(&mut self, rigid_body: RigidBody) -> ID {
        let id = self.gen_id();
        let rc_cell = BodyHandle::new(Box::new(rigid_body.with_id_(id)) as Box<Body>);

        self.bodies.push(rc_cell);

        return id;
    }

    fn add_static_body(&mut self, static_body: StaticBody) -> ID {
        let id = self.gen_id();
        let rc_cell = BodyHandle::new(Box::new(static_body.with_id_(id)) as Box<Body>);

        self.bodies.push(rc_cell);

        return id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<Box<Body>>> {
        self.bodies.get(id.0 as usize).map(|rc_cell| rc_cell.borrow())
    }

    fn find_rigid_body(&self, id: ID) -> Option<Ref<Box<RigidBody>>> {
        self.find_body(id).and_then(|body| {
            match body.downcast() {
                BodyType::Rigid(_rigid_body) => (),
                _otherwise => return None,
            }

            // TODO use the safe approach once #cell_extras has stabilized:
            // https://github.com/rust-lang/rust/issues/27746
            return unsafe { Some(mem::transmute(body)) };
        })
    }

    fn find_body_handle(&self, id: ID) -> Option<&BodyHandle> {
        self.bodies.get(id.0 as usize)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a> {
        Box::new(self.bodies.iter().map(|rc_cell| rc_cell.borrow()))
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a> {
        Box::new(self.bodies.iter().map(|rc_cell| rc_cell.borrow_mut()))
    }

    fn rigid_body_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<RigidBody>>> + 'a> {
        let iterator = self.bodies.iter()
            .filter_map(|rc_cell| {
                let body_ref = rc_cell.borrow_mut();

                match body_ref.downcast() {
                    BodyType::Rigid(_rigid_body) => (),

                    _otherwise => return None,
                }

                // TODO use the safe approach once #cell_extras has stabilized:
                // https://github.com/rust-lang/rust/issues/27746
                return unsafe { Some(mem::transmute(body_ref)) };
            });

        return Box::new(iterator);
    }
}
