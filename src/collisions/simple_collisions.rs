use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };
use std::collections::HashMap;

use core::{ RigidBody, UID, State, StaticBody, Transform };
use shapes::Shape;
use materials::Material;
use collisions::{ Collisions, Constraint };
use collisions::narrowphase::GjkEpaImplementation;

struct Detector {
    ids: [UID; 2],
    narrowphase: GjkEpaImplementation,
}

impl Detector {
    fn new(uid_0: UID, uid_1: UID) -> Detector {
        Detector {
            ids: [uid_0, uid_1],
            narrowphase: GjkEpaImplementation,
        }
    }
}

/// A simple implementation for representing space in the simulation.
pub struct SimpleCollisions {
    registry: HashMap<UID, Rc<RefCell<RigidBody>>>,
    static_registry: HashMap<UID, Rc<RefCell<StaticBody>>>,
    detectors: Vec<Detector>,
    static_detectors: Vec<Detector>,
    counter: UID,
}

impl SimpleCollisions {
    /// Instantiates a new `SimpleCollisions` object.
    pub fn new() -> SimpleCollisions {
        SimpleCollisions {
            registry: HashMap::new(),
            static_registry: HashMap::new(),
            detectors: Vec::new(),
            static_detectors: Vec::new(),
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
        let new_body = RigidBody::new_with_id(new_uid, Box::new(shape), Box::new(material), state);

        for &uid in self.registry.keys() {
            self.detectors.push(Detector::new(uid, new_uid));
        }

        self.registry.insert(new_uid, Rc::new(RefCell::new(new_body)));
        return new_uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, transform: Transform) -> UID {
        let new_uid = self.generate_uid();
        let new_static_body = StaticBody::new_with_id(new_uid, Box::new(shape), Box::new(material), transform);

        for &uid in self.registry.keys() {
            self.static_detectors.push(Detector::new(uid, new_uid));
        }

        self.static_registry.insert(new_uid, Rc::new(RefCell::new(new_static_body)));
        return new_uid;
    }

    fn find_body(&self, uid: UID) -> Option<Ref<RigidBody>> {
        self.registry.get(&uid).map(|cell| cell.borrow())
    }

    fn find_static_body(&self, uid: UID) -> Option<Ref<StaticBody>> {
        self.static_registry.get(&uid).map(|cell| cell.borrow())
    }

    fn find_body_mut(&mut self, uid: UID) -> Option<RefMut<RigidBody>> {
        self.registry.get_mut(&uid).map(|cell| cell.borrow_mut())
    }

    fn find_static_body_mut(&mut self, uid: UID) -> Option<RefMut<StaticBody>> {
        self.static_registry.get_mut(&uid).map(|cell| cell.borrow_mut())
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

    fn find_constraints(&self) -> Option<Vec<Constraint>> {
        let mut constraints = Vec::new();

        for detector in self.detectors.iter() {
            let body_0 = &*self.find_body(detector.ids[0]).unwrap();
            let body_1 = &*self.find_body(detector.ids[1]).unwrap();

            if let Some(intersection) = detector.narrowphase.find_intersection(body_0, body_1) {
                constraints.push(
                    Constraint::RigidRigid {
                        uids: (body_0.id(), body_1.id()),
                        contact_center: intersection.point(),
                        contact_normal: intersection.normal(),
                    }
                );
            }
        }

        for detector in self.static_detectors.iter() {
            let rigid_body = &*self.find_body(detector.ids[0]).unwrap();
            let static_body = &*self.find_static_body(detector.ids[1]).unwrap();

            if let Some(intersection) = detector.narrowphase.find_intersection(rigid_body, static_body) {
                constraints.push(
                    Constraint::RigidStatic {
                        rigid_uid: rigid_body.id(),
                        static_uid: static_body.id(),
                        contact_center: intersection.point(),
                        contact_normal: intersection.normal(),
                    }
                );
            }
        }

        if constraints.len() > 0 {
            return Some(constraints);
        } else {
            return None;
        }
    }
}
