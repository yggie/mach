///! The `geometry` module defines useful structs and functions for operating on
///! geometry primitives.

mod line;
mod plane;

pub use self::line::Line;
pub use self::plane::{Plane, PlaneLocation};
