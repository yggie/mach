use std::cell::Ref;

use core::{ Float, UID };
use maths::{ State, Transform, Vector };
use shapes::Shape;
use entities::{ Material, RigidBody };
use dynamics::Dynamics;
use collisions::CollisionSpace;

/// A `World` is a physical world in mach, it contains physical bodies and a set
/// of rules dictating how the bodies interact with the environment.
pub struct World<C: CollisionSpace, D: Dynamics> {
    collisions: C,
    dynamics: D,
}

impl<C: CollisionSpace, D: Dynamics> World<C, D> {
    /// Creates a new instance of a `World` with the given `CollisionSpace` and
    /// `Dynamics` components.
    pub fn new(collisions: C, dynamics: D) -> World<C, D> {
        World{
            collisions: collisions,
            dynamics: dynamics,
        }
    }

    /// Creates an instance of a `RigidBody` from the given components, returns
    /// a handle which can later be used to retrieve the `RigidBody`.
    #[inline(always)]
    pub fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> UID {
        self.collisions.create_body(shape, material, state)
    }

    /// Creates an instance of a `StaticBody` from the given components and
    /// returns an identifier which can be later used to retrieve the
    /// `StaticBody`.
    pub fn create_static_body<S: Shape>(&mut self, shape: S, material: &Material, transform: Transform) -> UID {
        self.collisions.create_static_body(shape, material, transform)
    }

    /// Searches the world for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if
    /// found.
    #[inline(always)]
    pub fn find_body(&mut self, uid: UID) -> Option<Ref<RigidBody>> {
        self.collisions.find_body(uid)
    }

    /// Returns an iterator over unique `RigidBody` instances in the `World`.
    #[inline(always)]
    pub fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self.collisions.bodies_iter()
    }

    /// Steps the `World` forward in time by the specified amount.
    #[inline(always)]
    pub fn update(&mut self, time_step: Float) {
        self.dynamics.update(&mut self.collisions, time_step);
    }

    /// Returns the value of the global gravity `Vector` set in the `World`.
    #[inline(always)]
    pub fn gravity(&self) -> Vector {
        self.dynamics.gravity()
    }

    /// Changes the global gravitational force acting on `RigidBody` objects.
    #[inline(always)]
    pub fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity);
    }

    /// Returns an immutable reference to the `CollisionSpace` object associated
    /// with the instance.
    #[inline(always)]
    pub fn collisions_component(&self) -> &C {
        &self.collisions
    }

    /// Returns a mutable reference to the `CollisionSpace` object associated
    /// with the instance.
    #[inline(always)]
    pub fn collisions_component_mut(&mut self) -> &mut C {
        &mut self.collisions
    }

    /// Returns an immutable reference to the `Dynamics` object associated with
    /// the instance.
    #[inline(always)]
    pub fn dynamics_component(&self) -> &D {
        &self.dynamics
    }

    /// Returns a mutable reference to the `Dynamics` object associated with the
    /// instance.
    #[inline(always)]
    pub fn dynamics_component_mut(&mut self) -> &mut D {
        &mut self.dynamics
    }
}
