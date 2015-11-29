use std::cell::Ref;

use {CustomWorld, EntityDesc, ID, Scalar, World};
use maths::{State, Vector};
use shapes::Shape;
use entities::{Material, RigidBody};
use dynamics::SimpleDynamics;
use collisions::SimpleCollisionSpace;

/// The default implementation of a `World` object, using all the engine’s core
/// trait implementations to function.
pub struct MachWorld(CustomWorld<SimpleCollisionSpace, SimpleDynamics>);

impl MachWorld {
    /// Creates a new `MachWorld` with the default configuration.
    pub fn new() -> MachWorld {
        MachWorld(CustomWorld::new(SimpleCollisionSpace::new(), SimpleDynamics::new()))
    }
}

impl World for MachWorld {
    #[inline(always)]
    fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID {
        self.0.create_body(shape, material, state)
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
    fn update(&mut self, time_step: Scalar) {
        self.0.update(time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> Vector {
        self.0.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: Vector) {
        self.0.set_gravity(gravity);
    }
}
