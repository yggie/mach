use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };
use std::collections::HashMap;

use ID;
use core::SharedCell;
use maths::{ State, Transform };
use shapes::Shape;
use entities::{ Material, RigidBody, StaticBody, VolumetricBody };
use collisions::{ CollisionSpace, Contact, ContactPair };
use collisions::narrowphase::{ GjkEpaImplementation, Intersection };

/// A simple implementation for representing space in the simulation.
pub struct SimpleCollisionSpace {
    registry: HashMap<ID, SharedCell<RigidBody>>,
    static_registry: HashMap<ID, SharedCell<StaticBody>>,
    rigid_body_pairs: Vec<(SharedCell<RigidBody>, SharedCell<RigidBody>)>,
    rigid_static_body_pairs: Vec<(SharedCell<RigidBody>, SharedCell<StaticBody>)>,
    counter: usize,
}

impl SimpleCollisionSpace {
    /// Instantiates a new `SimpleCollisionSpace` object.
    pub fn new() -> SimpleCollisionSpace {
        SimpleCollisionSpace {
            registry: HashMap::new(),
            static_registry: HashMap::new(),
            rigid_body_pairs: Vec::new(),
            rigid_static_body_pairs: Vec::new(),
            counter: 0,
        }
    }

    fn generate_id(&mut self) -> ID {
        self.counter = self.counter + 1;

        return ID(self.counter as u32);
    }
}

impl CollisionSpace for SimpleCollisionSpace {
    fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID {
        let new_id = self.generate_id();
        let new_body = RigidBody::new_with_id(new_id, Box::new(shape), material, state);
        let new_shared_cell = Rc::new(RefCell::new(new_body));

        for shared_cell in self.registry.values() {
            self.rigid_body_pairs.push((shared_cell.clone(), new_shared_cell.clone()));
        }

        self.registry.insert(new_id, new_shared_cell);
        return new_id;
    }

    fn create_static_body<S: Shape>(&mut self, shape: S, material: &Material, transform: Transform) -> ID {
        let new_id = self.generate_id();
        let new_static_body = StaticBody::new_with_id(new_id, Box::new(shape), material, transform);
        let new_rc_cell = Rc::new(RefCell::new(new_static_body));

        for shared_cell in self.registry.values() {
            self.rigid_static_body_pairs.push((shared_cell.clone(), new_rc_cell.clone()));
        }

        self.static_registry.insert(new_id, new_rc_cell);
        return new_id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.registry.get(&id).map(|cell| cell.borrow())
    }

    fn find_static_body(&self, id: ID) -> Option<Ref<StaticBody>> {
        self.static_registry.get(&id).map(|cell| cell.borrow())
    }

    fn find_body_mut(&mut self, id: ID) -> Option<RefMut<RigidBody>> {
        self.registry.get_mut(&id).map(|cell| cell.borrow_mut())
    }

    fn find_static_body_mut(&mut self, id: ID) -> Option<RefMut<StaticBody>> {
        self.static_registry.get_mut(&id).map(|cell| cell.borrow_mut())
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        Box::new(self.registry.values().map(|cell| cell.borrow()))
    }

    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<StaticBody>> + 'a> {
        Box::new(self.static_registry.values().map(|cell| cell.borrow()))
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<RigidBody>> + 'a> {
        Box::new(self.registry.iter_mut().map(|(_, cell)| cell.borrow_mut()))
    }

    fn find_intersection(&self, body_0: &VolumetricBody, body_1: &VolumetricBody) -> Option<Intersection> {
        GjkEpaImplementation.find_intersection(body_0, body_1)
    }

    fn find_contacts(&self) -> Option<Vec<Contact>> {
        let mut contacts = Vec::new();

        for &(ref rc_cell_0, ref rc_cell_1) in self.rigid_body_pairs.iter() {
            let body_0 = &*rc_cell_0.borrow();
            let body_1 = &*rc_cell_1.borrow();

            if let Some(intersection) = GjkEpaImplementation.find_intersection(body_0, body_1) {
                contacts.push(
                    Contact {
                        pair: ContactPair::RigidRigid(rc_cell_0.clone(), rc_cell_1.clone()),
                        center: intersection.point(),
                        normal: intersection.normal(),
                    }
                );
            }
        }

        for &(ref rigid_body_rc_cell, ref static_body_rc_cell) in self.rigid_static_body_pairs.iter() {
            let rigid_body = &*rigid_body_rc_cell.borrow();
            let static_body = &*static_body_rc_cell.borrow();

            if let Some(intersection) = GjkEpaImplementation.find_intersection(rigid_body, static_body) {
                contacts.push(
                    Contact {
                        pair: ContactPair::RigidStatic(rigid_body_rc_cell.clone(), static_body_rc_cell.clone()),
                        center: intersection.point(),
                        normal: intersection.normal(),
                    }
                );
            }
        }

        if contacts.len() > 0 {
            return Some(contacts);
        } else {
            return None;
        }
    }
}
