#[cfg(test)]
#[path="../tests/worlds/mach_world_test.rs"]
mod tests;

use {CustomWorld, Scalar, World};
use maths::Vec3D;
use utils::{Ref, Handle};
use dynamics::{DynamicBodyExtension, FixedBodyDef, RigidBodyDef};
use dynamics::solvers::MachConstraintSolver;
use dynamics::integrators::SemiImplicitEuler;
use collisions::{Body, CollisionBody, Contact};
use collisions::detection::GJKEPADetection;
use collisions::broadphase::BruteForceBroadphase;
use collisions::narrowphase::NullNarrowphase;

pub type MachBody<E> = Body<DynamicBodyExtension<E>, NullNarrowphase>;

pub struct MachWorld<E>(CustomWorld<BruteForceBroadphase<MachBody<E>>, MachConstraintSolver, GJKEPADetection, E, SemiImplicitEuler, MachBody<E>>) where E: 'static;

impl<E> MachWorld<E> {
    pub fn new() -> MachWorld<E> {
        let world = CustomWorld::new(
            GJKEPADetection::new(),
            SemiImplicitEuler::new(),
            BruteForceBroadphase::new(),
            MachConstraintSolver::new(),
            Vec3D::zero(),
        );

        MachWorld(world)
    }

    pub fn update(&mut self, time_step: Scalar) -> Vec<Contact<MachBody<E>>> {
        self.0.update(time_step)
    }
}

impl<E> World<MachBody<E>> for MachWorld<E> {
    fn update(&mut self, time_step: Scalar) -> Vec<Contact<MachBody<E>>> {
        self.0.update(time_step)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<MachBody<E>>> + 'a> {
        self.0.bodies_iter()
    }

    fn set_gravity(&mut self, gravity: Vec3D) {
        self.0.set_gravity(gravity)
    }

    fn create_rigid_body(&mut self, def: RigidBodyDef, extra: E) -> Handle<MachBody<E>> {
        self.0.create_rigid_body(def, extra)
    }

    fn create_fixed_body(&mut self, def: FixedBodyDef, extra: E) -> Handle<MachBody<E>> {
        self.0.create_fixed_body(def, extra)
    }
}
