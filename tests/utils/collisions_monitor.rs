extern crate mithril;

use mithril::core::{ Body, Handle, State, UID };
use mithril::shapes::Shape;
use mithril::materials::Material;
use mithril::collisions::{ Contact, Collisions };

fn verbose_format_body<H: Handle>(body: &Body<H>) -> String {
    format!("{}, Shape={}", body, body.shape())
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
        println!("[Collisions create_body] {}", verbose_format_body(body));
        return uid;
    }

    fn find_body(&self, uid: UID) -> Option<&Body<UID>> {
        self.0.find_body(uid)
    }

    fn find_body_mut(&mut self, uid: UID) -> Option<&mut Body<UID>> {
        self.0.find_body_mut(uid)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<UID>> + 'a>{
        self.0.bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<UID>> + 'a>{
        self.0.bodies_iter_mut()
    }

    fn find_contacts(&self) -> Vec<Contact<UID>> {
        self.0.find_contacts()
    }
}
