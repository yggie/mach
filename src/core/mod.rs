//! The `core` module contains the core implementation of the engine’s logic.

mod body;
mod static_body;
mod state;
mod transform;

use std::fmt;
use std::hash::Hash;

pub use self::body::Body;
pub use self::static_body::StaticBody;
pub use self::state::State;
pub use self::transform::Transform;

/// A trait that must be implemented for all types acting as a unique identifier
/// for a `Body`.
pub trait Handle: Copy + Hash + Eq + fmt::Display { }

impl Handle for usize { }
