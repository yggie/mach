use std::cell::Ref;

use {CustomWorld, EntityDesc, ID, Scalar, World};
use maths::Vect;
use entities::RigidBody;
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
    fn create_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.0.create_body(entity_desc)
    }

    fn create_static_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.0.create_static_body(entity_desc)
    }

    #[inline(always)]
    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.0.find_body(id)
    }

    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.0.bodies_iter()
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
