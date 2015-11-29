extern crate mach;

use std::cell::{Ref, RefMut};

use mach::{EntityDesc, ID};
use mach::utils::debug::renderevent;
use mach::entities::{RigidBody, StaticBody, VolumetricBody};
use mach::collisions::{CollisionSpace, Contact};
use mach::collisions::narrowphase::Intersection;

/// A utility class which wraps around a `CollisionSpace` component and produces
/// parseable output for debugging.
pub struct CollisionSpaceMonitor<C: CollisionSpace>(C);

impl<C: CollisionSpace> CollisionSpaceMonitor<C> {
    /// Returns a new `CollisionSpaceMonitor` wrapped around a `CollisionSpace`
    /// instance.
    pub fn new(collisions: C) -> CollisionSpaceMonitor<C> {
        CollisionSpaceMonitor(collisions)
    }
}

impl<C: CollisionSpace> CollisionSpace for CollisionSpaceMonitor<C> {
    fn create_body(&mut self, entity_desc: &EntityDesc) -> ID {
        let id = self.0.create_body(entity_desc);
        let body = self.0.find_body(id).unwrap();
        renderevent::create_rigid_body(&*body);
        return id;
    }

    fn create_static_body(&mut self, entity_desc: &EntityDesc) -> ID {
        let id = self.0.create_static_body(entity_desc);
        let static_body = self.0.find_static_body(id).unwrap();
        renderevent::create_static_body(&*static_body);
        return id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.0.find_body(id)
    }

    fn find_static_body(&self, id: ID) -> Option<Ref<StaticBody>> {
        self.0.find_static_body(id)
    }

    fn find_body_mut(&mut self, id: ID) -> Option<RefMut<RigidBody>> {
        self.0.find_body_mut(id)
    }

    fn find_static_body_mut(&mut self, id: ID) -> Option<RefMut<StaticBody>> {
        self.0.find_static_body_mut(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a>{
        self.0.bodies_iter()
    }

    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<StaticBody>> + 'a>{
        self.0.static_bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<RigidBody>> + 'a>{
        self.0.bodies_iter_mut()
    }

    fn find_intersection(&self, body_0: &VolumetricBody, body_1: &VolumetricBody) -> Option<Intersection> {
        self.0.find_intersection(body_0, body_1)
    }

    fn find_contacts(&self) -> Option<Vec<Contact>> {
        self.0.find_contacts()
    }
}
