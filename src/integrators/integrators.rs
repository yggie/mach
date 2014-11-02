//! Contains implementations of algorithms used to compute the next state of a
//! `Body` given the current state.

use core::Body;

pub use self::eulerintegration::euler_integration;

/// Computes the next state of the `Body` given the current state.
pub type StateIntegrator = fn(&mut Body, f32);

#[macro_escape]
#[cfg(test)]
#[path="../../tests/behaviours/state_integrator_behaviour.rs"]
mod behaviours;

mod eulerintegration;
