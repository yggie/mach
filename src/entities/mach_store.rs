#[cfg(test)]
#[path="../../tests/entities/mach_store_test.rs"]
mod mach_store_test;

use ID;
use maths::IntegratableMut;
use entities::{Body, BodyHandle, BodyParams, BodyType, EntityStore, Ref, RefMut, RigidBody, StaticBody};

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
    fn create_rigid_body(&mut self, params: &BodyParams) -> ID {
        let id = self.gen_id();
        let rigid_body = RigidBody::with_id(id, params);
        let rc_cell = BodyHandle::new(Box::new(rigid_body) as Box<Body>);

        self.bodies.push(rc_cell);

        return id;
    }

    fn create_static_body(&mut self, params: &BodyParams) -> ID {
        let id = self.gen_id();
        let static_body = StaticBody::with_id(id, params);
        let rc_cell = BodyHandle::new(Box::new(static_body) as Box<Body>);

        self.bodies.push(rc_cell);

        return id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<Box<Body>>> {
        self.bodies.get(id.0 as usize).map(|rc_cell| rc_cell.borrow())
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

    fn integratable_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=IntegratableMut> + 'a> {
        let iterator = self.bodies.iter()
            .filter_map(|rc_cell| {
                let body_ref = rc_cell.borrow_mut();
                match body_ref.downcast() {
                    BodyType::Rigid(_rigid_body) => (),

                    _otherwise => return None,
                }

                return Some(IntegratableMut::new(body_ref));
            });

        return Box::new(iterator);
    }
}
