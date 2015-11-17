use std::cell::Ref;

use mach::{ CustomWorld, ID, Scalar, World };
use mach::maths::{ State, Transform, Vector };
use mach::shapes::Shape;
use mach::entities::{ Material, RigidBody };
use mach::dynamics::Dynamics;
use mach::collisions::CollisionSpace;

use support::{ CollisionSpaceMonitor, DynamicsMonitor };

pub struct MonitoredWorld<C: CollisionSpace, D: Dynamics>(CustomWorld<CollisionSpaceMonitor<C>, DynamicsMonitor<D>>);

impl<C, D> MonitoredWorld<C, D> where C: CollisionSpace, D: Dynamics {
    pub fn new(collision_space: C, dynamics: D) -> MonitoredWorld<C, D> {
        MonitoredWorld(CustomWorld::new(
            CollisionSpaceMonitor::new(collision_space),
            DynamicsMonitor::new(dynamics),
        ))
    }
}

impl<C, D> World for MonitoredWorld<C, D> where C: CollisionSpace, D: Dynamics {
    #[inline(always)]
    fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID {
        self.0.create_body(shape, material, state)
    }

    fn create_static_body<S: Shape>(&mut self, shape: S, material: &Material, transform: Transform) -> ID {
        self.0.create_static_body(shape, material, transform)
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
