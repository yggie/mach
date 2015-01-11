//! The `dynamics` subsystem is responsible for the dynamic behaviour of the
//! engine. It contains subcomponents to handle time updates and collision
//! resolution.

#![unstable]

use space::Space;

pub use self::simple_dynamics::SimpleDynamics;

/// The `Dynamics` component is responsible for handling the dynamic component
/// of the simulation, including stepping the simulation forward in time and
/// managing environmental effects on bodies.
pub trait Dynamics {
    /// Steps the simulation forward in time by the specified amount.
    fn update<S: Space>(&mut self, &mut S, f32);
}

#[macro_use]
#[cfg(test)]
#[path="../../tests/behaviours/dynamics_behaviour.rs"]
mod behaviours;

#[experimental]
mod simple_dynamics;
