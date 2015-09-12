//! The `dynamics` subsystem is responsible for the dynamic behaviour of the
//! engine. It contains subcomponents to handle time updates and collision
//! resolution.

mod simple_dynamics;
mod integrators;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod integrator;

use core::Float;
use maths::Vector;
use collisions::CollisionSpace;

pub use self::integrator::Integrator;
pub use self::integrators::semi_implicit_euler::SemiImplicitEuler;
pub use self::simple_dynamics::SimpleDynamics;

/// The `Dynamics` component is responsible for handling the dynamic component
/// of the simulation, including stepping the simulation forward in time and
/// managing environmental effects on bodies.
pub trait Dynamics {
    /// Steps the simulation forward in time by the specified amount.
    fn update<C: CollisionSpace>(&mut self, &mut C, Float);

    /// Returns the global gravitational force acting on the `RigidBody`
    /// objects.
    fn gravity(&self) -> Vector;

    /// Adjusts the global gravitational force acting on the `RigidBody`
    /// objects.
    fn set_gravity(&mut self, Vector);
}
