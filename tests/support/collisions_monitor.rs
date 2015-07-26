extern crate mithril;

use mithril::core::{ Body, Handle, State, StaticBody, Transform };
use mithril::shapes::Shape;
use mithril::materials::Material;
use mithril::collisions::{ Contact, Collisions };

fn verbose_format_body<H: Handle>(body: &Body<H>) -> String {
    format!("{}, Shape={}", body, body.shape())
}

fn verbose_format_static_body<H: Handle>(static_body: &StaticBody<H>) -> String {
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
    type Identifier = C::Identifier;

    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> Self::Identifier {
        let uid = self.0.create_body(shape, material, state);
        let body = self.0.find_body(uid).unwrap();
        println!("[Collisions create_body] {}", verbose_format_body(body));
        return uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, transform: Transform) -> Self::Identifier {
        let uid = self.0.create_static_body(shape, material, transform);
        let static_body = self.0.find_static_body(uid).unwrap();
        println!("[Collisions create_static_body] {}", verbose_format_static_body(static_body));
        return uid;
    }

    fn find_body(&self, uid: Self::Identifier) -> Option<&Body<Self::Identifier>> {
        self.0.find_body(uid)
    }

    fn find_static_body(&self, uid: Self::Identifier) -> Option<&StaticBody<Self::Identifier>> {
        self.0.find_static_body(uid)
    }

    fn find_body_mut(&mut self, uid: Self::Identifier) -> Option<&mut Body<Self::Identifier>> {
        self.0.find_body_mut(uid)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<Self::Identifier>> + 'a>{
        self.0.bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<Self::Identifier>> + 'a>{
        self.0.bodies_iter_mut()
    }

    fn find_contacts(&self) -> Vec<Contact<Self::Identifier>> {
        self.0.find_contacts()
    }
}
