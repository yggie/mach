extern crate mach;

use mach::core::{ Body, UID, State, StaticBody, Transform };
use mach::shapes::Shape;
use mach::materials::Material;
use mach::collisions::{ Contact, Collisions };

fn verbose_format_body(body: &Body) -> String {
    format!("{}, Shape={}", body, body.shape())
}

fn verbose_format_static_body(static_body: &StaticBody) -> String {
    format!("{}, Shape={}", static_body, static_body.shape())
}

/// A utility class which wraps around a `Collisions` component and produces
/// parseable output for debugging.
pub struct CollisionsMonitor<C: Collisions>(C);

impl<C: Collisions> CollisionsMonitor<C> {
    /// Returns a new `CollisionsMonitor` wrapped around a `Collisions` instance.
    pub fn new(collisions: C) -> CollisionsMonitor<C> {
        CollisionsMonitor(collisions)
    }
}

impl<C: Collisions> Collisions for CollisionsMonitor<C> {
    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        let uid = self.0.create_body(shape, material, state);
        let body = self.0.find_body(uid).unwrap();
        println!("[CREATE] {}", verbose_format_body(body));
        return uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, transform: Transform) -> UID {
        let uid = self.0.create_static_body(shape, material, transform);
        let static_body = self.0.find_static_body(uid).unwrap();
        println!("[CREATE] {}", verbose_format_static_body(static_body));
        return uid;
    }

    fn find_body(&self, uid: UID) -> Option<&Body> {
        self.0.find_body(uid)
    }

    fn find_static_body(&self, uid: UID) -> Option<&StaticBody> {
        self.0.find_static_body(uid)
    }

    fn find_body_mut(&mut self, uid: UID) -> Option<&mut Body> {
        self.0.find_body_mut(uid)
    }

    fn find_static_body_mut(&mut self, uid: UID) -> Option<&mut StaticBody> {
        self.0.find_static_body_mut(uid)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body> + 'a>{
        self.0.bodies_iter()
    }

    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&StaticBody> + 'a>{
        self.0.static_bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body> + 'a>{
        self.0.bodies_iter_mut()
    }

    fn find_contacts(&self) -> Option<Vec<Contact>> {
        self.0.find_contacts()
    }
}
