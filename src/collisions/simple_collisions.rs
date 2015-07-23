use std::collections::HashMap;

use core::{ Body, State, StaticBody };
use math::{ Vector, Quaternion };
use shapes::Shape;
use materials::Material;
use collisions::{ Collisions, Contact, ContactPair, Proximity };

/// A simple implementation for representing space in the simulation.
pub struct SimpleCollisions {
    registry: HashMap<usize, Body<usize>>,
    static_registry: HashMap<usize, StaticBody<usize>>,
    proximities: Vec<Proximity<usize>>,
    counter: usize,
}

impl SimpleCollisions {
    /// Instantiates a new `SimpleCollisions` object.
    pub fn new() -> SimpleCollisions {
        SimpleCollisions {
            registry: HashMap::new(),
            static_registry: HashMap::new(),
            proximities: Vec::new(),
            counter: 0,
        }
    }

    fn generate_uid(&mut self) -> usize {
        self.counter = self.counter + 1;
        self.counter
    }
}

impl Collisions for SimpleCollisions {
    type Identifier = usize;

    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> Self::Identifier {
        let new_uid = self.generate_uid();
        let new_body = Body::new_with_id(new_uid, Box::new(shape), Box::new(material), state);

        for &uid in self.registry.keys() {
            self.proximities.push(Proximity::new(uid, new_uid));
        }

        self.registry.insert(new_uid, new_body);
        return new_uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, position: Vector, rotation: Quaternion) -> Self::Identifier {
        let new_uid = self.generate_uid();
        let new_static_body = StaticBody::new_with_id(new_uid, Box::new(shape), Box::new(material), position, rotation);

        for &uid in self.registry.keys() {
            self.proximities.push(Proximity::new(uid, new_uid));
        }

        self.static_registry.insert(new_uid, new_static_body);
        return new_uid;
    }

    fn find_body(&self, uid: Self::Identifier) -> Option<&Body<Self::Identifier>> {
        self.registry.get(&uid)
    }

    fn find_body_mut(&mut self, uid: Self::Identifier) -> Option<&mut Body<Self::Identifier>> {
        self.registry.get_mut(&uid)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<Self::Identifier>> + 'a> {
        Box::new(self.registry.values())
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<Self::Identifier>> + 'a> {
        Box::new(self.registry.iter_mut().map(|(_, body)| body))
    }

    fn find_contacts(&self) -> Vec<Contact<Self::Identifier>> {
        let mut contacts = Vec::new();

        for proximity in self.proximities.iter() {
            let body_0 = self.find_body(proximity.handles[0]).unwrap();
            let body_1 = self.find_body(proximity.handles[1]).unwrap();

            match proximity.find_intersection(body_0, body_1) {
                Some((contact_center, contact_normal)) => {
                    contacts.push(
                        Contact {
                            ids: ContactPair::RigidRigid(body_0.id(), body_1.id()),
                            center: contact_center,
                            normal: contact_normal,
                        }
                    );
                }

                None => { /* do nothing */ }
            }
        }

        return contacts;
    }
}
