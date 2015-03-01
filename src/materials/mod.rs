//! The `materials` module defines objects representing mechanical properties
//! of the physical entities. This module is currently experimental, and may
//! undergo a complete refactoring in the near future.

pub use self::rigid::Rigid;

use math::Matrix;
use shapes::Shape;

/// Defines the traits for all mechanical property descriptions.
pub trait Material: 'static {
    /// Computes the mass using the volume of the provided shape.
    fn mass_of(&self, &Shape) -> f32;

    /// Computes the inertia tensor for the shape provided.
    fn inertia_for(&self, &Shape) -> Matrix;

    /// Computes the density when applied to a shape.
    fn density_of(&self, &Shape) -> f32;
}

mod rigid;
