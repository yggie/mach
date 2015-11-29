use std::cell::Ref;

use {EntityDesc, ID, Scalar, World};
use maths::Vector;
use entities::RigidBody;
use dynamics::Dynamics;
use collisions::CollisionSpace;

/// A customizable implementation of a `World` object, which simply correctly
/// interfaces the individual `CollisionSpace` and `Dynamics` components in the
/// expected way.
pub struct CustomWorld<C: CollisionSpace, D: Dynamics> {
    collision_space: C,
    dynamics: D,
}

impl<C, D> CustomWorld<C, D> where C: CollisionSpace, D: Dynamics {
    /// Creates a new `CustomWorld` object built from the provided components.
    /// Each component is assumed to have been configured independently before
    /// being injected.
    pub fn new(collision_space: C, dynamics: D) -> CustomWorld<C, D> {
        CustomWorld {
            collision_space: collision_space,
            dynamics: dynamics,
        }
    }
}

impl<C, D> World for CustomWorld<C, D> where C: CollisionSpace, D: Dynamics {
    #[inline(always)]
    fn create_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.collision_space.create_body(entity_desc)
    }

    fn create_static_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.collision_space.create_static_body(entity_desc)
    }

    #[inline(always)]
    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.collision_space.find_body(id)
    }

    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.collision_space.bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: Scalar) {
        self.dynamics.update(&mut self.collision_space, time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> Vector {
        self.dynamics.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity);
    }
}
