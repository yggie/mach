//! The `materials` module defines objects representing mechanical properties
//! of the physical entities. This module is currently experimental, and may
//! undergo a complete refactoring in the near future.

pub use self::rigid::Rigid;

use shapes::Shape;

/// Defines the traits for all mechanical property descriptions.
pub trait Material: 'static {
    /// Computes the mass when applied to a shape.
    fn mass_of(&self, &Shape) -> f32;
    /// Computes the density when applied to a shape.
    fn density_of(&self, &Shape) -> f32;
}

mod rigid;
