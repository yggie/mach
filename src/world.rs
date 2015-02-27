use core::{ UID, State };
use math::Vector;
use shapes::Shape;
use dynamics::Dynamics;
use materials::Material;
use collisions::Collisions;

/// A `World` is a physical world in mithril, it contains physical bodies and a
/// set of rules dictating how the bodies interact with the environment.
pub struct World<C: Collisions, D: Dynamics> {
    collisions: C,
    dynamics: D,
}

impl<C: Collisions, D: Dynamics> World<C, D> {
    /// Creates a new instance of a `World` with the given `Collisions` and
    /// `Dynamics` components.
    pub fn new(collisions: C, dynamics: D) -> World<C, D> {
        World{
            collisions: collisions,
            dynamics: dynamics,
        }
    }

    /// Creates an instance of a `Body` from the given components, returns a
    /// handle which can later be used to retrieve the `Body`.
    #[inline(always)]
    pub fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        self.collisions.create_body(shape, material, state)
    }

    /// Steps the `World` forward in time by the specified amount.
    #[inline(always)]
    pub fn update(&mut self, time_step: f32) {
        self.dynamics.update(&mut self.collisions, time_step);
    }

    /// Changes the global gravitational force acting on `Body` objects.
    #[inline(always)]
    pub fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity);
    }

    /// Returns an immutable reference to the `Collisions` object associated
    /// with the instance.
    #[inline(always)]
    pub fn collisions_component(&self) -> &C {
        &self.collisions
    }

    /// Returns a mutable reference to the `Collisions` object associated with
    /// the instance.
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
