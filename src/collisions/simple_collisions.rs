use std::collections::HashMap;

use core::{ Body, Handle, State, StaticBody };
use math::{ Vector, Quaternion };
use shapes::Shape;
use materials::Material;
use collisions::{ Collisions, Contact, ContactPair };
use collisions::narrowphase::GjkEpaImplementation;

struct Detector<H: Handle> {
    handles: [H; 2],
    narrowphase: GjkEpaImplementation,
}

impl<H: Handle> Detector<H> {
    fn new(handle_0: H, handle_1: H) -> Detector<H> {
        Detector {
            handles: [handle_0, handle_1],
            narrowphase: GjkEpaImplementation,
        }
    }
}

/// A simple implementation for representing space in the simulation.
pub struct SimpleCollisions {
    registry: HashMap<usize, Body<usize>>,
    static_registry: HashMap<usize, StaticBody<usize>>,
    detectors: Vec<Detector<usize>>,
    counter: usize,
}

impl SimpleCollisions {
    /// Instantiates a new `SimpleCollisions` object.
    pub fn new() -> SimpleCollisions {
        SimpleCollisions {
            registry: HashMap::new(),
            static_registry: HashMap::new(),
            detectors: Vec::new(),
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
            self.detectors.push(Detector::new(uid, new_uid));
        }

        self.registry.insert(new_uid, new_body);
        return new_uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, position: Vector, rotation: Quaternion) -> Self::Identifier {
        let new_uid = self.generate_uid();
        let new_static_body = StaticBody::new_with_id(new_uid, Box::new(shape), Box::new(material), position, rotation);

        for &uid in self.registry.keys() {
            self.detectors.push(Detector::new(uid, new_uid));
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

        for detector in self.detectors.iter() {
            let body_0 = self.find_body(detector.handles[0]).unwrap();
            let body_1 = self.find_body(detector.handles[1]).unwrap();

            match detector.narrowphase.find_intersection(body_0, body_1) {
                Some(intersection) => {
                    contacts.push(
                        Contact {
                            ids: ContactPair::RigidRigid(body_0.id(), body_1.id()),
                            center: intersection.point(),
                            normal: intersection.normal(),
                        }
                    );
                }

                None => { /* do nothing */ }
            }
        }

        return contacts;
    }
}
