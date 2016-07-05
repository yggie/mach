//! The `utils` module contains various helper functions.

#[macro_use]
mod chain_method;
#[macro_use]
mod implement_op_overload_variants;

mod surface;
mod data_handle;
mod unit_vec_3d_generator;
mod compute_surfaces_for_convex_hull;

pub use self::surface::Surface;
pub use self::data_handle::{DataHandle, Ref, RefMut};
pub use self::unit_vec_3d_generator::UnitVec3DGenerator;
pub use self::compute_surfaces_for_convex_hull::compute_surfaces_for_convex_hull;
