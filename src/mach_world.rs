use std::cell::Ref;

use {CustomWorld, ID, Scalar, World};
use maths::Vect;
use entities::{BodyParams, RigidBody, StaticBody};
use dynamics::MachDynamics;
use detection::{Contact, MachSpace};

/// The default implementation of a `World` object, using all the engine’s core
/// trait implementations to function.
pub struct MachWorld(CustomWorld<MachSpace, MachDynamics>);

impl MachWorld {
    /// Creates a new `MachWorld` with the default configuration.
    pub fn new() -> MachWorld {
        MachWorld(CustomWorld::new(MachSpace::new(), MachDynamics::new()))
    }
}

impl World for MachWorld {
    #[inline(always)]
    fn create_rigid_body(&mut self, params: &BodyParams) -> ID {
        self.0.create_rigid_body(params)
    }

    fn create_static_body(&mut self, params: &BodyParams) -> ID {
        self.0.create_static_body(params)
    }

    #[inline(always)]
    fn find_rigid_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.0.find_rigid_body(id)
    }

    #[inline(always)]
    fn rigid_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.0.rigid_bodies_iter()
    }

    #[inline(always)]
    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<StaticBody>> + 'a> {
        self.0.static_bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: Scalar) -> Option<Vec<Contact>> {
        return self.0.update(time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> Vect {
        self.0.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: Vect) {
        self.0.set_gravity(gravity);
    }
}
