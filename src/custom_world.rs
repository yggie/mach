use std::cell::Ref;

use { ID, Scalar, World };
use maths::{ State, Transform, Vector };
use shapes::Shape;
use entities::{ Material, RigidBody };
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
    /// Creates an instance of a `RigidBody` from the given components, returns
    /// a handle which can later be used to retrieve the `RigidBody`.
    #[inline(always)]
    fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID {
        self.collision_space.create_body(shape, material, state)
    }

    /// Creates an instance of a `StaticBody` from the given components and
    /// returns an identifier which can be later used to retrieve the
    /// `StaticBody`.
    fn create_static_body<S: Shape>(&mut self, shape: S, material: &Material, transform: Transform) -> ID {
        self.collision_space.create_static_body(shape, material, transform)
    }

    /// Searches the world for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if
    /// found.
    #[inline(always)]
    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self.collision_space.find_body(id)
    }

    /// Returns an iterator over unique `RigidBody` instances in the `World`.
    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.collision_space.bodies_iter()
    }

    /// Steps the `World` forward in time by the specified amount.
    #[inline(always)]
    fn update(&mut self, time_step: Scalar) {
        self.dynamics.update(&mut self.collision_space, time_step);
    }

    /// Returns the value of the global gravity `Vector` set in the `World`.
    #[inline(always)]
    fn gravity(&self) -> Vector {
        self.dynamics.gravity()
    }

    /// Changes the global gravitational force acting on `RigidBody` objects.
    #[inline(always)]
    fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity);
    }
}
