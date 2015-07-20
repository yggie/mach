//! The `dynamics` subsystem is responsible for the dynamic behaviour of the
//! engine. It contains subcomponents to handle time updates and collision
//! resolution.

use core::Handle;
use math::Vector;
use collisions::Collisions;

pub use self::simple_dynamics::SimpleDynamics;
pub use self::force_accumulator::ForceAccumulator;

/// The `Dynamics` component is responsible for handling the dynamic component
/// of the simulation, including stepping the simulation forward in time and
/// managing environmental effects on bodies.
pub trait Dynamics {
    /// The identifier used to dereference `Body` instances.
    type Identifier: Handle;

    /// Steps the simulation forward in time by the specified amount.
    fn update<C: Collisions<Identifier=Self::Identifier>>(&mut self, &mut C, f32);

    /// Returns the global gravitational force acting on the `Body` objects.
    fn gravity(&self) -> Vector;

    /// Adjusts the global gravitational force acting on the `Body` objects.
    fn set_gravity(&mut self, Vector);
}

mod force_accumulator;
mod simple_dynamics;
