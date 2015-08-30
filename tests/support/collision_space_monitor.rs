extern crate mach;

use std::cell::{ Ref, RefMut };

use mach::core::{ RigidBody, UID, State, StaticBody, Transform, VolumetricBody };
use mach::utils::debug::renderevent;
use mach::shapes::Shape;
use mach::materials::Material;
use mach::collisions::{ CollisionSpace, Contact };
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
    fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        let uid = self.0.create_body(shape, material, state);
        let body = self.0.find_body(uid).unwrap();
        renderevent::create_rigid_body(&*body);
        return uid;
    }

    fn create_static_body<S: Shape, M: Material>(&mut self, shape: S, material: M, transform: Transform) -> UID {
        let uid = self.0.create_static_body(shape, material, transform);
        let static_body = self.0.find_static_body(uid).unwrap();
        renderevent::create_static_body(&*static_body);
        return uid;
    }

    fn find_body(&self, uid: UID) -> Option<Ref<RigidBody>> {
        self.0.find_body(uid)
    }

    fn find_static_body(&self, uid: UID) -> Option<Ref<StaticBody>> {
        self.0.find_static_body(uid)
    }

    fn find_body_mut(&mut self, uid: UID) -> Option<RefMut<RigidBody>> {
        self.0.find_body_mut(uid)
    }

    fn find_static_body_mut(&mut self, uid: UID) -> Option<RefMut<StaticBody>> {
        self.0.find_static_body_mut(uid)
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
