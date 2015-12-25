use std::cell::Ref;

use {EntityDesc, ID, Scalar, World};
use maths::Vect;
use entities::RigidBody;
use dynamics::Dynamics;
use detection::Space;

/// A customizable implementation of a `World` object, which simply correctly
/// interfaces the individual `Space` and `Dynamics` components in the
/// expected way.
pub struct CustomWorld<S: Space, D: Dynamics> {
    space: S,
    dynamics: D,
}

impl<S, D> CustomWorld<S, D> where S: Space, D: Dynamics {
    /// Creates a new `CustomWorld` object built from the provided components.
    /// Each component is assumed to have been configured independently before
    /// being injected.
    pub fn new(space: S, dynamics: D) -> CustomWorld<S, D> {
        CustomWorld {
            space: space,
            dynamics: dynamics,
        }
    }
}

impl<S, D> World for CustomWorld<S, D> where S: Space, D: Dynamics {
    #[inline(always)]
    fn create_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.space.create_body(entity_desc)
    }

    fn create_static_body(&mut self, entity_desc: &EntityDesc) -> ID {
        self.space.create_static_body(entity_desc)
    }

    #[inline(always)]
    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.space.find_body(id)
    }

    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.space.bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: Scalar) {
        self.dynamics.update(&mut self.space, time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> Vect {
        self.dynamics.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: Vect) {
        self.dynamics.set_gravity(gravity);
    }
}
