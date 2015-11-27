//! The `dynamics` subsystem is responsible for the dynamic behaviour of the
//! engine. It contains subcomponents to handle time updates and collision
//! resolution.

mod simple_dynamics;
mod integrators;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod dynamics;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod integrator;

pub use self::dynamics::Dynamics;
pub use self::integrator::Integrator;
pub use self::integrators::semi_implicit_euler::SemiImplicitEuler;
pub use self::simple_dynamics::SimpleDynamics;
