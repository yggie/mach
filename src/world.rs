use std::cell::Ref;

use { ID, Float };
use maths::{ State, Transform, Vector };
use shapes::Shape;
use entities::{ Material, RigidBody };
use dynamics::Dynamics;
use collisions::CollisionSpace;

/// A `World` is a physical world in mach, it contains physical bodies and a set
/// of rules dictating how the bodies interact with the environment.
pub struct World<C: CollisionSpace, D: Dynamics> {
    _collision_space: C,
    _dynamics: D,
}

impl<C: CollisionSpace, D: Dynamics> World<C, D> {
    /// Creates a new instance of a `World` with the given `CollisionSpace` and
    /// `Dynamics` components.
    pub fn new(collision_space: C, dynamics: D) -> World<C, D> {
        World{
            _collision_space: collision_space,
            _dynamics: dynamics,
        }
    }

    /// Creates an instance of a `RigidBody` from the given components, returns
    /// a handle which can later be used to retrieve the `RigidBody`.
    #[inline(always)]
    pub fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID {
        self._collision_space.create_body(shape, material, state)
    }

    /// Creates an instance of a `StaticBody` from the given components and
    /// returns an identifier which can be later used to retrieve the
    /// `StaticBody`.
    pub fn create_static_body<S: Shape>(&mut self, shape: S, material: &Material, transform: Transform) -> ID {
        self._collision_space.create_static_body(shape, material, transform)
    }

    /// Searches the world for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if
    /// found.
    #[inline(always)]
    pub fn find_body(&self, id: ID) -> Option<Ref<RigidBody>> {
        self._collision_space.find_body(id)
    }

    /// Returns an iterator over unique `RigidBody` instances in the `World`.
    #[inline(always)]
    pub fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a> {
        self._collision_space.bodies_iter()
    }

    /// Steps the `World` forward in time by the specified amount.
    #[inline(always)]
    pub fn update(&mut self, time_step: Float) {
        self._dynamics.update(&mut self._collision_space, time_step);
    }

    /// Returns the value of the global gravity `Vector` set in the `World`.
    #[inline(always)]
    pub fn gravity(&self) -> Vector {
        self._dynamics.gravity()
    }

    /// Changes the global gravitational force acting on `RigidBody` objects.
    #[inline(always)]
    pub fn set_gravity(&mut self, gravity: Vector) {
        self._dynamics.set_gravity(gravity);
    }

    /// Returns an immutable reference to the `CollisionSpace` object associated
    /// with the instance.
    #[inline(always)]
    pub fn collision_space(&self) -> &C {
        &self._collision_space
    }

    /// Returns a mutable reference to the `CollisionSpace` object associated
    /// with the instance.
    #[inline(always)]
    pub fn collision_spaces_mut(&mut self) -> &mut C {
        &mut self._collision_space
    }

    /// Returns an immutable reference to the `Dynamics` object associated with
    /// the instance.
    #[inline(always)]
    pub fn dynamics(&self) -> &D {
        &self._dynamics
    }

    /// Returns a mutable reference to the `Dynamics` object associated with the
    /// instance.
    #[inline(always)]
    pub fn dynamics_mut(&mut self) -> &mut D {
        &mut self._dynamics
    }
}
