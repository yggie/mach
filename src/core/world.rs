use core::{ UID, State };
use space::Space;
use shapes::Shape;
use dynamics::Dynamics;
use materials::Material;

/// A `World` is a physical world in mithril, it contains physical bodies and a
/// set of rules dictating how the bodies interact with the environment.
pub struct World<S: Space, D: Dynamics> {
    space: S,
    dynamics: D,
}

impl<S: Space, D: Dynamics> World<S, D> {
    /// Creates a new instance of a `World` with the given `Space` and `Time`
    /// components.
    pub fn new(space: S, dynamics: D) -> World<S, D> {
        World{
            space: space,
            dynamics: dynamics,
        }
    }

    /// Creates an instance of a `Body` from the given components, returns a
    /// handle which can be used to retrieve the `Body` from the `Space`.
    #[inline(always)]
    pub fn create_body<S: Shape, M: Material>(&mut self, shape: S, material: M, state: State) -> UID {
        self.space.create_body(shape, material, state)
    }

    /// Steps the `World` forward in time by the specified amount.
    #[inline(always)]
    pub fn update(&mut self, time_step: f32) {
        self.dynamics.update(&mut self.space, time_step);
    }

    /// Returns an immutable reference to the `Shape` object associated with the
    /// instance.
    #[inline(always)]
    pub fn space(&self) -> &S {
        &self.space
    }

    /// Returns a mutable reference to the `Shape` object associated with the
    /// instance.
    #[inline(always)]
    pub fn space_mut(&mut self) -> &mut S {
        &mut self.space
    }

    /// Returns an immutable reference to the `Dynamics` object associated with
    /// the instance.
    #[inline(always)]
    pub fn dynamics(&self) -> &D {
        &self.dynamics
    }

    /// Returns a mutable reference to the `Dynamics` object associated with the
    /// instance.
    #[inline(always)]
    pub fn dynamics_mut(&mut self) -> &mut D {
        &mut self.dynamics
    }
}
