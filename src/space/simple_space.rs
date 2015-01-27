use std::num::Float;
use std::slice::{ Iter, IterMut };

use shapes::Shape;
use materials::Material;
use space::{ Contact, Space };
use core::{ Body, State, UID };

#[cfg(test)]
#[path="../../tests/space/simple_space_test.rs"]
mod tests;

/// A simple implementation for representing space in the simulation.
pub struct SimpleSpace {
    bodies: Vec<Body>,
}

impl SimpleSpace {
    /// Instantiates a new `SimpleSpace` object.
    pub fn new() -> SimpleSpace {
        SimpleSpace{ bodies: Vec::new() }
    }

    fn compute_contact_between_bodies(&self, index_0: usize, index_1: usize) -> Option<Contact> {
        let bodies = [&self.bodies[index_0], &self.bodies[index_1]];
        let shapes = [bodies[0].shape(), bodies[1].shape()];
        let states = [bodies[0].state(), bodies[1].state()];
        let tolerance = shapes[0].surface_radius() + shapes[1].surface_radius();

        let rel_pos = states[1].position() - states[0].position();
        let dist_sq = rel_pos.length_sq();

        if dist_sq > tolerance*tolerance {
            return None;
        }

        let contact_normal = rel_pos.normalize();
        let contact_point = contact_normal * (dist_sq.sqrt() / 2.0);
        return Some(Contact {
            body_ids: [bodies[0].id(), bodies[1].id()],
            point: contact_point,
            normal: contact_normal,
        });
    }
}

impl Space for SimpleSpace {
    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        let uid = self.bodies.len();
        let body = Body::new_with_id(uid, Box::new(shape), Box::new(material), state);
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

    fn bodies(&self) -> Iter<Body> {
        self.bodies.iter()
    }

    fn bodies_mut(&mut self) -> IterMut<Body> {
        self.bodies.iter_mut()
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
        let length = self.bodies.len();

        for i in range(0us, length) {
            for j in range(i + 1, length) {
                match self.compute_contact_between_bodies(i, j) {
                    Some(contact) => {
                        contacts.push(contact);
                    }

                    None => { }
                }
            }
        }

        return contacts;
    }
}
