#[cfg(test)]
#[path="../tests/worlds/mach_world_test.rs"]
mod tests;

use {CustomWorld, Scalar, World};
use maths::Vec3D;
use utils::Ref;
use dynamics::{DynamicBody, DynamicBodyHandle, DynamicBodyType, FixedBodyDef, RigidBodyDef};
use dynamics::solvers::MachConstraintSolver;
use dynamics::integrators::SemiImplicitEuler;
use collisions::Contact;
use collisions::detection::GJKEPADetection;
use collisions::broadphase::BruteForceBroadphase;
use collisions::narrowphase::NullNarrowphase;

pub struct MachWorld<T>(CustomWorld<BruteForceBroadphase<NullNarrowphase, DynamicBodyType<T>>, MachConstraintSolver, GJKEPADetection, SemiImplicitEuler, NullNarrowphase, T>) where T: 'static;

impl<T> MachWorld<T> {
    pub fn new() -> MachWorld<T> {
        let world = CustomWorld::new(
            GJKEPADetection::new(),
            SemiImplicitEuler::new(),
            BruteForceBroadphase::new(),
            MachConstraintSolver::new(),
            Vec3D::zero(),
        );

        MachWorld(world)
    }

    pub fn update(&mut self, time_step: Scalar) -> Vec<Contact<NullNarrowphase, DynamicBodyType<T>>> {
        self.0.update(time_step)
    }
}

impl<T> World<NullNarrowphase, T> for MachWorld<T> {
    fn update(&mut self, time_step: Scalar) -> Vec<Contact<NullNarrowphase, DynamicBodyType<T>>> {
        self.0.update(time_step)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<DynamicBody<NullNarrowphase, T>>> + 'a> {
        self.0.bodies_iter()
    }

    fn set_gravity(&mut self, gravity: Vec3D) {
        self.0.set_gravity(gravity)
    }

    fn create_rigid_body(&mut self, def: RigidBodyDef, extra: T) -> DynamicBodyHandle<NullNarrowphase, T> {
        self.0.create_rigid_body(def, extra)
    }

    fn create_fixed_body(&mut self, def: FixedBodyDef, extra: T) -> DynamicBodyHandle<NullNarrowphase, T> {
        self.0.create_fixed_body(def, extra)
    }
}
