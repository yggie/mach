#[cfg(test)]
#[path="../tests/worlds/mach_world_test.rs"]
mod tests;

use std::cell::{Ref, RefMut};

use {broadphase, narrowphase, CustomWorld, ID, Scalar, World};
use maths::Vect;
use maths::integrators::SemiImplicitEuler;
use solvers::MachConstraintSolver;
use entities::{Body, BodyHandle, BodyParams, EntityStore, MachStore, RigidBody};
use detection::{ContactEvent, GjkEpaDetection};

pub struct MachWorld(CustomWorld<broadphase::BruteForce<MachStore>, narrowphase::BruteForce, GjkEpaDetection, MachStore, SemiImplicitEuler, MachConstraintSolver>);

impl MachWorld {
    pub fn new() -> MachWorld {
        MachWorld(CustomWorld {
            broadphase: broadphase::BruteForce::new(),
            narrowphase: narrowphase::BruteForce::new(),
            detection: GjkEpaDetection::new(),
            entity_store: MachStore::new(),
            integrator: SemiImplicitEuler::new(),
            constraint_solver: MachConstraintSolver::new(),
            gravity: Vect::zero(),
        })
    }
}

impl World for MachWorld {
    fn update(&mut self, time_step: Scalar) -> Vec<ContactEvent> {
        self.0.update(time_step)
    }

    fn set_gravity(&mut self, gravity: Vect) {
        self.0.set_gravity(gravity)
    }
}

impl EntityStore for MachWorld {
    fn create_rigid_body(&mut self, params: &BodyParams) -> ID {
        self.0.create_rigid_body(params)
    }

    fn create_static_body(&mut self, params: &BodyParams) -> ID {
        self.0.create_static_body(params)
    }

    fn find_body(&self, id: ID) -> Option<Ref<Box<Body>>> {
        self.0.find_body(id)
    }

    fn find_rigid_body(&self, id: ID) -> Option<Ref<Box<RigidBody>>> {
        self.0.find_rigid_body(id)
    }

    fn find_body_handle(&self, id: ID) -> Option<&BodyHandle> {
        self.0.find_body_handle(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a> {
        self.0.bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a> {
        self.0.bodies_iter_mut()
    }

    fn rigid_body_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<RigidBody>>> + 'a> {
        self.0.rigid_body_iter_mut()
    }
}
