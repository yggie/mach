use std::collections::HashMap;

use core::{ Body, State, UID };
use shapes::Shape;
use materials::Material;
use collisions::{ Collisions, Contact, Proximity };

/// A simple implementation for representing space in the simulation.
pub struct SimpleCollisions {
    registry: HashMap<UID, Body<UID>>,
    proximities: Vec<Proximity<UID>>,
    counter: UID,
}

impl SimpleCollisions {
    /// Instantiates a new `SimpleCollisions` object.
    pub fn new() -> SimpleCollisions {
        SimpleCollisions {
            registry: HashMap::new(),
            proximities: Vec::new(),
            counter: 0,
        }
    }

    fn generate_uid(&mut self) -> UID {
        self.counter = self.counter + 1;
        self.counter
    }
}

impl Collisions for SimpleCollisions {
    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        let new_uid = self.generate_uid();
        let new_body = Body::new_with_handle(new_uid, Box::new(shape), Box::new(material), state);

        for &uid in self.registry.keys() {
            self.proximities.push(Proximity::new(uid, new_uid));
        }

        self.registry.insert(new_uid, new_body);
        return new_uid;
    }

    fn find_body(&self, uid: UID) -> Option<&Body<UID>> {
        self.registry.get(&uid)
    }

    fn find_body_mut(&mut self, uid: UID) -> Option<&mut Body<UID>> {
        self.registry.get_mut(&uid)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<UID>> + 'a> {
        Box::new(self.registry.values())
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<UID>> + 'a> {
        Box::new(self.registry.iter_mut().map(|(_, body)| body))
    }

    // TODO is there a safe way of doing this?
    // fn get_bodies_mut(&mut self, uids: Vec<UID>) -> Vec<Option<&mut Body<UID>>> {
    //     return uids.iter_mut()
    //         .map(|uid| self.registry.get(uid))
    //         .collect();
    // }

    fn find_contacts(&self) -> Vec<Contact<UID>> {
        let mut contacts = Vec::new();

        for proximity in self.proximities.iter() {
            let body_0 = self.find_body(proximity.handles[0]).unwrap();
            let body_1 = self.find_body(proximity.handles[1]).unwrap();

            match proximity.find_intersection(body_0, body_1) {
                Some(contact) => { contacts.push(contact); }

                None => { /* do nothing */ }
            }
        }

        return contacts;
    }
}
