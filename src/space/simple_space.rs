use shapes::Shape;
use materials::Material;
use space::{ Contact, Pair, Space };
use core::{ Body, State, UID };

#[cfg(test)]
#[path="../../tests/space/simple_space_test.rs"]
mod tests;

/// A simple implementation for representing space in the simulation.
pub struct SimpleSpace {
    bodies: Vec<Body>,
    pairs: Vec<Pair<UID>>,
}

impl SimpleSpace {
    /// Instantiates a new `SimpleSpace` object.
    pub fn new() -> SimpleSpace {
        SimpleSpace{ bodies: Vec::new(), pairs: Vec::new() }
    }
}

impl Space for SimpleSpace {
    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        let uid = self.bodies.len();
        let body = Body::new_with_id(uid, Box::new(shape), Box::new(material), state);

        for other_body in self.bodies.iter() {
            self.pairs.push(Pair::new(other_body.id(), body.id()));
        }

        self.bodies.push(body);
        return uid;
    }

    fn get_body(&self, id: UID) -> Option<&Body> {
        for body in self.bodies.iter() {
            if body.id() == id {
                return Some(body);
            }
        }

        return None;
    }

    fn get_body_mut(&mut self, id: UID) -> Option<&mut Body> {
        for body in self.bodies.iter_mut() {
            if body.id() == id {
                return Some(body);
            }
        }

        return None;
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body> + 'a> {
        Box::new(self.bodies.iter())
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body> + 'a> {
        Box::new(self.bodies.iter_mut())
    }

    fn get_bodies(&self, uids: Vec<UID>) -> Vec<Option<&Body>> {
        let mut options = Vec::with_capacity(uids.len());

        for body in self.bodies.iter() {
            let mut found = false;
            for uid in uids.iter() {
                if body.id() == *uid {
                    options.push(Some(body));
                    found = true;
                    break;
                }
            }

            if !found {
                options.push(None);
            }
        }

        return options;
    }

    fn get_bodies_mut(&mut self, uids: Vec<UID>) -> Vec<Option<&mut Body>> {
        let mut options = Vec::with_capacity(uids.len());

        for body in self.bodies.iter_mut() {
            let mut found = false;
            for uid in uids.iter() {
                if body.id() == *uid {
                    options.push(Some(body));
                    found = true;
                    break;
                }
            }

            if !found {
                options.push(None);
            }
        }

        return options;
    }

    fn find_contacts(&self) -> Vec<Contact> {
        let mut contacts = Vec::new();

        for pair in self.pairs.iter() {
            let bodies: Vec<&Body> = self.get_bodies(vec!(pair.handles[0], pair.handles[1])).iter().map(|b| b.unwrap()).collect();
            match pair.compute_contact(bodies[0], bodies[1]) {
                Some(contact) => { contacts.push(contact); }

                None => { /* do nothing */ }
            }
        }

        return contacts;
    }
}
