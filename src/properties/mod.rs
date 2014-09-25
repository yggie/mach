//! The properties module defines objects representing mechanical properties of
//! the physical entities.

pub use self::rigid::Rigid;

use shapes::Shape;

use std::fmt;

/// Defines the traits for all mechanical property descriptions.
pub trait Property: Clone + Eq + fmt::Show {
    /// Computes the mass when applied to a shape.
    fn mass_of(&self, &Shape) -> f32;
    /// Computes the density when applied to a shape.
    fn density_of(&self, &Shape) -> f32;
}

mod rigid;
