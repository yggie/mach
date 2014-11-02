//! Contains implementations of algorithms used to compute the next `State` of
//! a `Body` given the current `State`.

use core::Body;

pub use self::eulerintegration::euler_integration;

/// Steps each `Body` in the `Database` forward in time by one tick.
pub type TimeMarcher = fn(&mut Body, f32);

#[macro_escape]
#[cfg(test)]
#[path="../../tests/behaviour/time_integrator_behaviour.rs"]
mod behaviours;

mod eulerintegration;
